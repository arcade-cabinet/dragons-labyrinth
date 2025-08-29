"""
Training System - Learn patterns from minimal anchors using ML discovery.

Starts with only world name and hex pattern, discovers everything else through
ML clustering, vectorization, and pattern analysis. Saves learned patterns
for iterative improvement across runs.
"""

from __future__ import annotations

import json
import logging
import re
from pathlib import Path
from typing import Any

import numpy as np
from sklearn.cluster import KMeans, DBSCAN
from sklearn.feature_extraction.text import TfidfVectorizer, CountVectorizer
from sklearn.decomposition import TruncatedSVD
from sklearn.metrics.pairwise import cosine_similarity


class TrainingSystem:
    """
    ML-driven training system that learns patterns from minimal anchors.
    
    Features:
    - Starts from anchors.json (world name + hex pattern only)
    - Discovers regions, biomes, entity types through clustering
    - Learns patterns through vectorization and similarity analysis
    - Saves learned patterns to training/ directory for future runs
    - Iteratively improves with each run
    """
    
    def __init__(self):
        self.training_dir = Path("training")
        self.anchors = self._load_anchors()
        self.learned_patterns = {}
        self.discovery_stats = {}
        
        # Load existing training data if available
        self._load_existing_training_data()
        
        # ML components for discovery
        self.vectorizer = TfidfVectorizer(
            max_features=3000,
            ngram_range=(1, 3),
            stop_words='english',
            min_df=2,
            max_df=0.9
        )
        
        self.logger = logging.getLogger("training_system")
    
    def _load_anchors(self) -> dict[str, Any]:
        """Load minimal anchor patterns."""
        
        anchor_path = self.training_dir / "meta" / "anchors.json"
        
        try:
            with open(anchor_path, "r", encoding="utf-8") as f:
                return json.load(f)
        except FileNotFoundError:
            # Absolute minimal anchors if file missing
            return {
                "world_name": "The Lands of Vo'il",
                "known_patterns": {
                    "hex_pattern": r"^([a-zA-Z0-9]{8})\s+Hex\s+([A-Z0-9]+)\s+in\s+([^(]+)\s*\(([^)]+)\)\s*([^>]*?)\s*>",
                    "world_validation": "The Lands of Vo'il"
                },
                "target_tables": [
                    "biome", "monster", "inn", "cave", "temple", "tomb",
                    "city", "town", "village", "farms_cabins", "stronghold", 
                    "cult", "militia", "syndicate"
                ]
            }
    
    def _load_existing_training_data(self) -> None:
        """Load any existing training data from previous runs."""
        
        self.learned_patterns = {}
        
        for table in self.anchors["target_tables"]:
            training_file = self.training_dir / f"{table}" / "patterns.json"
            
            if training_file.exists():
                try:
                    with open(training_file, "r", encoding="utf-8") as f:
                        self.learned_patterns[table] = json.load(f)
                except:
                    self.learned_patterns[table] = {"patterns": [], "keywords": [], "examples": []}
            else:
                self.learned_patterns[table] = {"patterns": [], "keywords": [], "examples": []}
    
    def discover_patterns_from_entities(self, entities: list[tuple[str, str]]) -> dict[str, Any]:
        """
        Discover patterns from entities using pure ML with minimal anchors.
        
        Args:
            entities: List of (uuid, content) tuples
            
        Returns:
            Discovery results with learned patterns
        """
        
        if not entities:
            return {}
        
        self.logger.info(f"Starting pattern discovery from {len(entities)} entities using minimal anchors")
        
        # Extract texts
        texts = [content for _, content in entities]
        uuids = [uuid for uuid, _ in entities]
        
        # Step 1: Use anchor patterns to find initial high-confidence entities
        anchor_entities = self._find_anchor_entities(entities)
        
        # Step 2: Vectorize all content for similarity analysis
        X = self._vectorize_content(texts)
        
        # Step 3: Use clustering to discover natural groups
        clusters = self._discover_clusters(X, texts, uuids)
        
        # Step 4: Analyze clusters to discover table types and patterns
        discovered_tables = self._analyze_clusters_for_tables(clusters, texts)
        
        # Step 5: Learn patterns from high-confidence examples
        learned_patterns = self._learn_patterns_from_examples(discovered_tables)
        
        # Step 6: Save learned patterns for future runs
        self._save_learned_patterns(learned_patterns)
        
        # Step 7: Apply learned patterns to classify all entities
        classifications = self._classify_entities_with_learned_patterns(entities, learned_patterns)
        
        return {
            "anchor_entities": anchor_entities,
            "discovered_clusters": clusters,
            "discovered_tables": discovered_tables,
            "learned_patterns": learned_patterns,
            "entity_classifications": classifications,
            "discovery_stats": self.discovery_stats
        }
    
    def _find_anchor_entities(self, entities: list[tuple[str, str]]) -> dict[str, list[str]]:
        """Find high-confidence entities using anchor patterns."""
        
        anchor_entities = {"hex_tiles": [], "world_references": []}
        
        world_name = self.anchors["world_name"]
        hex_pattern = re.compile(self.anchors["known_patterns"]["hex_pattern"], re.MULTILINE)
        
        for uuid, content in entities:
            # Check for hex pattern match
            if hex_pattern.search(content) and world_name in content:
                anchor_entities["hex_tiles"].append(uuid)
            
            # Check for world references (potential regions)
            elif world_name in content:
                anchor_entities["world_references"].append(uuid)
        
        self.discovery_stats["anchor_entities"] = {
            "hex_tiles": len(anchor_entities["hex_tiles"]),
            "world_references": len(anchor_entities["world_references"])
        }
        
        return anchor_entities
    
    def _vectorize_content(self, texts: list[str]) -> np.ndarray:
        """Vectorize content for ML analysis."""
        
        try:
            X = self.vectorizer.fit_transform(texts)
            
            # Reduce dimensionality for clustering
            svd = TruncatedSVD(n_components=min(100, X.shape[1]), random_state=42)
            X_reduced = svd.fit_transform(X)
            
            return X_reduced
            
        except Exception as e:
            self.logger.warning(f"Vectorization failed: {e}")
            return np.random.random((len(texts), 10))  # Fallback
    
    def _discover_clusters(self, X: np.ndarray, texts: list[str], uuids: list[str]) -> dict[str, Any]:
        """Discover natural clusters in content."""
        
        if len(texts) < 5:
            return {"error": "Too few entities for clustering"}
        
        # Try different clustering approaches
        clustering_results = {}
        
        # KMeans clustering
        try:
            n_clusters = min(self.anchors.get("ml_config", {}).get("clustering_params", {}).get("max_clusters", 15), 
                           max(3, len(texts) // 8))
            
            kmeans = KMeans(n_clusters=n_clusters, random_state=42, n_init=10)
            kmeans_labels = kmeans.fit_predict(X)
            
            clustering_results["kmeans"] = self._analyze_clustering_result(kmeans_labels, texts, uuids, "kmeans")
            
        except Exception as e:
            self.logger.warning(f"KMeans clustering failed: {e}")
        
        # DBSCAN clustering for density-based discovery
        try:
            dbscan = DBSCAN(eps=0.3, min_samples=3)
            dbscan_labels = dbscan.fit_predict(X)
            
            clustering_results["dbscan"] = self._analyze_clustering_result(dbscan_labels, texts, uuids, "dbscan")
            
        except Exception as e:
            self.logger.warning(f"DBSCAN clustering failed: {e}")
        
        self.discovery_stats["clustering"] = {
            "methods_attempted": len(clustering_results),
            "successful_methods": len([r for r in clustering_results.values() if "error" not in r])
        }
        
        return clustering_results
    
    def _analyze_clustering_result(self, labels: np.ndarray, texts: list[str], uuids: list[str], method: str) -> dict[str, Any]:
        """Analyze a clustering result to extract patterns."""
        
        # Group by cluster
        clusters = {}
        for i, label in enumerate(labels):
            if label == -1:  # DBSCAN noise
                continue
                
            if label not in clusters:
                clusters[label] = {"indices": [], "texts": [], "uuids": []}
            
            clusters[label]["indices"].append(i)
            clusters[label]["texts"].append(texts[i])
            clusters[label]["uuids"].append(uuids[i])
        
        # Analyze each cluster
        cluster_analysis = {}
        for cluster_id, cluster_data in clusters.items():
            if len(cluster_data["texts"]) < 2:  # Skip tiny clusters
                continue
            
            # Extract common terms
            common_terms = self._extract_cluster_terms(cluster_data["texts"])
            
            # Infer cluster type
            cluster_type = self._infer_cluster_type(cluster_data["texts"], common_terms)
            
            # Extract representative patterns
            patterns = self._extract_cluster_patterns(cluster_data["texts"])
            
            cluster_analysis[f"cluster_{cluster_id}"] = {
                "size": len(cluster_data["texts"]),
                "common_terms": common_terms,
                "inferred_type": cluster_type,
                "learned_patterns": patterns,
                "sample_uuids": cluster_data["uuids"][:3],
                "sample_content": [text[:100] + "..." for text in cluster_data["texts"][:2]]
            }
        
        return {
            "method": method,
            "num_clusters": len(clusters),
            "total_entities": len([l for l in labels if l != -1]),
            "noise_entities": int((labels == -1).sum()) if method == "dbscan" else 0,
            "clusters": cluster_analysis
        }
    
    def _extract_cluster_terms(self, cluster_texts: list[str]) -> list[str]:
        """Extract characteristic terms from cluster."""
        
        # Combine all cluster text
        combined_text = " ".join(cluster_texts)
        
        # Extract meaningful terms (3+ chars, not too common)
        words = re.findall(r'\b[A-Za-z]{3,}\b', combined_text.lower())
        
        # Count frequency
        word_freq = {}
        for word in words:
            if word not in ["the", "and", "with", "for", "this", "that", "from", "they", "have", "been"]:
                word_freq[word] = word_freq.get(word, 0) + 1
        
        # Return top characteristic terms
        sorted_words = sorted(word_freq.items(), key=lambda x: x[1], reverse=True)
        return [word for word, count in sorted_words[:10] if count >= 2]
    
    def _infer_cluster_type(self, cluster_texts: list[str], common_terms: list[str]) -> str:
        """Infer what type of entity this cluster represents."""
        
        combined_text = " ".join(cluster_texts).lower()
        
        # Use anchor pattern first
        world_name = self.anchors["world_name"].lower()
        hex_pattern = re.compile(r"hex\s+[a-z0-9]+", re.I)
        
        if world_name in combined_text and hex_pattern.search(combined_text):
            return "biome"  # Hex tiles with world name
        
        # Use common terms to infer type
        type_hints = {
            "monster": ["creature", "monster", "beast", "hp", "ac", "str", "dex", "con"],
            "cave": ["cave", "cavern", "underground", "tunnel", "mining"],
            "temple": ["temple", "shrine", "altar", "holy", "sacred", "worship"],
            "tomb": ["tomb", "crypt", "burial", "undead", "ancient", "curse"],
            "city": ["city", "metropolis", "district", "thousands", "walls"],
            "town": ["town", "market", "merchant", "trade", "shops"],
            "village": ["village", "hamlet", "farm", "rural", "small"],
            "inn": ["inn", "tavern", "isolated", "rest", "healing"],
            "stronghold": ["stronghold", "fortress", "castle", "garrison", "military"],
            "farms_cabins": ["farm", "cabin", "cottage", "crops", "livestock"],
            "cult": ["cult", "worship", "ritual", "sacrifice", "dark"],
            "militia": ["militia", "guard", "patrol", "defend", "soldier"],
            "syndicate": ["syndicate", "gang", "crime", "smuggle", "underground"]
        }
        
        # Score each potential type
        type_scores = {}
        for entity_type, keywords in type_hints.items():
            score = sum(1 for keyword in keywords if any(keyword in term for term in common_terms))
            if score > 0:
                type_scores[entity_type] = score / len(keywords)
        
        if type_scores:
            return max(type_scores, key=type_scores.get)
        else:
            return "unknown"
    
    def _extract_cluster_patterns(self, cluster_texts: list[str]) -> list[str]:
        """Extract regex patterns from cluster examples."""
        
        patterns = []
        
        # Look for common structural patterns
        for text in cluster_texts[:5]:  # Analyze first 5 examples
            # Extract potential name patterns
            name_matches = re.findall(r'^([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)', text, re.MULTILINE)
            if name_matches:
                patterns.append(f"Name pattern: {name_matches[0]}")
            
            # Extract numeric patterns
            numbers = re.findall(r'\d+', text)
            if len(numbers) > 3:
                patterns.append("Contains multiple numbers (possible stats)")
            
            # Extract structural markers
            if ":" in text:
                colon_patterns = re.findall(r'([A-Z][a-z]+):\s*([^:\n]+)', text)
                for pattern in colon_patterns[:2]:  # First 2
                    patterns.append(f"Structure: {pattern[0]}: {pattern[1][:20]}...")
        
        return patterns
    
    def _analyze_clusters_for_tables(self, clusters: dict[str, Any], texts: list[str]) -> dict[str, Any]:
        """Analyze clusters to discover table types and characteristics."""
        
        discovered_tables = {}
        
        # Analyze each clustering method
        for method, cluster_data in clusters.items():
            if "error" in cluster_data:
                continue
            
            method_discoveries = {}
            
            for cluster_name, cluster_info in cluster_data.get("clusters", {}).items():
                inferred_type = cluster_info["inferred_type"]
                cluster_size = cluster_info["size"]
                
                if inferred_type != "unknown" and cluster_size >= 3:
                    if inferred_type not in method_discoveries:
