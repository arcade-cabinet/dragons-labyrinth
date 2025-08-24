"""
Dragon Asset Search Tool
Semantic search for CC0 library with horror progression awareness
Implements the 80/20 rule - search before generate
"""

from typing import List, Dict, Optional, Any
import sqlite3
import numpy as np
from sentence_transformers import SentenceTransformer
import chromadb
from chromadb.config import Settings
import logging
from pathlib import Path


class DragonAssetSearchTool:
    """
    Intelligent asset search tool for Dragon's Labyrinth
    Searches CC0 library with semantic understanding and horror awareness
    """
    
    def __init__(
        self,
        db_path: str = "assets/library/asset_index.db",
        chroma_path: str = "assets/library/chroma_db"
    ):
        self.db_path = db_path
        self.chroma_path = chroma_path
        self.logger = logging.getLogger("DragonAssetSearchTool")
        
        # Initialize semantic search model
        self.model = SentenceTransformer('all-MiniLM-L6-v2')
        
        # Initialize ChromaDB for vector search
        self.chroma_client = chromadb.PersistentClient(
            path=chroma_path,
            settings=Settings(anonymized_telemetry=False)
        )
        
        # Get or create collection
        try:
            self.collection = self.chroma_client.get_collection("dragon_assets")
            self.logger.info(f"Loaded existing collection with {self.collection.count()} assets")
        except ValueError:
            self.collection = self.chroma_client.create_collection(
                name="dragon_assets",
                metadata={"hnsw:space": "cosine"}
            )
            self.logger.info("Created new collection for assets")
    
    def index_cc0_library(self, force_reindex: bool = False) -> int:
        """
        Index all CC0 library assets with semantic embeddings
        Returns number of assets indexed
        """
        if not force_reindex and self.collection.count() > 0:
            self.logger.info(f"Collection already indexed with {self.collection.count()} assets")
            return self.collection.count()
        
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Get all assets from SQLite
        cursor.execute("""
            SELECT id, file_path, name, category, tags, 
                   performance_score, mobile_compatible
            FROM cc0_assets
        """)
        
        assets = cursor.fetchall()
        conn.close()
        
        if not assets:
            self.logger.warning("No assets found in database to index")
            return 0
        
        # Prepare data for ChromaDB
        ids = []
        documents = []
        metadatas = []
        
        for asset in assets:
            asset_id, file_path, name, category, tags, perf_score, mobile = asset
            
            # Create searchable document
            doc = f"{name} {category} {tags or ''}"
            
            ids.append(str(asset_id))
            documents.append(doc)
            metadatas.append({
                "file_path": file_path,
                "name": name,
                "category": category,
                "tags": tags or "",
                "performance_score": float(perf_score or 0.5),
                "mobile_compatible": bool(mobile)
            })
        
        # Generate embeddings and add to collection
        self.logger.info(f"Indexing {len(documents)} assets...")
        embeddings = self.model.encode(documents)
        
        # Add in batches for better performance
        batch_size = 100
        for i in range(0, len(ids), batch_size):
            batch_end = min(i + batch_size, len(ids))
            self.collection.add(
                ids=ids[i:batch_end],
                documents=documents[i:batch_end],
                embeddings=embeddings[i:batch_end].tolist(),
                metadatas=metadatas[i:batch_end]
            )
        
        self.logger.info(f"Successfully indexed {len(documents)} assets")
        return len(documents)
    
    def search_dragon_assets(
        self,
        query: str,
        dread_level: int = 0,
        category: Optional[str] = None,
        limit: int = 8,
        mobile_only: bool = True
    ) -> List[Dict[str, Any]]:
        """
        Search CC0 library with horror progression awareness
        
        Args:
            query: Semantic search query
            dread_level: Current horror progression (0-4)
            category: Filter by asset category
            limit: Maximum results to return
            mobile_only: Only return mobile-compatible assets
        
        Returns:
            List of ranked asset matches with metadata
        """
        # Enhance query based on dread level
        enhanced_query = self._enhance_query_for_dread(query, dread_level)
        
        # Build where clause for filtering
        where_clause = {}
        if category:
            where_clause["category"] = category
        if mobile_only:
            where_clause["mobile_compatible"] = True
        
        # Perform semantic search
        try:
            results = self.collection.query(
                query_texts=[enhanced_query],
                n_results=limit * 2,  # Get extra for post-filtering
                where=where_clause if where_clause else None
            )
        except Exception as e:
            self.logger.error(f"Search failed: {e}")
            return []
        
        # Process and rank results
        ranked_assets = []
        
        if results and results["ids"] and results["ids"][0]:
            for i, asset_id in enumerate(results["ids"][0]):
                metadata = results["metadatas"][0][i]
                distance = results["distances"][0][i] if "distances" in results else 0
                
                # Calculate composite score
                similarity_score = 1 - distance  # Convert distance to similarity
                performance_score = metadata.get("performance_score", 0.5)
                horror_score = self._calculate_horror_compatibility(
                    metadata, dread_level
                )
                
                composite_score = (
                    similarity_score * 0.4 +
                    performance_score * 0.3 +
                    horror_score * 0.3
                )
                
                ranked_assets.append({
                    "id": asset_id,
                    "file_path": metadata.get("file_path"),
                    "name": metadata.get("name"),
                    "category": metadata.get("category"),
                    "tags": metadata.get("tags"),
                    "similarity_score": similarity_score,
                    "performance_score": performance_score,
                    "horror_score": horror_score,
                    "composite_score": composite_score,
                    "mobile_compatible": metadata.get("mobile_compatible", False)
                })
        
        # Sort by composite score and return top results
        ranked_assets.sort(key=lambda x: x["composite_score"], reverse=True)
        return ranked_assets[:limit]
    
    def _enhance_query_for_dread(self, query: str, dread_level: int) -> str:
        """Enhance search query based on horror progression"""
        dread_modifiers = {
            0: "peaceful bright cheerful medieval fantasy",
            1: "unsettling shadows creepy medieval dark",
            2: "ominous corrupted swamp decay horror",
            3: "terrifying nightmare twisted evil darkness",
            4: "pure horror abyss nightmare terror dragon"
        }
        
        modifier = dread_modifiers.get(dread_level, "")
        return f"{query} {modifier}".strip()
    
    def _calculate_horror_compatibility(
        self,
        metadata: Dict[str, Any],
        dread_level: int
    ) -> float:
        """Calculate how well an asset fits the current horror level"""
        tags = metadata.get("tags", "").lower()
        name = metadata.get("name", "").lower()
        category = metadata.get("category", "").lower()
        
        combined_text = f"{tags} {name} {category}"
        
        # Keywords for each dread level
        dread_keywords = {
            0: ["peaceful", "bright", "cheerful", "village", "meadow", "sunny"],
            1: ["shadow", "creepy", "dark", "mysterious", "abandoned", "fog"],
            2: ["corrupted", "swamp", "decay", "rotten", "diseased", "toxic"],
            3: ["nightmare", "twisted", "evil", "demonic", "cursed", "blood"],
            4: ["horror", "abyss", "terror", "dragon", "labyrinth", "death"]
        }
        
        # Calculate keyword matches
        score = 0.5  # Base score
        
        # Bonus for matching current dread level
        current_keywords = dread_keywords.get(dread_level, [])
        for keyword in current_keywords:
            if keyword in combined_text:
                score += 0.1
        
        # Penalty for mismatched dread levels
        for level, keywords in dread_keywords.items():
            if abs(level - dread_level) > 1:  # Far from current level
                for keyword in keywords:
                    if keyword in combined_text:
                        score -= 0.05
        
        return max(0.0, min(1.0, score))
    
    def find_similar_assets(
        self,
        asset_id: str,
        limit: int = 5
    ) -> List[Dict[str, Any]]:
        """Find assets similar to a given asset"""
        try:
            # Get the asset's embedding
            result = self.collection.get(
                ids=[asset_id],
                include=["embeddings", "metadatas"]
            )
            
            if not result["embeddings"]:
                return []
            
            embedding = result["embeddings"][0]
            
            # Search for similar
            similar = self.collection.query(
                query_embeddings=[embedding],
                n_results=limit + 1  # +1 to exclude self
            )
            
            # Process results (exclude the original asset)
            similar_assets = []
            for i, found_id in enumerate(similar["ids"][0]):
                if found_id != asset_id:
                    metadata = similar["metadatas"][0][i]
                    similar_assets.append({
                        "id": found_id,
                        "file_path": metadata.get("file_path"),
                        "name": metadata.get("name"),
                        "category": metadata.get("category"),
                        "similarity": 1 - similar["distances"][0][i]
                    })
            
            return similar_assets[:limit]
            
        except Exception as e:
            self.logger.error(f"Failed to find similar assets: {e}")
            return []
    
    def get_asset_by_id(self, asset_id: str) -> Optional[Dict[str, Any]]:
        """Get specific asset by ID"""
        try:
            result = self.collection.get(
                ids=[asset_id],
                include=["metadatas"]
            )
            
            if result["metadatas"]:
                metadata = result["metadatas"][0]
                return {
                    "id": asset_id,
                    "file_path": metadata.get("file_path"),
                    "name": metadata.get("name"),
                    "category": metadata.get("category"),
                    "tags": metadata.get("tags"),
                    "performance_score": metadata.get("performance_score"),
                    "mobile_compatible": metadata.get("mobile_compatible")
                }
            
        except Exception as e:
            self.logger.error(f"Failed to get asset {asset_id}: {e}")
        
        return None
    
    def calculate_reuse_percentage(self) -> Dict[str, float]:
        """Calculate how much of the library is being reused (80/20 tracking)"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Get usage statistics
        cursor.execute("""
            SELECT 
                COUNT(DISTINCT id) as total_assets,
                SUM(CASE WHEN used_count > 0 THEN 1 ELSE 0 END) as used_assets,
                AVG(used_count) as avg_usage
            FROM cc0_assets
        """)
        
        result = cursor.fetchone()
        conn.close()
        
        if result:
            total, used, avg_usage = result
            reuse_percentage = (used / total * 100) if total > 0 else 0
            
            return {
                "total_assets": total,
                "used_assets": used or 0,
                "reuse_percentage": reuse_percentage,
                "average_usage": avg_usage or 0,
                "target_percentage": 80.0,
                "achieving_target": reuse_percentage >= 80.0
            }
        
        return {
            "total_assets": 0,
            "used_assets": 0,
            "reuse_percentage": 0,
            "average_usage": 0,
            "target_percentage": 80.0,
            "achieving_target": False
        }
    
    def mark_asset_used(self, asset_id: str) -> None:
        """Mark an asset as used (for tracking 80/20 rule)"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute("""
            UPDATE cc0_assets 
            SET used_count = used_count + 1,
                last_used = datetime('now')
            WHERE id = ?
        """, (asset_id,))
        
        conn.commit()
        conn.close()
        
        self.logger.info(f"Marked asset {asset_id} as used")


# Convenience function for LangChain tool integration
def create_search_tool() -> DragonAssetSearchTool:
    """Create and initialize the search tool"""
    tool = DragonAssetSearchTool()
    
    # Index library if needed
    if tool.collection.count() == 0:
        tool.index_cc0_library()
    
    return tool
