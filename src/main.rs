use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


const BASE_SPEED : f32 = 5.;
const TIME_STEP : f32 = 1. / 60.;
const PLAYER_SPRITE: &str = "player.png";
const SPRITE_SCALE: f32 = 1.0;
const BLOCK_SIZE: f32 = 16.;
const PLAYER_SIZE: (f32,f32) = (BLOCK_SIZE, BLOCK_SIZE * 2.);


fn main() {
    let primary_window = Window {
        title: "Window Name".to_string(),
        resolution: (1280.0, 720.0).into(),
        resizable: false,
        ..Default::default()
    };
    
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                }))
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(BLOCK_SIZE))

        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(setup_ground)

        .add_system(display_events)
        .add_system(apply_forces)

        .run();
}

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct SpriteSize(pub Vec2);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_ground(mut commands: Commands) {
    commands.spawn(RigidBody::Fixed)
            .insert(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)))
            .insert(Collider::cuboid(10.0 * BLOCK_SIZE, 1.0 * BLOCK_SIZE));
    
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., 350., 0.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })

        .insert(Player)
        .insert(Velocity  { linvel: Vec2::new(0.0, 0.0),
                            angvel: 0.0,})
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(PLAYER_SIZE.0 / 2., PLAYER_SIZE.1 / 2.))
        .insert(GravityScale(1.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0, })
        .insert(ExternalImpulse {
                impulse: Vec2::new(0.0, 0.0),
                torque_impulse: 0.0, });
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event)
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
   

fn apply_forces(kb: Res<Input<KeyCode>>, 
                    mut ext_forces: Query<&mut ExternalForce, With<Player>>,
                    mut ext_impulses: Query<&mut ExternalImpulse, With<Player>>) {

    for mut ext_impulse in ext_impulses.iter_mut() {
        ext_impulse.impulse =
                        if kb.pressed(KeyCode::Up) { Vec2::new(0.0, 14.0) }
                        else { Vec2::new(0.0, 0.0) };
                        // else kb.pressed(KeyCode::Down) { Vec2::new(0.0, 0.0) };
        }

    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force =
                             if kb.pressed(KeyCode::Left) { Vec2::new(-200.0, 0.0)  }
                        else if kb.pressed(KeyCode::Right) { Vec2::new(200.0, 0.0) }
                        else { Vec2::new(0.0, 0.0) };
        }
}


fn keyboard_event(kb: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>){
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.linvel.x = if kb.pressed(KeyCode::Left) { -1. }
                    else if kb.pressed(KeyCode::Right) { 1. }
                    else { 
                0.
            };

        velocity.linvel.y = if kb.pressed(KeyCode::Down) { -1. }
                    else if kb.pressed(KeyCode::Up) { 1. }
                    else { 
                0.
            }

        }
}

