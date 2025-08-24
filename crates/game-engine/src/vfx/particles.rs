use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use std::collections::HashMap;

/// Particle effects system using bevy_hanabi
/// This provides horror-responsive visual effects that degrade with dread
pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .init_resource::<ParticleEffectAssets>()
            .init_resource::<EffectConfiguration>()
            .add_event::<SpawnParticleEvent>()
            .add_systems(Startup, setup_particle_effects)
            .add_systems(Update, (
                update_dread_particles,
                spawn_corruption_particles,
                handle_dragon_breath_particles,
                process_companion_breakdown_particles,
                spawn_environmental_particles,
                handle_particle_events,
            ).chain());
    }
}

/// Particle effect assets organized by type and dread level
#[derive(Resource, Default)]
pub struct ParticleEffectAssets {
    /// Corruption spread particles for each dread level
    pub corruption_effects: HashMap<u8, Handle<EffectAsset>>,
    /// Dragon breath and presence effects
    pub dragon_effects: HashMap<String, Handle<EffectAsset>>,
    /// Companion emotional state particles
    pub companion_effects: HashMap<(String, u8), Handle<EffectAsset>>,
    /// Environmental effects (fog, ash, etc.)
    pub environmental_effects: HashMap<u8, Handle<EffectAsset>>,
    /// Forge trial effects
    pub forge_effects: HashMap<String, Handle<EffectAsset>>,
}

/// Configuration for particle effects
#[derive(Resource)]
pub struct EffectConfiguration {
    /// Global particle density multiplier
    pub particle_density: f32,
    /// Performance mode (reduces particles on weak hardware)
    pub performance_mode: bool,
    /// Maximum simultaneous effects
    pub max_effects: usize,
    /// Dread-based color shift
    pub dread_color_shift: Color,
}

impl Default for EffectConfiguration {
    fn default() -> Self {
        Self {
            particle_density: 1.0,
            performance_mode: false,
            max_effects: 50,
            dread_color_shift: Color::WHITE,
        }
    }
}

/// Event for spawning particle effects
#[derive(Event)]
pub struct SpawnParticleEvent {
    pub effect_type: ParticleEffectType,
    pub position: Vec3,
    pub intensity: f32,
    pub duration: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum ParticleEffectType {
    Corruption,
    DragonBreath,
    DragonPresence,
    CompanionBreakdown(String),
    EnvironmentalFog,
    EnvironmentalAsh,
    ForgeLight,
    ForgeDark,
    SanityDistortion,
}

/// Setup base particle effect templates
fn setup_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_assets: ResMut<ParticleEffectAssets>,
) {
    // Create corruption spread effect for each dread level
    for dread_level in 0..=4 {
        let mut gradient = Gradient::new();
        
        // Color gradient based on dread level
        let start_color = match dread_level {
            0 => Vec4::new(0.0, 1.0, 0.0, 0.3),  // Green (nature)
            1 => Vec4::new(0.5, 0.5, 0.0, 0.4),  // Yellow-brown (decay)
            2 => Vec4::new(0.8, 0.2, 0.0, 0.5),  // Orange-red (corruption)
            3 => Vec4::new(0.6, 0.0, 0.6, 0.6),  // Purple (void)
            4 => Vec4::new(0.1, 0.0, 0.1, 0.8),  // Dark purple (horror)
            _ => Vec4::new(0.5, 0.5, 0.5, 0.5),
        };
        
        gradient.add_key(0.0, start_color);
        gradient.add_key(0.5, start_color * 0.8);
        gradient.add_key(1.0, Vec4::new(start_color.x, start_color.y, start_color.z, 0.0));
        
        let effect = create_corruption_effect(gradient, dread_level);
        let handle = effects.add(effect);
        particle_assets.corruption_effects.insert(dread_level, handle);
    }
    
    // Create dragon effects
    particle_assets.dragon_effects.insert(
        "breath".to_string(),
        effects.add(create_dragon_breath_effect()),
    );
    particle_assets.dragon_effects.insert(
        "presence".to_string(),
        effects.add(create_dragon_presence_effect()),
    );
    
    // Create forge effects
    particle_assets.forge_effects.insert(
        "light".to_string(),
        effects.add(create_forge_light_effect()),
    );
    particle_assets.forge_effects.insert(
        "dark".to_string(),
        effects.add(create_forge_dark_effect()),
    );
}

