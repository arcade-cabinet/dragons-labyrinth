//! Removes empty placeholder entities from HBF data
//! 
//! This transformer identifies and removes the 68,556 placeholder entities
//! that contain only basic fields (created_at, updated_at, uuid) with no actual content.

use serde_json::Value;
use std::collections::HashSet;

/// Removes empty placeholder entities from the HBF data
pub struct EmptyRemover {
    removed_count: usize,
    removed_uuids: HashSet<String>,
}

impl EmptyRemover {
    pub fn new() -> Self {
        Self {
            removed_count: 0,
            removed_uuids: HashSet::new(),
        }
    }

    /// Process entities and remove empty ones
    pub fn process(&mut self, entities: Vec<Value>) -> Vec<Value> {
        entities.into_iter()
            .filter(|entity| {
                if self.is_empty_entity(entity) {
                    // Track removed UUID for reporting
                    if let Some(uuid) = entity.get("uuid").and_then(|v| v.as_str()) {
                        self.removed_uuids.insert(uuid.to_string());
                    }
                    self.removed_count += 1;
                    false
                } else {
                    true
                }
            })
            .collect()
    }

    /// Check if an entity is empty (only has uuid, no content)
    fn is_empty_entity(&self, entity: &Value) -> bool {
        let obj = match entity.as_object() {
            Some(o) => o,
            None => return false,
        };

        // Check if content field is empty/null
        if let Some(content_obj) = obj.get("content") {
            if let Some(content_map) = content_obj.as_object() {
                if let Some(content_str) = content_map.get("content") {
                    if let Some(content_text) = content_str.as_str() {
                        // Empty if content is null, empty string, or just whitespace
                        return content_text.trim().is_empty() || content_text == "null";
                    }
                }
            }
        }
        
        // Also empty if no content field at all
        !obj.contains_key("content") || obj.get("content").is_none()
    }

    /// Get statistics about removed entities
    pub fn get_stats(&self) -> EmptyRemovalStats {
        EmptyRemovalStats {
            removed_count: self.removed_count,
            removed_uuids: self.removed_uuids.clone(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct EmptyRemovalStats {
    pub removed_count: usize,
    pub removed_uuids: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_identifies_empty_entity() {
        let remover = EmptyRemover::new();
        
        let empty = json!({
            "created_at": 1234567890,
            "updated_at": 1234567891,
            "uuid": "test-uuid"
        });
        
        assert!(remover.is_empty_entity(&empty));
    }

    #[test]
    fn test_keeps_content_entity() {
        let remover = EmptyRemover::new();
        
        let with_content = json!({
            "created_at": 1234567890,
            "updated_at": 1234567891,
            "uuid": "test-uuid",
            "name": "Test Settlement"
        });
        
        assert!(!remover.is_empty_entity(&with_content));
    }

    #[test]
    fn test_removes_empty_entities() {
        let mut remover = EmptyRemover::new();
        
        let entities = vec![
            json!({
                "created_at": 1,
                "updated_at": 2,
                "uuid": "empty-1"
            }),
            json!({
                "created_at": 3,
                "updated_at": 4,
                "uuid": "content-1",
                "name": "Village"
            }),
            json!({
                "created_at": 5,
                "updated_at": 6,
                "uuid": "empty-2"
            }),
        ];
        
        let filtered = remover.process(entities);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["uuid"], "content-1");
        
        let stats = remover.get_stats();
        assert_eq!(stats.removed_count, 2);
        assert!(stats.removed_uuids.contains("empty-1"));
        assert!(stats.removed_uuids.contains("empty-2"));
    }
}
