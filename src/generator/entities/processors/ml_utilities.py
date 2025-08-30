"""
ML Utilities - Clean ML functions for entity processing.

Direct ML functionality with no defensive programming:
- Multi-scale vectorization and clustering
- Feature extraction and analysis
- Relationship discovery
- Entity processing
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

import numpy as np
import pandas as pd
from sklearn.cluster import DBSCAN, KMeans
from sklearn.decomposition import LatentDirichletAllocation, TruncatedSVD
from sklearn.ensemble import IsolationForest
from sklearn.feature_extraction.text import CountVectorizer, TfidfVectorizer
from sklearn.manifold import TSNE
from sklearn.metrics.pairwise import cosine_similarity

from generator.constants import REGIONS, SETTLEMENTS, FACTIONS


def create_vectorizers() -> dict[str, Any]:
    """Create multi-scale vectorizers for content analysis."""
    return {
        "short": TfidfVectorizer(max_features=1000, ngram_range=(1, 2), stop_words='english'),
        "long": TfidfVectorizer(max_features=5000, ngram_range=(1, 3), stop_words='english', min_df=2, max_df=0.95),
        "semantic": CountVectorizer(max_features=10000, ngram_range=(1, 4), stop_words='english')
    }


def create_ml_components() -> dict[str, Any]:
    """Create ML components for dimensionality reduction and clustering."""
    return {
        "svd": TruncatedSVD(n_components=100, random_state=42),
        "lda": LatentDirichletAllocation(n_components=20, random_state=42, max_iter=10),
        "tsne": TSNE(n_components=2, random_state=42, perplexity=30),
        "dbscan": DBSCAN(eps=0.3, min_samples=5),
        "isolation_forest": IsolationForest(contamination=0.1, random_state=42)
    }


def process_entity_batch(entity_pairs: list[tuple[str, str]]) -> dict[str, Any]:
    """
    Process batch of entities with full ML pipeline.
    
    Args:
        entity_pairs: List of (uuid, content) tuples
        
    Returns:
        Processing results with classifications and relationships
    """
    
    texts = [content for _, content in entity_pairs]
    uuids = [uuid for uuid, _ in entity_pairs]
    
    # Create vectorizers and ML components
    vectorizers = create_vectorizers()
    ml_components = create_ml_components()
    
    # Multi-scale vectorization
    X_short = vectorizers["short"].fit_transform(texts)
    X_long = vectorizers["long"].fit_transform(texts) if len(texts) > 5 else X_short
    X_semantic = vectorizers["semantic"].fit_transform(texts)
    
    # Advanced dimensionality reduction
    X_reduced = ml_components["svd"].fit_transform(X_long)
    
    # Topic modeling for content understanding
    topics = ml_components["lda"].fit_transform(X_long) if len(texts) > 10 else None
    
    # Clustering ensemble for pattern discovery
    if len(texts) >= 3:
        n_clusters = min(15, max(3, len(texts) // 8))
        kmeans = KMeans(n_clusters=n_clusters, random_state=42, n_init=10)
        kmeans_labels = kmeans.fit_predict(X_reduced)
        dbscan_labels = ml_components["dbscan"].fit_predict(X_reduced)
    else:
        # Single entity or very small clusters - no clustering needed
        kmeans_labels = np.zeros(len(texts), dtype=int)
        dbscan_labels = np.zeros(len(texts), dtype=int)
    
    # Anomaly detection
    anomalies = ml_components["isolation_forest"].fit_predict(X_reduced)
    
    # Process each entity with ML context
    results = []
    for i, (uuid, content) in enumerate(entity_pairs):
        result = process_single_entity(
            uuid=uuid,
            content=content,
            reduced_embedding=X_reduced[i],
            kmeans_cluster=kmeans_labels[i],
            dbscan_cluster=dbscan_labels[i],
            is_anomaly=(anomalies[i] == -1),
            topic_distribution=topics[i] if topics is not None else None
        )
        results.append(result)
    
    # Discover relationships using embeddings
    relationships = discover_entity_relationships(results, X_reduced)
    
    return {
        "entities": results,
        "relationships": relationships,
        "cluster_analysis": analyze_clusters(kmeans_labels, dbscan_labels, texts),
        "anomaly_count": int((anomalies == -1).sum()),
        "processing_stats": {
            "total_processed": len(entity_pairs),
            "anomalies_found": int((anomalies == -1).sum()),
            "relationships_discovered": len(relationships)
        }
    }


def process_single_entity(
    uuid: str,
    content: str,
    reduced_embedding: np.ndarray,
    kmeans_cluster: int,
    dbscan_cluster: int,
    is_anomaly: bool,
    topic_distribution: np.ndarray | None
) -> dict[str, Any]:
    """Process single entity with full ML context."""
    
    # Extract comprehensive ML features
    ml_features = extract_comprehensive_ml_features(content)
    
    # Extract basic data from content
    extracted_data = extract_basic_entity_data(content)
    
    return {
        "uuid": uuid,
        "extracted_data": extracted_data,
        "ml_context": {
            "kmeans_cluster": int(kmeans_cluster),
            "dbscan_cluster": int(dbscan_cluster),
            "is_anomaly": bool(is_anomaly),
            "embedding": reduced_embedding.tolist(),
            "topic_distribution": topic_distribution.tolist() if topic_distribution is not None else None
        },
        "ml_features": ml_features,
        "confidence": calculate_entity_confidence(extracted_data, ml_features)
    }


def extract_comprehensive_ml_features(content: str) -> dict[str, Any]:
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
        
        # Entity indicators  
        "named_entities": len(re.findall(r'[A-Z][a-z]+(?:\s+[A-Z][a-z]+)*', content)),
        "title_mentions": len(re.findall(r'(Lord|Lady|Captain|Elder|King|Queen)', content)),
        "class_mentions": len(re.findall(r'(wizard|fighter|cleric|rogue|paladin)', content_lower)),
        
        # Horror/corruption indicators
        "corruption_words": len(re.findall(r'(tainted|corrupted|nightmare|cursed|void)', content_lower)),
        "horror_elements": len(re.findall(r'(dark|blood|bone|fear|terror|dread)', content_lower)),
        "violence_indicators": len(re.findall(r'(kill|murder|death|torture|pain)', content_lower)),
        
        # Settlement indicators
        "population_words": len(re.findall(r'(village|town|city|settlement|hundreds|thousands)', content_lower)),
        "service_mentions": len(re.findall(r'(tavern|inn|shop|market|temple)', content_lower)),
        "trade_indicators": len(re.findall(r'(merchant|trade|commerce|goods|caravan)', content_lower)),
        
        # Dungeon indicators
        "dungeon_words": len(re.findall(r'(crypt|tomb|cave|temple|lair|shrine)', content_lower)),
        "trap_mentions": len(re.findall(r'(trap|puzzle|secret|hidden)', content_lower)),
        "treasure_mentions": len(re.findall(r'(treasure|hoard|chest|vault|gold)', content_lower)),
        
        # Faction indicators
        "organization_words": len(re.findall(r'(guild|cult|militia|gang|order)', content_lower)),
        "conflict_words": len(re.findall(r'(war|battle|fight|enemy|alliance)', content_lower)),
        "loyalty_words": len(re.findall(r'(member|loyal|betray|faction|leader)', content_lower))
    }


def extract_basic_entity_data(content: str) -> dict[str, Any]:
    """Extract basic entity data from content."""
    
    # Try to parse as JSON first
    json_data = parse_json_content(content)
    if json_data:
        return json_data
    
    # Extract basic information from text
    return {
        "raw_content": content,
        "content_type": determine_content_type(content),
        "key_terms": extract_key_terms(content),
        "entities_mentioned": extract_mentioned_entities(content)
    }


def parse_json_content(content: str) -> dict[str, Any] | None:
    """Parse JSON content if possible."""
    content = content.strip()
    if content.startswith('{') and content.endswith('}'):
        return json.loads(content)
    return None


def determine_content_type(content: str) -> str:
    """Determine the type of content based on patterns."""
    content_lower = content.lower()
    
    if 'village of' in content_lower or 'town of' in content_lower or 'city of' in content_lower:
        return "settlement"
    elif any(word in content_lower for word in ['crypt', 'tomb', 'cave', 'temple', 'lair']):
        return "dungeon"
    elif any(word in content_lower for word in ['defiled', 'fists', 'swords', 'justice', 'wolves']):
        return "faction"
    elif 'hex' in content_lower and any(biome in content_lower for biome in ['forest', 'desert', 'mountain', 'plains']):
        return "biome"
    elif any(word in content_lower for word in ['aurora', 'fearless', 'vicious', 'ragthorn']):
        return "region"
    else:
        return "unknown"


def extract_key_terms(content: str) -> list[str]:
    """Extract key terms from content."""
    # Find capitalized words/phrases
    key_terms = re.findall(r'[A-Z][a-z]+(?:\s+[A-Z][a-z]+)*', content)
    # Remove duplicates and common words
    filtered_terms = []
    common_words = {'The', 'Of', 'And', 'In', 'To', 'With', 'For', 'On', 'At', 'By'}
    for term in key_terms:
        if term not in common_words and len(term) > 2:
            filtered_terms.append(term)
    return list(set(filtered_terms))[:10]  # Top 10 unique terms


def extract_mentioned_entities(content: str) -> list[str]:
    """Extract mentioned entities from content."""
    entities = []
    
    for entity_list in [REGIONS, SETTLEMENTS, FACTIONS]:
        for entity in entity_list:
            if entity in content:
                entities.append(entity)
    
    return entities


def calculate_entity_confidence(extracted_data: dict[str, Any], ml_features: dict[str, Any]) -> float:
    """Calculate confidence score for entity extraction."""
    confidence = 0.5  # Base confidence
    
    # Boost confidence based on content richness
    if extracted_data.get("content_type") != "unknown":
        confidence += 0.2
    
    if extracted_data.get("key_terms"):
        confidence += min(0.2, len(extracted_data["key_terms"]) * 0.02)
    
    if extracted_data.get("entities_mentioned"):
        confidence += min(0.1, len(extracted_data["entities_mentioned"]) * 0.05)
    
    # Boost confidence based on ML features
    feature_indicators = [
        ml_features.get("has_stat_blocks", False),
        ml_features.get("has_coordinates", False),
        ml_features.get("currency_mentions", 0) > 0,
        ml_features.get("named_entities", 0) > 0
    ]
    
    confidence += sum(feature_indicators) * 0.05
    
    return min(1.0, confidence)


def discover_entity_relationships(results: list[dict], embeddings: np.ndarray) -> list[dict]:
    """Discover relationships between entities using embeddings."""
    
    if len(results) < 2:
        return []
    
    relationships = []
    
    # Compute similarity matrix
    similarity_matrix = cosine_similarity(embeddings)
    
    # Find strong relationships (similarity > 0.75)
    for i in range(len(results)):
        for j in range(i + 1, len(results)):
            similarity = similarity_matrix[i, j]
            if similarity > 0.75:
                rel_type = infer_relationship_type(results[i], results[j])
                relationships.append({
                    "entity1_uuid": results[i]["uuid"],
                    "entity2_uuid": results[j]["uuid"],
                    "similarity_score": float(similarity),
                    "relationship_type": rel_type,
                    "confidence": min(1.0, similarity * 1.2)
                })
    
    return relationships


def infer_relationship_type(entity1: dict[str, Any], entity2: dict[str, Any]) -> str:
    """Infer relationship type between two entities."""
    
    type1 = entity1.get("extracted_data", {}).get("content_type", "unknown")
    type2 = entity2.get("extracted_data", {}).get("content_type", "unknown")
    
    # Same type = similar entities
    if type1 == type2:
        return f"similar_{type1}"
    
    # Cross-type relationships
    if (type1 == "settlement" and type2 == "region") or (type1 == "region" and type2 == "settlement"):
        return "located_in"
    elif (type1 == "faction" and type2 == "settlement") or (type1 == "settlement" and type2 == "faction"):
        return "operates_in"
    elif (type1 == "dungeon" and type2 == "region") or (type1 == "region" and type2 == "dungeon"):
        return "located_in"
    else:
        return "related"


def analyze_clusters(kmeans_labels: np.ndarray, dbscan_labels: np.ndarray, texts: list[str]) -> dict[str, Any]:
    """Analyze clustering results."""
    
    n_kmeans_clusters = len(set(kmeans_labels))
    n_dbscan_clusters = len(set(dbscan_labels)) - (1 if -1 in dbscan_labels else 0)  # Exclude noise
    
    return {
        "kmeans_clusters": n_kmeans_clusters,
        "dbscan_clusters": n_dbscan_clusters,
        "dbscan_noise_points": int((dbscan_labels == -1).sum()),
        "largest_cluster_size": int(np.bincount(kmeans_labels).max()),
        "cluster_distribution": {
            "kmeans": np.bincount(kmeans_labels).tolist(),
            "dbscan": np.bincount(dbscan_labels[dbscan_labels >= 0]).tolist() if len(dbscan_labels[dbscan_labels >= 0]) > 0 else []
        }
    }


def generate_csv_analysis(results: list[dict], table_name: str) -> str:
    """Generate CSV content for analysis like hex_tiles_full.csv."""
    
    if not results:
        return f"# No {table_name} entities extracted\n"
    
    # Get all possible headers
    all_headers = set()
    for result in results:
        all_headers.update(result.get("extracted_data", {}).keys())
        all_headers.update(result.get("ml_features", {}).keys())
        all_headers.add("uuid")
        all_headers.add("confidence")
    
    headers = sorted(list(all_headers))
    
    # Generate CSV
    csv_lines = [",".join(headers)]
    
    for result in results:
        row = []
        for header in headers:
            if header == "uuid":
                value = result.get("uuid", "")
            elif header == "confidence":
                value = result.get("confidence", 0.0)
            elif header in result.get("extracted_data", {}):
                value = result["extracted_data"][header]
            elif header in result.get("ml_features", {}):
                value = result["ml_features"][header]
            else:
                value = ""
            
            # Handle complex types
            if isinstance(value, (list, dict)):
                value = json.dumps(value)
            else:
                value = str(value)
            
            # Escape commas and quotes
            if ',' in value or '"' in value:
                value = f'"{value.replace('"', '""')}"'
            
            row.append(value)
        
        csv_lines.append(",".join(row))
    
    return "\n".join(csv_lines)