/// Create corruption particle effect
fn create_corruption_effect(gradient: Gradient<Vec4>, dread_level: u8) -> EffectAsset {
    let mut effect = EffectAsset::new(
        Spawner::rate(20.0.into())
            .with_starts_active(true)
            .with_period(1.0),
        Module::default(),
    );
    
    // Initialize particle properties
    effect = effect
        .with_name("corruption_spread")
        .init(InitPositionSphereModifier {
            center: Vec3::ZERO,
            radius: 2.0,
            dimension: ShapeDimension::Volume,
        })
        .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            speed: Value::Uniform((0.5, 2.0)),
        })
        .init(InitLifetimeModifier {
            lifetime: Value::Uniform((2.0, 4.0)),
        })
        .init(InitSizeModifier {
            size: Value::Uniform((0.1, 0.3)),
        });
    
    // Update modifiers
    effect = effect
        .update(AccelModifier {
            accel: Vec3::new(0.0, -0.5, 0.0), // Slight downward drift
        })
        .update(LinearDragModifier {
            drag: 0.5,
        })
        .update(ConformToSphereModifier {
            center: Vec3::ZERO,
            radius: 5.0,
            strength: 0.3,
        });
    
    // Render modifiers
    effect = effect
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec2::ONE, Vec2::new(2.0, 2.0)),
        })
        .render(BillboardModifier {});
    
    effect
}

/// Create dragon breath particle effect
fn create_dragon_breath_effect() -> EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.3, 0.0, 0.8)); // Orange-red
    gradient.add_key(0.3, Vec4::new(1.0, 0.5, 0.0, 0.6)); // Yellow-orange
    gradient.add_key(0.7, Vec4::new(0.5, 0.1, 0.0, 0.4)); // Dark red
    gradient.add_key(1.0, Vec4::new(0.1, 0.0, 0.0, 0.0)); // Fade out
    
    let mut effect = EffectAsset::new(
        Spawner::rate(100.0.into())
            .with_starts_active(false),
        Module::default(),
    );
    
    effect = effect
        .with_name("dragon_breath")
        .init(InitPositionCone3dModifier {
            base_position: Vec3::ZERO,
            axis: Vec3::Z,
            angle: 30.0_f32.to_radians(),
            base_radius: 0.5,
            top_radius: 3.0,
            height: 5.0,
            dimension: ShapeDimension::Volume,
        })
        .init(InitVelocityModifier {
            velocity: Vec3::new(0.0, 0.0, 10.0),
        })
        .init(InitLifetimeModifier {
            lifetime: Value::Uniform((1.0, 2.0)),
        })
        .init(InitSizeModifier {
            size: Value::Uniform((0.3, 0.6)),
        });
    
    effect = effect
        .update(AccelModifier {
            accel: Vec3::new(0.0, 2.0, 0.0), // Fire rises
        })
        .update(LinearDragModifier {
            drag: 1.0,
        })
        .update(TurbulenceModifier {
            frequency: 2.0,
            amplitude: 1.0,
            scale: 1.0,
            scroll_speed: Some(Vec3::new(0.0, 1.0, 0.0)),
        });
    
    effect = effect
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec2::new(0.5, 0.5), Vec2::new(2.0, 2.0)),
        })
        .render(BillboardModifier {});
    
    effect
}

