
// Auto-generated Bevy integration for ui variants
// Generated: 2025-08-25T22:56:08.361179
// Total variants: 0
// Sprite sheets: 0
// Architecture: Universal variant system v2.0

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Asset path constants


// Sprite sheet path constants  


#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiVariantAtlas {
    pub sprite_sheets: HashMap<String, Handle<TextureAtlas>>,
    pub individual_variants: HashMap<String, Handle<Image>>,
    pub variant_metadata: HashMap<String, VariantMetadata>,
    pub archetype_lookup: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantMetadata {
    pub base_archetype: String,
    pub variant_combination: HashMap<String, String>,
    pub sprite_sheet_group: String,
    pub atlas_index: Option<usize>,
    pub resolution: String,
    pub layer_type: String,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantQuery {
    pub archetype: String,
    pub required_variants: HashMap<String, String>,
    pub fallback_allowed: bool,
}

pub struct UiVariantPlugin;

impl Plugin for UiVariantPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_ui_variants)
            .init_resource::<UiVariantAtlas>()
            .add_systems(Update, (
                update_variant_cache,
                handle_variant_requests
            ));
    }
}

fn load_ui_variants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut variant_atlas = UiVariantAtlas::default();

    // Load sprite sheets with atlas data
    // TODO: Load actual atlas JSON and create TextureAtlas resources

    // Load individual variants for fallback
    // TODO: Load individual variant images

    commands.insert_resource(variant_atlas);
}

fn update_variant_cache(
    // TODO: System to cache frequently used variants
) {
    // Implementation for variant caching optimization
}

fn handle_variant_requests(
    // TODO: System to handle runtime variant selection
) {
    // Implementation for dynamic variant selection
}

// Helper functions for variant selection
impl UiVariantAtlas {
    pub fn get_variant_by_combination(
        &self,
        base_archetype: &str,
        required_variants: &HashMap<String, String>,
    ) -> Option<Handle<Image>> {
        // Find exact match first
        for (variant_name, metadata) in &self.variant_metadata {
            if metadata.base_archetype == base_archetype {
                let matches = required_variants.iter().all(|(key, value)| {
                    metadata.variant_combination.get(key).map_or(false, |v| v == value)
                });
                
                if matches {
                    return self.individual_variants.get(variant_name).cloned();
                }
            }
        }
        
        // Fallback to any variant of the archetype
        self.get_random_variant_of_archetype(base_archetype)
    }
    
    pub fn get_random_variant_of_archetype(&self, base_archetype: &str) -> Option<Handle<Image>> {
        let matching_variants: Vec<_> = self.variant_metadata
            .iter()
            .filter(|(_, metadata)| metadata.base_archetype == base_archetype)
            .collect();
            
        if !matching_variants.is_empty() {
            let random_index = fastrand::usize(..matching_variants.len());
            let (variant_name, _) = matching_variants[random_index];
            return self.individual_variants.get(variant_name).cloned();
        }
        
        None
    }
    
    pub fn get_variants_by_archetype(&self, base_archetype: &str) -> Vec<String> {
        self.archetype_lookup
            .get(base_archetype)
            .cloned()
            .unwrap_or_default()
    }
    
    pub fn query_variants(&self, query: &VariantQuery) -> Vec<String> {
        let mut matching_variants = Vec::new();
        
        for (variant_name, metadata) in &self.variant_metadata {
            if metadata.base_archetype == query.archetype {
                let matches = if query.fallback_allowed {
                    // Partial matching allowed
                    query.required_variants.iter().any(|(key, value)| {
                        metadata.variant_combination.get(key).map_or(false, |v| v == value)
                    })
                } else {
                    // Exact matching required
                    query.required_variants.iter().all(|(key, value)| {
                        metadata.variant_combination.get(key).map_or(false, |v| v == value)
                    })
                };
                
                if matches {
                    matching_variants.push(variant_name.clone());
                }
            }
        }
        
        matching_variants
    }
}

// Component for entities that need variant selection
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct UiVariantSelector {
    pub archetype: String,
    pub preferred_variants: HashMap<String, String>,
    pub current_variant: Option<String>,
    pub allow_fallback: bool,
}

impl UiVariantSelector {
    pub fn new(archetype: &str) -> Self {
        Self {
            archetype: archetype.to_string(),
            preferred_variants: HashMap::new(),
            current_variant: None,
            allow_fallback: true,
        }
    }
    
    pub fn with_variant(mut self, dimension: &str, value: &str) -> Self {
        self.preferred_variants.insert(dimension.to_string(), value.to_string());
        self
    }
    
    pub fn strict_matching(mut self) -> Self {
        self.allow_fallback = false;
        self
    }
}
