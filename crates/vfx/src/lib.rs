//! Visual effects system for Dragon's Labyrinth

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct VFXPlugin;

impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .init_resource::<VFXConfig>()
            .init_resource::<ParticleEffects>()
            .add_systems(Startup, setup_vfx)
            .add_systems(Update, (
                update_particle_systems,
                spawn_effect_requests,
                cleanup_expired_effects,
            ));
    }
}

#[derive(Resource)]
pub struct VFXConfig {
    pub max_particles: u32,
    pub quality_level: QualityLevel,
    pub enable_bloom: bool,
    pub enable_distortion: bool,
}

impl Default for VFXConfig {
    fn default() -> Self {
        Self {
            max_particles: 10000,
            quality_level: QualityLevel::High,
            enable_bloom: true,
            enable_distortion: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum QualityLevel {
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Resource, Default)]
pub struct ParticleEffects {
    pub fire: Option<Handle<EffectAsset>>,
    pub smoke: Option<Handle<EffectAsset>>,
    pub explosion: Option<Handle<EffectAsset>>,
    pub magic: Option<Handle<EffectAsset>>,
    pub blood: Option<Handle<EffectAsset>>,
    pub dust: Option<Handle<EffectAsset>>,
    pub sparks: Option<Handle<EffectAsset>>,
}

#[derive(Component)]
pub struct EffectSpawner {
    pub effect_type: EffectType,
    pub spawn_rate: f32,
    pub lifetime: f32,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub enum EffectType {
    Fire,
    Smoke,
    Explosion,
    Magic(MagicType),
    Blood,
    Dust,
    Sparks,
    Custom(String),
}

#[derive(Clone, Debug)]
pub enum MagicType {
    Heal,
    Damage,
    Shield,
    Teleport,
    Summon,
}

#[derive(Event)]
pub struct SpawnEffectEvent {
    pub effect_type: EffectType,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: f32,
    pub duration: Option<f32>,
}

fn setup_vfx(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<ParticleEffects>,
) {
    // Create fire effect
    let mut fire = EffectAsset::new(
        Spawner::rate(30.0.into())
            .with_starts_active(false),
        Module::default(),
    );
    particle_effects.fire = Some(effects.add(fire));
    
    // Additional effects would be created here
}

fn update_particle_systems(
    mut spawners: Query<(&mut EffectSpawner, &mut ParticleEffect)>,
    time: Res<Time>,
) {
    for (mut spawner, mut effect) in spawners.iter_mut() {
        if spawner.active {
            spawner.lifetime -= time.delta_seconds();
            if spawner.lifetime <= 0.0 {
                spawner.active = false;
            }
        }
    }
}

fn spawn_effect_requests(
    mut commands: Commands,
    mut events: EventReader<SpawnEffectEvent>,
    particle_effects: Res<ParticleEffects>,
) {
    for event in events.read() {
        let effect_handle = match &event.effect_type {
            EffectType::Fire => particle_effects.fire.clone(),
            EffectType::Smoke => particle_effects.smoke.clone(),
            EffectType::Explosion => particle_effects.explosion.clone(),
            _ => None,
        };
        
        if let Some(handle) = effect_handle {
            commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(handle),
                    transform: Transform::from_translation(event.position)
                        .with_rotation(event.rotation)
                        .with_scale(Vec3::splat(event.scale)),
                    ..default()
                },
                EffectSpawner {
                    effect_type: event.effect_type.clone(),
                    spawn_rate: 30.0,
                    lifetime: event.duration.unwrap_or(5.0),
                    active: true,
                },
            ));
        }
    }
}

fn cleanup_expired_effects(
    mut commands: Commands,
    query: Query<(Entity, &EffectSpawner)>,
) {
    for (entity, spawner) in query.iter() {
        if !spawner.active && spawner.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}