/// Create dragon presence effect (dark mist)
fn create_dragon_presence_effect() -> EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.1, 0.0, 0.2, 0.0));
    gradient.add_key(0.2, Vec4::new(0.2, 0.0, 0.3, 0.6));
    gradient.add_key(0.8, Vec4::new(0.1, 0.0, 0.1, 0.4));
    gradient.add_key(1.0, Vec4::new(0.0, 0.0, 0.0, 0.0));
    
    let mut effect = EffectAsset::new(
        Spawner::rate(30.0.into())
            .with_starts_active(true),
        Module::default(),
    );
    
    effect = effect
        .with_name("dragon_presence")
        .init(InitPositionSphereModifier {
            center: Vec3::ZERO,
            radius: 10.0,
            dimension: ShapeDimension::Surface,
        })
        .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            speed: Value::Uniform((0.1, 0.5)),
        })
        .init(InitLifetimeModifier {
            lifetime: Value::Uniform((5.0, 10.0)),
        })
        .init(InitSizeModifier {
            size: Value::Uniform((1.0, 3.0)),
        });
    
    effect = effect
        .update(LinearDragModifier {
            drag: 0.1,
        })
        .update(ConformToSphereModifier {
            center: Vec3::ZERO,
            radius: 10.0,
            strength: 0.1,
        });
    
    effect = effect
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec2::ONE, Vec2::new(3.0, 3.0)),
        })
        .render(BillboardModifier {});
    
    effect
}

/// Create forge light effect (ethereal wisps)
fn create_forge_light_effect() -> EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.8, 0.9, 1.0, 0.0));
    gradient.add_key(0.3, Vec4::new(0.6, 0.8, 1.0, 0.8));
    gradient.add_key(0.7, Vec4::new(0.4, 0.6, 0.9, 0.5));
    gradient.add_key(1.0, Vec4::new(0.2, 0.3, 0.8, 0.0));
    
    let mut effect = EffectAsset::new(
        Spawner::rate(10.0.into())
            .with_starts_active(true),
        Module::default(),
    );
    
    effect = effect
        .with_name("forge_light")
        .init(InitPositionCircleModifier {
            center: Vec3::ZERO,
            axis: Vec3::Y,
            radius: 1.0,
            dimension: ShapeDimension::Edge,
        })
        .init(InitVelocityModifier {
            velocity: Vec3::new(0.0, 2.0, 0.0),
        })
        .init(InitLifetimeModifier {
            lifetime: Value::Uniform((3.0, 5.0)),
        })
        .init(InitSizeModifier {
            size: Value::Uniform((0.2, 0.4)),
        });
    
    effect = effect
        .update(AccelModifier {
            accel: Vec3::new(0.0, 0.5, 0.0),
        })
        .update(OrbitModifier {
            center: Vec3::ZERO,
            speed: 1.0,
            radius: 2.0,
            axis: Vec3::Y,
        });
    
    effect = effect
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec2::new(0.2, 0.2), Vec2::ONE),
        })
        .render(BillboardModifier {});
    
    effect
}

/// Create forge dark effect (blood and shadow)
fn create_forge_dark_effect() -> EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.0, 0.0, 0.0));
    gradient.add_key(0.3, Vec4::new(0.8, 0.1, 0.0, 0.9));
    gradient.add_key(0.7, Vec4::new(0.3, 0.0, 0.0, 0.6));
    gradient.add_key(1.0, Vec4::new(0.0, 0.0, 0.0, 0.0));
    
    let mut effect = EffectAsset::new(
        Spawner::burst(20.into(), 0.5.into())
            .with_starts_active(false),
        Module::default(),
    );
    
    effect = effect
        .with_name("forge_dark")
        .init(InitPositionSphereModifier {
            center: Vec3::ZERO,
            radius: 0.5,
            dimension: ShapeDimension::Volume,
        })
        .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            speed: Value::Uniform((2.0, 5.0)),
        })
        .init(InitLifetimeModifier {
            lifetime: Value::Uniform((1.0, 2.0)),
        })
        .init(InitSizeModifier {
            size: Value::Uniform((0.1, 0.3)),
        });
    
    effect = effect
        .update(AccelModifier {
            accel: Vec3::new(0.0, -5.0, 0.0), // Blood drops fall
        })
        .update(LinearDragModifier {
            drag: 0.5,
        });
    
    effect = effect
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec2::new(0.3, 0.3), Vec2::ZERO),
        })
        .render(BillboardModifier {});
    
    effect
}

