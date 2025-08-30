"""
Advanced ML Database Manager - Powerful ML with Dragon's Labyrinth tables.

Target tables aligned with game goals:
- Biome (actual biome data for tiles)
- Dungeon (coordinator) -> Cave, Temple, Tomb  
- Monster
- Settlement (coordinator) -> City, Town, Village
- Inn (isolated healing places)
- Dwelling (coordinator) -> Farms & Cabins, Strongholds
- Factions (coordinator) -> Cults, Militias, Syndicates
"""

from __future__ import annotations

import json
import logging
import re
import sqlite3
from pathlib import Path
from typing import Any

import numpy as np
from sklearn.cluster import DBSCAN, KMeans
from sklearn.decomposition import LatentDirichletAllocation, TruncatedSVD
from sklearn.ensemble import IsolationForest, RandomForestClassifier
from sklearn.feature_extraction.text import CountVectorizer, TfidfVectorizer
from sklearn.manifold import TSNE
from sklearn.metrics.pairwise import cosine_similarity
from sklearn.neighbors import NearestNeighbors
from sklearn.preprocessing import StandardScaler
from sqlmodel import SQLModel, Session, create_engine, select

from generator.constants import GAME_DB_PATH
from generator.entities.json_entities import JSONEntityRecord
from generator.entities.html_entities import HTMLEntityRecord


