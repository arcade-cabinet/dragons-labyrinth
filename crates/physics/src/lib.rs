//! Physics system for Dragon's Labyrinth

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .init_resource::<PhysicsConfig>()
            .add_systems(Startup, setup_physics)
            .add_systems(Update, (
                handle_collisions,
                update_physics_bodies,
            ));
    }
}

#[derive(Resource)]
pub struct PhysicsConfig {
    pub gravity: Vec3,
    pub simulation_rate: f32,
    pub debug_render: bool,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            simulation_rate: 60.0,
            debug_render: false,
        }
    }
}

#[derive(Component)]
pub struct PhysicsBody {
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32,
}

#[derive(Component)]
pub struct CollisionGroups {
    pub membership: u32,
    pub filter: u32,
}

fn setup_physics(
    mut rapier_config: ResMut<RapierConfiguration>,
    physics_config: Res<PhysicsConfig>,
) {
    rapier_config.gravity = physics_config.gravity;
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                // Handle collision start
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                // Handle collision end
            }
        }
    }
}

fn update_physics_bodies(
    mut bodies: Query<&mut Velocity, With<PhysicsBody>>,
) {
    // Physics body updates
}