/// Update particle effects based on dread level
fn update_dread_particles(
    mut effect_config: ResMut<EffectConfiguration>,
    dread_state: Res<crate::resources::DreadState>,
) {
    // Adjust global particle settings based on dread
    effect_config.dread_color_shift = match dread_state.current_level {
        0 => Color::WHITE,
        1 => Color::srgb(0.9, 0.9, 0.8),  // Slightly desaturated
        2 => Color::srgb(0.8, 0.7, 0.6),  // Warmer, more orange
        3 => Color::srgb(0.6, 0.5, 0.7),  // Purple tint
        4 => Color::srgb(0.3, 0.2, 0.4),  // Dark purple
        _ => Color::WHITE,
    };
    
    // Increase particle density with dread
    effect_config.particle_density = 1.0 + (dread_state.current_level as f32 * 0.3);
}

/// Spawn corruption particles at corrupted tiles
fn spawn_corruption_particles(
    mut commands: Commands,
    tile_query: Query<(&Transform, &crate::maps::CorruptibleTile), Changed<crate::maps::CorruptibleTile>>,
    particle_assets: Res<ParticleEffectAssets>,
    dread_state: Res<crate::resources::DreadState>,
    effect_config: Res<EffectConfiguration>,
) {
    for (transform, corruptible) in tile_query.iter() {
        if corruptible.corruption_level > 0.5 {
            if let Some(effect_handle) = particle_assets.corruption_effects.get(&dread_state.current_level) {
                // Spawn corruption particles at highly corrupted tiles
                let intensity = corruptible.corruption_level * effect_config.particle_density;
                
                commands.spawn((
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(effect_handle.clone())
                            .with_property("spawn_rate", (20.0 * intensity).into()),
                        transform: *transform,
                        ..default()
                    },
                    Name::new("CorruptionParticles"),
                ));
            }
        }
    }
}

/// Handle dragon breath particle effects
fn handle_dragon_breath_particles(
    mut commands: Commands,
    dragon_query: Query<&Transform, With<crate::components::Dragon>>,
    player_query: Query<&Transform, With<crate::components::Player>>,
    particle_assets: Res<ParticleEffectAssets>,
    buttons: Res<ButtonInput<KeyCode>>,
) {
    if buttons.just_pressed(KeyCode::KeyB) {  // Test dragon breath
        if let (Ok(dragon_transform), Ok(player_transform)) = 
            (dragon_query.get_single(), player_query.get_single()) {
            
            if let Some(breath_effect) = particle_assets.dragon_effects.get("breath") {
                let direction = (player_transform.translation - dragon_transform.translation).normalize();
                
                commands.spawn((
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(breath_effect.clone()),
                        transform: Transform::from_translation(dragon_transform.translation)
                            .looking_at(player_transform.translation, Vec3::Y),
                        ..default()
                    },
                    Name::new("DragonBreath"),
                ));
            }
        }
    }
}

/// Process companion breakdown particle effects
fn process_companion_breakdown_particles(
    mut commands: Commands,
    companion_query: Query<(&Transform, &crate::components::Companion), Changed<crate::components::Companion>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for (transform, companion) in companion_query.iter() {
        if companion.trauma >= 0.8 {
            // Create breakdown particles (tears, shaking, etc.)
            let mut gradient = Gradient::new();
            gradient.add_key(0.0, Vec4::new(0.5, 0.5, 1.0, 0.8));  // Blue tears
            gradient.add_key(1.0, Vec4::new(0.3, 0.3, 0.8, 0.0));
            
            let mut effect = EffectAsset::new(
                Spawner::rate(5.0.into()),
                Module::default(),
            );
            
            effect = effect
                .init(InitPositionSphereModifier {
                    center: Vec3::new(0.0, 1.5, 0.0),  // Head height
                    radius: 0.2,
                    dimension: ShapeDimension::Volume,
                })
                .init(InitVelocityModifier {
                    velocity: Vec3::new(0.0, -2.0, 0.0),  // Tears fall
                })
                .init(InitLifetimeModifier {
                    lifetime: Value::Single(1.0),
                })
                .init(InitSizeModifier {
                    size: Value::Single(0.05),
                });
            
            effect = effect
                .update(AccelModifier {
                    accel: Vec3::new(0.0, -9.8, 0.0),  // Gravity
                })
                .render(ColorOverLifetimeModifier { gradient })
                .render(BillboardModifier {});
            
            let effect_handle = effects.add(effect);
            
            commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect_handle),
                    transform: *transform,
                    ..default()
                },
                Name::new(format!("{}_breakdown", companion.name)),
            ));
        }
    }
}