class DragonLabyrinthMLProcessor:
    """
    Advanced ML processor with auto-discovery and refinement.
    
    Features:
    - Auto-discovery of regions from hex_tiles_full.csv
    - Multi-scale vectorization for content analysis
    - Clustering ensemble for pattern discovery
    - Second-pass refinement using discovered patterns
    - Entity memory for continuous learning
    - Relationship discovery using embeddings
    """
    
    # Target table hierarchy for Dragon's Labyrinth
    TARGET_TABLES = {
        "biome": None,  # Direct table
        "monster": None,  # Direct table
        "inn": None,  # Direct table
        "dungeon": ["cave", "temple", "tomb"],  # Coordinator
        "settlement": ["city", "town", "village"],  # Coordinator
        "dwelling": ["farms_cabins", "stronghold"],  # Coordinator
        "factions": ["cult", "militia", "syndicate"]  # Coordinator
    }
    
    def __init__(self):
        # Auto-discovered regions and patterns (dynamic)
        self.discovered_regions = []
        self.region_clusters = {}
        self.auto_patterns = {}
        
        # Load existing hex analysis for region discovery
        self._auto_discover_regions()
        # Multi-scale vectorizers for different content types
        self.short_vectorizer = TfidfVectorizer(
            max_features=1000,
            ngram_range=(1, 2),
            stop_words='english'
        )
        
        self.long_vectorizer = TfidfVectorizer(
            max_features=5000,
            ngram_range=(1, 3),
            stop_words='english',
            min_df=2,
            max_df=0.95
        )
        
        self.semantic_vectorizer = CountVectorizer(
            max_features=10000,
            ngram_range=(1, 4),
            stop_words='english'
        )
        
        # Advanced dimensionality reduction
        self.svd = TruncatedSVD(n_components=100, random_state=42)
        self.lda = LatentDirichletAllocation(n_components=20, random_state=42, max_iter=10)
        self.tsne = TSNE(n_components=2, random_state=42, perplexity=30)
        
        # Clustering ensemble
        self.kmeans = None  # Initialized per batch
        self.dbscan = DBSCAN(eps=0.3, min_samples=5)
        
        # Anomaly detection
        self.isolation_forest = IsolationForest(contamination=0.1, random_state=42)
        
        # Entity memory for continuous learning
        self.entity_memory = {
            "biomes": [], "monsters": [], "inns": [],
            "dungeons": {"cave": [], "temple": [], "tomb": []},
            "settlements": {"city": [], "town": [], "village": []},
            "dwellings": {"farms_cabins": [], "stronghold": []},
            "factions": {"cult": [], "militia": [], "syndicate": []}
        }
        
        # Compile comprehensive patterns
        self.patterns = self._compile_comprehensive_patterns()
        
        # Processing statistics
        self.stats = {
            "total_processed": 0, "successful_extractions": 0, "failed_extractions": 0,
            "pattern_matches": 0, "ml_classifications": 0, "anomalies_found": 0,
            "relationships_discovered": 0, "regions_discovered": len(self.discovered_regions),
            "auto_patterns_generated": 0
        }
    
    def _auto_discover_regions(self) -> None:
        """Auto-discover all region names from hex_tiles_full.csv using ML clustering."""
        
        try:
            import pandas as pd
            
            # Load hex tiles analysis
            csv_path = Path("hbf_analysis/hex_tiles_full.csv")
            if not csv_path.exists():
                self.discovered_regions = ["Fearless Wilds", "Vicious Crags", "Ragthorn Woods", "Heartseeker Forest"]
                return
            
            df = pd.read_csv(csv_path)
            
            # Extract all unique regions
            if "region" in df.columns:
                unique_regions = df["region"].dropna().unique().tolist()
                self.discovered_regions = [region.strip() for region in unique_regions if region.strip()]
            else:
                # Try to extract from other columns
                region_candidates = set()
                for col in df.columns:
                    if "region" in col.lower() or "area" in col.lower():
                        candidates = df[col].dropna().unique()
                        for candidate in candidates:
                            if isinstance(candidate, str) and len(candidate.split()) <= 3:
                                region_candidates.add(candidate.strip())
                
                self.discovered_regions = list(region_candidates)
            
            # If we found regions, do ML analysis to cluster them
            if self.discovered_regions:
                self._cluster_regions_by_similarity()
                self._generate_auto_patterns()
        
        except Exception as e:
            # Fallback to known regions
            self.discovered_regions = ["Fearless Wilds", "Vicious Crags", "Ragthorn Woods", "Heartseeker Forest"]
    
    def _cluster_regions_by_similarity(self) -> None:
        """Use ML to cluster regions by semantic similarity."""
        
        if len(self.discovered_regions) < 2:
            return
        
        try:
            # Vectorize region names for similarity analysis
            region_vectorizer = TfidfVectorizer(ngram_range=(1, 2), lowercase=True)
            region_vectors = region_vectorizer.fit_transform(self.discovered_regions)
            
            # Cluster regions
            n_clusters = min(6, max(2, len(self.discovered_regions) // 3))
            kmeans = KMeans(n_clusters=n_clusters, random_state=42, n_init=10)
            region_labels = kmeans.fit_predict(region_vectors.toarray())
            
            # Group regions by cluster
            self.region_clusters = {}
            for i, region in enumerate(self.discovered_regions):
                cluster_id = region_labels[i]
                if cluster_id not in self.region_clusters:
                    self.region_clusters[cluster_id] = []
                self.region_clusters[cluster_id].append(region)
            
            # Analyze cluster characteristics
            for cluster_id, regions in self.region_clusters.items():
                cluster_terms = []
                for region in regions:
                    cluster_terms.extend(region.lower().split())
                
                # Find common terms in cluster
                term_freq = {}
                for term in cluster_terms:
                    if len(term) > 3:  # Skip short words
                        term_freq[term] = term_freq.get(term, 0) + 1
                
                # Dominant terms for this cluster
                dominant_terms = sorted(term_freq.items(), key=lambda x: x[1], reverse=True)[:3]
                self.region_clusters[cluster_id] = {
                    "regions": regions,
                    "dominant_terms": [term for term, _ in dominant_terms],
                    "biome_hint": self._infer_biome_from_terms([term for term, _ in dominant_terms])
                }
        
        except Exception:
            # Keep discovered regions without clustering
            pass
    
    def _infer_biome_from_terms(self, terms: list[str]) -> str:
        """Infer biome type from region terms."""
        
        biome_keywords = {
            "forest": ["wood", "tree", "forest", "grove", "timber"],
            "mountain": ["crag", "peak", "mountain", "hill", "stone"],
            "swamp": ["bog", "marsh", "swamp", "mire", "wet"],
            "desert": ["sand", "dry", "desert", "dune", "arid"],
            "plains": ["plain", "field", "grass", "meadow", "prairie"],
            "jungle": ["wild", "jungle", "vine", "dense", "tropical"]
        }
        
        for biome, keywords in biome_keywords.items():
            if any(keyword in term for term in terms for keyword in keywords):
                return biome
        
        return "unknown"
    
    def _generate_auto_patterns(self) -> None:
        """Generate regex patterns based on discovered regions."""
        
        # Generate region detection patterns
        region_patterns = []
        for region in self.discovered_regions:
            # Create flexible pattern for each region
            escaped_region = re.escape(region)
            pattern = re.compile(escaped_region, re.IGNORECASE)
            region_patterns.append(pattern)
        
        # Update patterns with discovered regions
        self.auto_patterns["regions"] = region_patterns
        
        # Generate biome patterns based on cluster analysis
        biome_patterns = []
        for cluster_data in self.region_clusters.values():
            for term in cluster_data["dominant_terms"]:
                if len(term) > 3:
                    pattern = re.compile(rf"\b{re.escape(term)}\w*", re.IGNORECASE)
                    biome_patterns.append(pattern)
        
        self.auto_patterns["biomes_discovered"] = biome_patterns
        
        self.stats["auto_patterns_generated"] = len(self.auto_patterns.get("regions", [])) + len(self.auto_patterns.get("biomes_discovered", []))
    
    def _compile_comprehensive_patterns(self) -> dict[str, list[re.Pattern]]:
        """Compile comprehensive regex patterns for Dragon's Labyrinth tables."""
        
        return {
            "biome": [
                re.compile(r"(forest|mountain|desert|ocean|swamp|plains|tundra|jungle)", re.I),
                re.compile(r"(terrain|landscape|environment|biome|climate)", re.I),
                re.compile(r"Hex\s+[A-Z0-9]+\s+in\s+", re.I),  # HBF hex pattern
                re.compile(r"(weather|temperature|seasonal)", re.I)
            ],
            "monster": [
                re.compile(r"(creature|monster|beast|horror|abomination)", re.I),
                re.compile(r"CR:\s*\d+|Challenge Rating", re.I),
                re.compile(r"(hp|hit points?|AC|armor class)", re.I),
                re.compile(r"STR\s+DEX\s+CON\s+INT\s+WIS\s+CHA", re.I),
                re.compile(r"(tainted|corrupted|nightmare|void-touched)", re.I)  # Horror variants
            ],
            "cave": [
                re.compile(r"(cave|cavern|grotto|underground|tunnel|mine)", re.I),
                re.compile(r"(stalactite|stalagmite|echo|damp|darkness)", re.I),
                re.compile(r"(natural formation|excavated|mining)", re.I)
            ],
            "temple": [
                re.compile(r"(temple|shrine|sanctuary|altar|chapel)", re.I),
                re.compile(r"(holy|sacred|divine|blessed|consecrated)", re.I),
                re.compile(r"(priest|cleric|worship|prayer|ritual)", re.I),
                re.compile(r"(deity|god|goddess|divine entity)", re.I)
            ],
            "tomb": [
                re.compile(r"(tomb|crypt|mausoleum|burial|grave|barrow)", re.I),
                re.compile(r"(undead|skeleton|zombie|ghost|spirit|wraith)", re.I),
                re.compile(r"(sarcophagus|coffin|remains|ancient|curse)", re.I)
            ],
            "city": [
                re.compile(r"(city|metropolis|capital|urban center)", re.I),
                re.compile(r"(district|ward|quarter|plaza|avenue)", re.I),
                re.compile(r"(thousands|population|crowded|bustling)", re.I),
                re.compile(r"(walls|fortified|defended|garrison)", re.I)
            ],
            "town": [
                re.compile(r"(town|township|market town|trading post)", re.I),
                re.compile(r"(merchant|shop|guild|market square)", re.I),
                re.compile(r"(hundreds|moderate size|growing|prosperous)", re.I),
                re.compile(r"(trade route|commerce|goods)", re.I)
            ],
            "village": [
                re.compile(r"(village|hamlet|settlement|small community)", re.I),
                re.compile(r"(farm|cottage|rural|pastoral|agricultural)", re.I),
                re.compile(r"(dozens|small|quiet|peaceful|rustic)", re.I),
                re.compile(r"(elder|headman|village chief)", re.I)
            ],
            "inn": [
                re.compile(r"(inn|tavern|lodge|hostel)\s+(?!in\s+(?:the\s+)?(?:city|town|village))", re.I),
                re.compile(r"(isolated|remote|wilderness|crossroads|roadside)", re.I),
                re.compile(r"(rest|healing|refuge|shelter|waystation)", re.I),
                re.compile(r"(proprietor|innkeeper|host)", re.I)
            ],
            "farms_cabins": [
                re.compile(r"(farm|cabin|homestead|cottage|ranch)", re.I),
                re.compile(r"(field|crop|livestock|garden|orchard)", re.I),
                re.compile(r"(family|settler|frontier|rural dwelling)", re.I),
                re.compile(r"(harvest|grain|cattle|sheep|chicken)", re.I)
            ],
            "stronghold": [
                re.compile(r"(stronghold|fortress|keep|castle|citadel|fort)", re.I),
                re.compile(r"(fortified|defensive|garrison|military)", re.I),
                re.compile(r"(lord|noble|knight|commander|baron)", re.I),
                re.compile(r"(tower|wall|moat|battlements)", re.I)
            ],
            "cult": [
                re.compile(r"(cult|worship|dark ritual|sacrifice|summoning)", re.I),
                re.compile(r"(forbidden|secret|ancient|eldritch|void)", re.I),
                re.compile(r"(zealot|fanatic|devotee|acolyte|cultist)", re.I),
                re.compile(r"(dark entity|elder god|cosmic horror)", re.I)
            ],
            "militia": [
                re.compile(r"(militia|guard|watch|patrol|defense force)", re.I),
                re.compile(r"(defend|protect|serve|duty|law)", re.I),
                re.compile(r"(captain|sergeant|soldier|recruit|veteran)", re.I),
                re.compile(r"(Fists of Justice|Swords of Justice)", re.I)  # Known factions
            ],
            "syndicate": [
                re.compile(r"(syndicate|gang|cartel|crime organization)", re.I),
                re.compile(r"(smuggle|steal|underground|black market|racket)", re.I),
                re.compile(r"(boss|enforcer|lieutenant|operation|territory)", re.I),
                re.compile(r"(extortion|kidnapping|assassination|heist)", re.I)
            ]
        }
    
    def process_entity_batch(self, entities: list[tuple[str, str]]) -> dict[str, Any]:
        """
        Process batch of entities with full ML pipeline.
        
        Args:
            entities: List of (uuid, content) tuples
            
        Returns:
            Processing results with classifications and relationships
        """
        
        if not entities:
            return {}
        
        # Extract texts and UUIDs
        texts = [content for _, content in entities]
        uuids = [uuid for uuid, _ in entities]
        
        self.stats["total_processed"] += len(entities)
        
        # Multi-scale vectorization
        X_short = self.short_vectorizer.fit_transform(texts)
        X_long = self.long_vectorizer.fit_transform(texts) if len(texts) > 5 else X_short
        X_semantic = self.semantic_vectorizer.fit_transform(texts)
        
        # Advanced dimensionality reduction
        X_reduced = self.svd.fit_transform(X_long)
        
        # Topic modeling for content understanding
        try:
            topics = self.lda.fit_transform(X_long) if len(texts) > 10 else None
        except:
            topics = None
        
        # Clustering ensemble for pattern discovery
        n_clusters = min(15, max(3, len(texts) // 8))
        self.kmeans = KMeans(n_clusters=n_clusters, random_state=42, n_init=10)
        kmeans_labels = self.kmeans.fit_predict(X_reduced)
        
        try:
            dbscan_labels = self.dbscan.fit_predict(X_reduced)
        except:
            dbscan_labels = np.zeros(len(texts))
        
        # Anomaly detection
        try:
            anomalies = self.isolation_forest.fit_predict(X_reduced)
            self.stats["anomalies_found"] += int((anomalies == -1).sum())
        except:
            anomalies = np.zeros(len(texts))
        
        # Process each entity with ML context
        results = []
        for i, (uuid, content) in enumerate(entities):
            result = self._process_single_entity_with_context(
                uuid=uuid,
                content=content,
                short_vector=X_short[i],
                long_vector=X_long[i] if X_long is not X_short else X_short[i],
                semantic_vector=X_semantic[i],
                reduced_embedding=X_reduced[i],
                kmeans_cluster=kmeans_labels[i],
                dbscan_cluster=dbscan_labels[i],
                is_anomaly=(anomalies[i] == -1),
                topic_distribution=topics[i] if topics is not None else None
            )
            results.append(result)
            
            # Update entity memory for continuous learning
            self._update_entity_memory(result)
        
        # Discover relationships using embeddings
        relationships = self._discover_entity_relationships(results, X_reduced)
        self.stats["relationships_discovered"] += len(relationships)
        
        # Generate analysis CSVs for inspection
        csv_outputs = self._generate_analysis_csvs(results)
        
        return {
            "entities": results,
            "relationships": relationships,
            "cluster_analysis": self._analyze_clusters(kmeans_labels, dbscan_labels, texts),
            "anomaly_analysis": self._analyze_anomalies(anomalies, uuids, texts),
            "topic_analysis": self._analyze_topics(topics, texts) if topics is not None else {},
            "csv_outputs": csv_outputs,
            "processing_stats": self.stats.copy()
        }
    
    def _process_single_entity_with_context(
        self,
        uuid: str,
        content: str,
        short_vector: Any,
        long_vector: Any, 
        semantic_vector: Any,
        reduced_embedding: np.ndarray,
        kmeans_cluster: int,
        dbscan_cluster: int,
        is_anomaly: bool,
        topic_distribution: np.ndarray | None
    ) -> dict[str, Any]:
        """Process single entity with full ML context."""
        
        # Step 1: Pattern classification (fast, high confidence)
        pattern_scores = self._pattern_classification_scoring(content)
        self.stats["pattern_matches"] += int(any(score > 0 for score in pattern_scores.values()))
        
        # Step 2: ML feature extraction (comprehensive features)
        ml_features = self._extract_comprehensive_ml_features(content)
        
        # Step 3: Ensemble classification (patterns + features + similarity)
        target_table, subtype, confidence = self._ensemble_classification(
            content=content,
            pattern_scores=pattern_scores,
            ml_features=ml_features,
            embedding=reduced_embedding
        )
        self.stats["ml_classifications"] += 1
        
        # Step 4: Deep extraction based on determined type
        if confidence > 0.3:
            extracted_data = self._deep_extraction_by_type(content, target_table, subtype)
            self.stats["successful_extractions"] += 1
        else:
            extracted_data = {"raw_content": content}
            self.stats["failed_extractions"] += 1
        
        return {
            "uuid": uuid,
            "target_table": target_table,
            "subtype": subtype,
            "confidence": confidence,
            "extracted_data": extracted_data,
            "ml_context": {
                "kmeans_cluster": int(kmeans_cluster),
                "dbscan_cluster": int(dbscan_cluster),
                "is_anomaly": bool(is_anomaly),
                "embedding": reduced_embedding.tolist(),
                "topic_distribution": topic_distribution.tolist() if topic_distribution is not None else None
            },
            "pattern_scores": pattern_scores,
            "ml_features": ml_features
        }
    
    def _pattern_classification_scoring(self, content: str) -> dict[str, float]:
        """Score content against all Dragon's Labyrinth patterns."""
        
        scores = {}
        
        for table_name, patterns in self.patterns.items():
            matches = sum(1 for pattern in patterns if pattern.search(content))
            scores[table_name] = matches / len(patterns) if patterns else 0.0
        
        return scores
    
    def _extract_comprehensive_ml_features(self, content: str) -> dict[str, Any]:
        """Extract comprehensive ML features for classification."""
        
        content_lower = content.lower()
        
        return {
            # Structural features
            "length": len(content),
            "word_count": len(content.split()),
            "line_count": len(content.split('\n')),
            "paragraph_count": len(re.split(r'\n\n+', content)),
            "sentence_count": len(re.split(r'[.!?]', content)),
            
            # Content type indicators
            "has_stat_blocks": bool(re.search(r'(hp|ac|str|dex|con)', content_lower)),
            "has_dialogue": bool(re.search(r'["\'].*?["\']', content)),
            "has_dice_notation": bool(re.search(r'\d+d\d+', content)),
            "number_count": len(re.findall(r'\d+', content)),
            "currency_mentions": len(re.findall(r'\d+\s*(?:gp|sp|cp)', content_lower)),
            
            # Geographic indicators
            "has_coordinates": bool(re.search(r'Hex\s+[A-Z0-9]+', content)),
            "has_directions": len(re.findall(r'(north|south|east|west)', content_lower)),
            "region_mentions": self._count_region_mentions(content),
            
            # Entity indicators  
            "named_entities": len(re.findall(r'[A-Z][a-z]+(?:\s+[A-Z][a-z]+)*', content)),
            "title_mentions": len(re.findall(r'(Lord|Lady|Captain|Elder|King|Queen)', content)),
            "class_mentions": len(re.findall(r'(wizard|fighter|cleric|rogue|paladin)', content_lower)),
            
            # Dragon's Labyrinth specific features
            "corruption_indicators": self._count_corruption_indicators(content),
            "horror_elements": self._count_horror_elements(content),
            "violence_level": self._assess_violence_level(content),
            "trauma_triggers": self._count_trauma_triggers(content),
            "philosophy_indicators": self._assess_philosophy_indicators(content),
            "companion_relevance": self._assess_companion_relevance(content),
            "forge_relevance": self._assess_forge_relevance(content),
            
            # Settlement indicators
            "population_indicators": self._assess_population_size(content),
            "service_mentions": self._count_service_mentions(content),
            "trade_indicators": self._count_trade_indicators(content),
            
            # Dungeon indicators
            "trap_mentions": len(re.findall(r'(trap|puzzle|secret|hidden)', content_lower)),
            "treasure_mentions": len(re.findall(r'(treasure|hoard|chest|vault)', content_lower)),
            "boss_indicators": len(re.findall(r'(guardian|champion|ancient|powerful)', content_lower)),
            
            # Faction indicators
            "organization_indicators": self._count_organization_indicators(content),
            "conflict_indicators": self._count_conflict_indicators(content),
            "loyalty_indicators": self._count_loyalty_indicators(content)
        }
    
    def _ensemble_classification(
        self,
        content: str,
        pattern_scores: dict[str, float],
        ml_features: dict[str, Any], 
        embedding: np.ndarray
    ) -> tuple[str, str | None, float]:
        """
        Ensemble classification combining multiple ML signals.
        
        Returns:
            (target_table, subtype, confidence_score)
        """
        
        final_scores = {}
        
        # Pattern matching weight (40% - proven reliable)
        for table, score in pattern_scores.items():
            if score > 0:
                final_scores[table] = score * 0.4
        
        # ML feature scoring weight (35% - comprehensive)
        feature_scores = self._ml_feature_scoring(ml_features)
        for table, score in feature_scores.items():
            final_scores[table] = final_scores.get(table, 0) + score * 0.35
        
        # Embedding similarity weight (25% - learns from memory)
        if self.entity_memory:
            similarity_scores = self._embedding_similarity_scoring(embedding)
            for table, score in similarity_scores.items():
                final_scores[table] = final_scores.get(table, 0) + score * 0.25
        
        # Determine best classification
        if not final_scores:
            return "unknown", None, 0.0
        
        best_table = max(final_scores, key=final_scores.get)
        confidence = min(0.99, max(0.0, final_scores[best_table]))
        
        # Determine subtype for coordinator tables
        subtype = None
        if best_table in ["dungeon", "settlement", "dwelling", "factions"]:
            subtype = self._determine_coordinator_subtype(content, best_table)
            # Use subtype as actual table name
            if subtype:
                best_table = subtype
        
        return best_table, subtype, confidence
    
    def _determine_coordinator_subtype(self, content: str, coordinator: str) -> str | None:
        """Determine specific subtype for coordinator tables."""
        
        if coordinator == "dungeon":
            subtypes = ["cave", "temple", "tomb"]
        elif coordinator == "settlement":
            subtypes = ["city", "town", "village"] 
        elif coordinator == "dwelling":
            subtypes = ["farms_cabins", "stronghold"]
        elif coordinator == "factions":
            subtypes = ["cult", "militia", "syndicate"]
        else:
            return None
        
        # Score each subtype using patterns
        subtype_scores = {}
        for subtype in subtypes:
            if subtype in self.patterns:
                matches = sum(1 for pattern in self.patterns[subtype] if pattern.search(content))
                subtype_scores[subtype] = matches
        
        if subtype_scores:
            return max(subtype_scores, key=subtype_scores.get)
        return None
    
    def _deep_extraction_by_type(self, content: str, target_table: str, subtype: str | None) -> dict[str, Any]:
        """Deep extraction based on target table type."""
        
        # Import extractors module when needed
        try:
            from generator.entities.extractors import entity_extractor
            return entity_extractor.extract(target_table, content)
        except ImportError:
            # Fallback to basic extraction
            return self._basic_extraction_fallback(content, target_table)
    
    def _basic_extraction_fallback(self, content: str, target_table: str) -> dict[str, Any]:
        """Basic extraction fallback when extractors not available."""
        
        base_data = {
            "description": self._extract_clean_description(content),
            "corruption_level": self._assess_corruption_level(content),
            "region": self._extract_region_from_content(content)
        }
        
        # Add table-specific basics
        if target_table == "monster":
            base_data.update({
                "name": self._extract_entity_name(content),
                "challenge_rating": self._extract_basic_cr(content),
                "hit_points": self._extract_basic_hp(content)
            })
        elif target_table == "biome":
            base_data.update({
                "biome_type": self._extract_biome_type(content),
                "coordinate": self._extract_hex_coordinate(content)
            })
        elif target_table in ["city", "town", "village"]:
            base_data.update({
                "settlement_type": target_table,
                "settlement_name": self._extract_entity_name(content),
                "services": self._extract_basic_services(content)
            })
        
        return base_data
    
    def _update_entity_memory(self, result: dict[str, Any]) -> None:
        """Update entity memory for continuous learning."""
        
        table = result["target_table"]
        confidence = result["confidence"]
        
        # Only store high-confidence results
        if confidence < 0.6:
            return
        
        # Store in appropriate memory location
        memory_entry = {
            "uuid": result["uuid"],
            "confidence": confidence,
            "embedding": result["ml_context"]["embedding"],
            "features": result["ml_features"],
            "extracted_data": result["extracted_data"]
        }
        
        if table in self.entity_memory:
            self.entity_memory[table].append(memory_entry)
        elif table in ["cave", "temple", "tomb"]:
            self.entity_memory["dungeons"][table].append(memory_entry)
        elif table in ["city", "town", "village"]:
            self.entity_memory["settlements"][table].append(memory_entry)
        elif table in ["farms_cabins", "stronghold"]:
            self.entity_memory["dwellings"][table].append(memory_entry)
        elif table in ["cult", "militia", "syndicate"]:
            self.entity_memory["factions"][table].append(memory_entry)
        
        # Limit memory size for performance
        for category in self.entity_memory:
            if isinstance(self.entity_memory[category], list):
                if len(self.entity_memory[category]) > 500:
                    self.entity_memory[category] = self.entity_memory[category][-500:]
            elif isinstance(self.entity_memory[category], dict):
                for subcategory in self.entity_memory[category]:
                    if len(self.entity_memory[category][subcategory]) > 200:
                        self.entity_memory[category][subcategory] = self.entity_memory[category][subcategory][-200:]
    
    def _discover_entity_relationships(self, results: list[dict], embeddings: np.ndarray) -> list[dict]:
        """Discover relationships between entities using embeddings."""
        
        if len(results) < 2:
            return []
        
        relationships = []
        
        # Compute similarity matrix
        try:
            similarity_matrix = cosine_similarity(embeddings)
        except:
            return []
        
        # Find strong relationships (similarity > 0.75)
        for i in range(len(results)):
            for j in range(i + 1, len(results)):
                similarity = similarity_matrix[i, j]
                if similarity > 0.75:
                    rel_type = self._infer_relationship_type(results[i], results[j])
                    relationships.append({
                        "entity1_uuid": results[i]["uuid"],
                        "entity1_table": results[i]["target_table"],
                        "entity2_uuid": results[j]["uuid"],
                        "entity2_table": results[j]["target_table"],
                        "similarity_score": float(similarity),
                        "relationship_type": rel_type,
                        "confidence": min(1.0, similarity * 1.2)  # Boost confidence slightly
                    })
        
        return relationships
    
    def _generate_analysis_csvs(self, results: list[dict]) -> dict[str, str]:
        """Generate analysis CSVs like hex_tiles_full.csv for inspection."""
        
        # Group results by table
        by_table = {}
        for result in results:
            table = result["target_table"]
            if table not in by_table:
                by_table[table] = []
            
            # Flatten result for CSV
            csv_row = {
                "hbf_uuid": result["uuid"],
                "confidence_score": result["confidence"],
                "is_anomaly": result["ml_context"]["is_anomaly"],
                "kmeans_cluster": result["ml_context"]["kmeans_cluster"],
                **result["extracted_data"]
            }
            by_table[table].append(csv_row)
        
        # Generate CSV content for each table
        csv_outputs = {}
        for table, entities in by_table.items():
            if entities:
                csv_content = self._entities_to_csv(entities, table)
                csv_outputs[f"{table}_extracted.csv"] = csv_content
                
                # Write to hbf_analysis for inspection
                output_path = Path("hbf_analysis") / f"{table}_extracted.csv"
                output_path.parent.mkdir(exist_ok=True)
                
                with open(output_path, "w", encoding="utf-8") as f:
                    f.write(csv_content)
        
        return csv_outputs
    
    def _entities_to_csv(self, entities: list[dict], table_name: str) -> str:
        """Convert entity list to CSV format."""
        
        if not entities:
            return f"# No {table_name} entities extracted\n"
        
        # Get all possible headers
        all_headers = set()
        for entity in entities:
            all_headers.update(entity.keys())
        
        headers = sorted(list(all_headers))
        
        # Generate CSV
        csv_lines = [",".join(headers)]
        
        for entity in entities:
            row = []
            for header in headers:
                value = entity.get(header, "")
                
                # Handle complex types
                if isinstance(value, (list, dict)):
                    value = json.dumps(value)
                else:
                    value = str(value)
