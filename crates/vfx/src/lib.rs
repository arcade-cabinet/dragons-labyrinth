//! Visual effects system for Dragon's Labyrinth

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use motiongfx_bevy::prelude::*;

pub struct VFXPlugin;

impl Plugin for VFXPlugin {
    fn build(&self, app: &mut App) {
        app
            // Particle systems
            .add_plugins(HanabiPlugin)
            
            // Motion graphics
            .add_plugins(MotionGfxPlugin)
            
            .init_resource::<VFXConfig>()
            .init_resource::<ParticleEffects>()
            .init_resource::<MotionEffects>()
            .add_systems(Startup, setup_vfx)
            .add_systems(Update, (
                update_particle_systems,
                update_motion_graphics,
                spawn_effect_requests,
                cleanup_expired_effects,
                animate_ui_elements,
            ));
    }
}

#[derive(Resource)]
pub struct VFXConfig {
    pub max_particles: u32,
    pub quality_level: QualityLevel,
    pub enable_bloom: bool,
    pub enable_distortion: bool,
    pub enable_motion_blur: bool,
}

impl Default for VFXConfig {
    fn default() -> Self {
        Self {
            max_particles: 10000,
            quality_level: QualityLevel::High,
            enable_bloom: true,
            enable_distortion: true,
            enable_motion_blur: true,
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

#[derive(Resource, Default)]
pub struct MotionEffects {
    pub screen_shake: Option<Entity>,
    pub camera_zoom: Option<Entity>,
    pub ui_transitions: Vec<Entity>,
    pub object_animations: Vec<Entity>,
}

#[derive(Component)]
pub struct EffectSpawner {
    pub effect_type: EffectType,
    pub spawn_rate: f32,
    pub lifetime: f32,
    pub active: bool,
}

#[derive(Component)]
pub struct MotionAnimation {
    pub animation_type: AnimationType,
    pub duration: f32,
    pub elapsed: f32,
    pub easing: EasingFunction,
    pub loop_mode: LoopMode,
}

#[derive(Clone, Debug)]
pub enum AnimationType {
    Scale(Vec3, Vec3),
    Rotate(Quat, Quat),
    Translate(Vec3, Vec3),
    Color(Color, Color),
    Fade(f32, f32),
    Shake(f32),
    Pulse(f32),
    Custom(String),
}

#[derive(Clone, Debug)]
pub enum EasingFunction {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInElastic,
    EaseOutElastic,
    Bounce,
}

#[derive(Clone, Debug)]
pub enum LoopMode {
    Once,
    Loop,
    PingPong,
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
    ScreenEffect(ScreenEffectType),
    Custom(String),
}

#[derive(Clone, Debug)]
pub enum MagicType {
    Heal,
    Damage,
    Shield,
    Teleport,
    Summon,
    Curse,
    Blessing,
}

#[derive(Clone, Debug)]
pub enum ScreenEffectType {
    Shake,
    Flash,
    Fade,
    ChromaticAberration,
    Vignette,
    Blur,
}

#[derive(Event)]
pub struct SpawnEffectEvent {
    pub effect_type: EffectType,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: f32,
    pub duration: Option<f32>,
    pub intensity: f32,
}

#[derive(Event)]
pub struct StartMotionEvent {
    pub target: Entity,
    pub animation: AnimationType,
    pub duration: f32,
    pub easing: EasingFunction,
    pub loop_mode: LoopMode,
}

fn setup_vfx(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_effects: ResMut<ParticleEffects>,
) {
    // Create fire effect with Hanabi
    let mut fire = EffectAsset::new(
        Spawner::rate(30.0.into())
            .with_starts_active(false),
        Module::default(),
    );
    particle_effects.fire = Some(effects.add(fire));
    
    // Create smoke effect
    let mut smoke = EffectAsset::new(
        Spawner::rate(20.0.into())
            .with_starts_active(false),
        Module::default(),
    );
    particle_effects.smoke = Some(effects.add(smoke));
    
    // Additional particle effects would be created here
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

fn update_motion_graphics(
    mut animations: Query<(Entity, &mut Transform, &mut MotionAnimation)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut animation) in animations.iter_mut() {
        animation.elapsed += time.delta_seconds();
        
        let progress = (animation.elapsed / animation.duration).min(1.0);
        let eased_progress = apply_easing(progress, &animation.easing);
        
        match &animation.animation_type {
            AnimationType::Scale(start, end) => {
                transform.scale = start.lerp(*end, eased_progress);
            }
            AnimationType::Rotate(start, end) => {
                transform.rotation = start.slerp(*end, eased_progress);
            }
            AnimationType::Translate(start, end) => {
                transform.translation = start.lerp(*end, eased_progress);
            }
            AnimationType::Shake(intensity) => {
                let shake = Vec3::new(
                    (animation.elapsed * 50.0).sin() * intensity,
                    (animation.elapsed * 47.0).cos() * intensity,
                    0.0,
                );
                transform.translation += shake;
            }
            AnimationType::Pulse(scale) => {
                let pulse = 1.0 + (animation.elapsed * 2.0 * std::f32::consts::PI).sin() * scale;
                transform.scale = Vec3::splat(pulse);
            }
            _ => {}
        }
        
        // Handle loop modes
        if animation.elapsed >= animation.duration {
            match animation.loop_mode {
                LoopMode::Once => {
                    commands.entity(entity).remove::<MotionAnimation>();
                }
                LoopMode::Loop => {
                    animation.elapsed = 0.0;
                }
                LoopMode::PingPong => {
                    // Reverse the animation
                    animation.elapsed = 0.0;
                    match &mut animation.animation_type {
                        AnimationType::Scale(start, end) => {
                            std::mem::swap(start, end);
                        }
                        AnimationType::Rotate(start, end) => {
                            std::mem::swap(start, end);
                        }
                        AnimationType::Translate(start, end) => {
                            std::mem::swap(start, end);
                        }
                        _ => {}
                    }
                }
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
        match &event.effect_type {
            EffectType::Fire | EffectType::Smoke | EffectType::Explosion => {
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
            EffectType::ScreenEffect(screen_type) => {
                handle_screen_effect(screen_type, event.intensity, event.duration.unwrap_or(1.0));
            }
            _ => {}
        }
    }
}

fn animate_ui_elements(
    mut motion_events: EventReader<StartMotionEvent>,
    mut commands: Commands,
) {
    for event in motion_events.read() {
        commands.entity(event.target).insert(MotionAnimation {
            animation_type: event.animation.clone(),
            duration: event.duration,
            elapsed: 0.0,
            easing: event.easing.clone(),
            loop_mode: event.loop_mode.clone(),
        });
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

fn apply_easing(t: f32, easing: &EasingFunction) -> f32 {
    match easing {
        EasingFunction::Linear => t,
        EasingFunction::EaseInQuad => t * t,
        EasingFunction::EaseOutQuad => t * (2.0 - t),
        EasingFunction::EaseInOutQuad => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                -1.0 + (4.0 - 2.0 * t) * t
            }
        }
        EasingFunction::EaseInCubic => t * t * t,
        EasingFunction::EaseOutCubic => {
            let t = t - 1.0;
            t * t * t + 1.0
        }
        EasingFunction::EaseInOutCubic => {
            if t < 0.5 {
                4.0 * t * t * t
            } else {
                let t = 2.0 * t - 2.0;
                1.0 + t * t * t / 2.0
            }
        }
        EasingFunction::EaseInElastic => {
            (10.0 * t - 10.75).sin() * (2.0_f32).powf(-10.0 * (1.0 - t))
        }
        EasingFunction::EaseOutElastic => {
            (10.0 * t - 0.75).sin() * (2.0_f32).powf(-10.0 * t) + 1.0
        }
        EasingFunction::Bounce => {
            if t < 0.5 {
                0.5 - 0.5 * (1.0 - 2.0 * t).powi(2)
            } else {
                0.5 + 0.5 * (2.0 * t - 1.0).powi(2)
            }
        }
    }
}

fn handle_screen_effect(effect_type: &ScreenEffectType, intensity: f32, duration: f32) {
    // Screen effect implementation would go here
    match effect_type {
        ScreenEffectType::Shake => {
            // Implement screen shake
        }
        ScreenEffectType::Flash => {
            // Implement screen flash
        }
        _ => {}
    }
}

/// Helper function to create a particle burst
pub fn create_particle_burst(
    position: Vec3,
    particle_count: u32,
    spread: f32,
    lifetime: f32,
) -> impl Bundle {
    // Particle burst bundle creation
    ()
}

/// Helper function for UI element animations
pub fn animate_ui_appear(entity: Entity, commands: &mut Commands) {
    commands.entity(entity).insert(MotionAnimation {
        animation_type: AnimationType::Scale(Vec3::ZERO, Vec3::ONE),
        duration: 0.3,
        elapsed: 0.0,
        easing: EasingFunction::EaseOutCubic,
        loop_mode: LoopMode::Once,
    });
}

pub fn animate_ui_disappear(entity: Entity, commands: &mut Commands) {
    commands.entity(entity).insert(MotionAnimation {
        animation_type: AnimationType::Scale(Vec3::ONE, Vec3::ZERO),
        duration: 0.2,
        elapsed: 0.0,
        easing: EasingFunction::EaseInCubic,
        loop_mode: LoopMode::Once,
    });
}