/// Spawn environmental particle effects
fn spawn_environmental_particles(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    dread_state: Res<crate::resources::DreadState>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut spawned: Local<bool>,
) {
    if *spawned {
        return;
    }
    
    if let Ok(camera_transform) = camera_query.get_single() {
        // Create environmental fog/ash based on dread level
        if dread_state.current_level >= 2 {
            let mut gradient = Gradient::new();
            
            let (particle_count, color, size) = match dread_state.current_level {
                2 => (50.0, Vec4::new(0.7, 0.7, 0.6, 0.3), 2.0),   // Light fog
                3 => (100.0, Vec4::new(0.5, 0.4, 0.3, 0.5), 3.0),  // Dense fog
                4 => (200.0, Vec4::new(0.2, 0.1, 0.1, 0.7), 4.0),  // Ash and darkness
                _ => (30.0, Vec4::new(0.8, 0.8, 0.8, 0.2), 1.0),
            };
            
            gradient.add_key(0.0, color * 0.5);
            gradient.add_key(0.5, color);
            gradient.add_key(1.0, color * 0.3);
            
            let mut effect = EffectAsset::new(
                Spawner::rate(particle_count.into())
                    .with_starts_active(true),
                Module::default(),
            );
            
            effect = effect
                .init(InitPositionSphereModifier {
                    center: Vec3::ZERO,
                    radius: 30.0,
                    dimension: ShapeDimension::Volume,
                })
                .init(InitVelocityModifier {
                    velocity: Vec3::new(1.0, 0.0, 0.5),  // Drift
                })
                .init(InitLifetimeModifier {
                    lifetime: Value::Uniform((10.0, 20.0)),
                })
                .init(InitSizeModifier {
                    size: Value::Uniform((size * 0.8, size * 1.2)),
                });
            
            effect = effect
                .update(LinearDragModifier {
                    drag: 0.1,
                })
                .render(ColorOverLifetimeModifier { gradient })
                .render(BillboardModifier {});
            
            let effect_handle = effects.add(effect);
            
            commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect_handle),
                    transform: *camera_transform,
                    ..default()
                },
                Name::new("EnvironmentalParticles"),
            ));
            
            *spawned = true;
        }
    }
}

/// Handle particle spawn events from other systems
fn handle_particle_events(
    mut commands: Commands,
    mut events: EventReader<SpawnParticleEvent>,
    particle_assets: Res<ParticleEffectAssets>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for event in events.read() {
        let effect_handle = match &event.effect_type {
            ParticleEffectType::Corruption => {
                particle_assets.corruption_effects.get(&0).cloned()
            }
            ParticleEffectType::DragonBreath => {
                particle_assets.dragon_effects.get("breath").cloned()
            }
            ParticleEffectType::DragonPresence => {
                particle_assets.dragon_effects.get("presence").cloned()
            }
            ParticleEffectType::ForgeLight => {
                particle_assets.forge_effects.get("light").cloned()
            }
            ParticleEffectType::ForgeDark => {
                particle_assets.forge_effects.get("dark").cloned()
            }
            _ => None,
        };
        
        if let Some(handle) = effect_handle {
            let mut particle_bundle = ParticleEffectBundle {
                effect: ParticleEffect::new(handle)
                    .with_property("spawn_rate", (20.0 * event.intensity).into()),
                transform: Transform::from_translation(event.position),
                ..default()
            };
            
            let entity = commands.spawn((
                particle_bundle,
                Name::new(format!("{:?}_effect", event.effect_type)),
            )).id();
            
            // Add lifetime component if duration specified
            if let Some(duration) = event.duration {
                commands.entity(entity).insert(EffectLifetime {
                    remaining: duration,
                });
            }
        }
    }
}

/// Component for effects with limited lifetime
#[derive(Component)]
pub struct EffectLifetime {
    pub remaining: f32,
}

/// System to clean up expired effects
pub fn cleanup_expired_effects(
    mut commands: Commands,
    mut query: Query<(Entity, &mut EffectLifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.remaining -= time.delta_secs();
        if lifetime.remaining <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
