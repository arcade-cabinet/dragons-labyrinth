//! Physics system for Dragon's Labyrinth using Avian

use bevy::prelude::*;
use avian3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Avian physics with default settings
            .add_plugins(PhysicsPlugins::default())
            
            // Optional debug rendering
            .add_plugins(PhysicsDebugPlugin::default())
            
            .init_resource::<PhysicsConfig>()
            .add_systems(Startup, setup_physics)
            .add_systems(Update, (
                handle_collisions,
                update_physics_bodies,
                apply_forces,
            ))
            .add_systems(PostProcessCollisions, process_collision_events);
    }
}

#[derive(Resource)]
pub struct PhysicsConfig {
    pub gravity: Vec3,
    pub substeps: u32,
    pub debug_render: bool,
    pub air_resistance: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            substeps: 4,
            debug_render: false,
            air_resistance: 0.01,
        }
    }
}

/// Custom physics body component with game-specific properties
#[derive(Component)]
pub struct GamePhysicsBody {
    pub mass_override: Option<f32>,
    pub is_kinematic: bool,
    pub affected_by_gravity: bool,
}

/// Collision layers for different entity types
#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player,
    Enemy,
    Environment,
    Projectile,
    Trigger,
    Item,
}

/// Force application component
#[derive(Component, Default)]
pub struct ForceAccumulator {
    pub force: Vec3,
    pub torque: Vec3,
    pub impulse: Vec3,
}

fn setup_physics(
    mut gravity: ResMut<Gravity>,
    physics_config: Res<PhysicsConfig>,
) {
    gravity.0 = physics_config.gravity;
}

fn handle_collisions(
    mut collision_events: EventReader<Collision>,
    query: Query<(Entity, &Name), With<RigidBody>>,
) {
    for Collision(contacts) in collision_events.read() {
        let entity_a = contacts.entity1;
        let entity_b = contacts.entity2;
        
        // Handle collision logic here
        if let (Ok((_, name_a)), Ok((_, name_b))) = (
            query.get(entity_a),
            query.get(entity_b),
        ) {
            debug!("Collision between {} and {}", name_a, name_b);
        }
    }
}

fn update_physics_bodies(
    mut bodies: Query<(
        &mut LinearVelocity,
        &mut AngularVelocity,
        &GamePhysicsBody,
    )>,
    physics_config: Res<PhysicsConfig>,
    time: Res<Time>,
) {
    for (mut linear_vel, mut angular_vel, body) in bodies.iter_mut() {
        // Apply air resistance
        linear_vel.0 *= 1.0 - (physics_config.air_resistance * time.delta_seconds());
        angular_vel.0 *= 1.0 - (physics_config.air_resistance * time.delta_seconds());
        
        // Custom physics logic based on body properties
        if !body.affected_by_gravity {
            // Counter gravity for this body
            linear_vel.0.y += 9.81 * time.delta_seconds();
        }
    }
}

fn apply_forces(
    mut bodies: Query<(
        &mut ExternalForce,
        &mut ExternalTorque,
        &mut ExternalImpulse,
        &mut ForceAccumulator,
    )>,
) {
    for (mut force, mut torque, mut impulse, mut accumulator) in bodies.iter_mut() {
        // Apply accumulated forces
        if accumulator.force != Vec3::ZERO {
            force.apply_force(accumulator.force);
            accumulator.force = Vec3::ZERO;
        }
        
        if accumulator.torque != Vec3::ZERO {
            torque.apply_torque(accumulator.torque);
            accumulator.torque = Vec3::ZERO;
        }
        
        if accumulator.impulse != Vec3::ZERO {
            impulse.apply_impulse(accumulator.impulse);
            accumulator.impulse = Vec3::ZERO;
        }
    }
}

fn process_collision_events(
    mut collision_events: EventReader<CollisionStarted>,
    mut collision_ended: EventReader<CollisionEnded>,
) {
    for CollisionStarted(entity_a, entity_b) in collision_events.read() {
        // Process collision start
        debug!("Collision started between {:?} and {:?}", entity_a, entity_b);
    }
    
    for CollisionEnded(entity_a, entity_b) in collision_ended.read() {
        // Process collision end
        debug!("Collision ended between {:?} and {:?}", entity_a, entity_b);
    }
}

/// Helper function to create a physics body bundle
pub fn create_physics_body(
    mass: f32,
    collider: Collider,
    restitution: f32,
    friction: f32,
) -> impl Bundle {
    (
        RigidBody::Dynamic,
        collider,
        Mass(mass),
        Restitution::new(restitution),
        Friction::new(friction),
        LinearVelocity::default(),
        AngularVelocity::default(),
        ExternalForce::default(),
        ExternalTorque::default(),
        ExternalImpulse::default(),
        ForceAccumulator::default(),
    )
}