use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


const BASE_SPEED : f32 = 500.;
const TIME_STEP : f32 = 1. / 60.;
const PLAYER_SPRITE: &str = "player.png";
const PLAYER_SIZE: (f32,f32) = (16., 32.);
const SPRITE_SCALE: f32 = 5.0;
const PIXELS_PER_METER: f32 = 20.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))

        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(setup_ground)

        .add_system(keyboard_event)
        .add_system(move_player)

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
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
            .insert(Collider::cuboid(500.0, 50.0));
    
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bottom = 50.;
    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.,),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..default()
        },
        ..default()
    })
        .insert(Player)
        .insert(Velocity  { linvel: Vec2::new(1.0, 2.0),
                            angvel: 0.4,})
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(PLAYER_SIZE.0 / 2., PLAYER_SIZE.1 / 2.))
        .insert(GravityScale(500.0));
}


fn move_player(mut query: Query<(Entity, &Velocity, &mut Transform)>) {
    for (_entity, velocity, mut transform, /* movable */) in query.iter_mut() {
		let translation = &mut transform.translation;
		translation.x += velocity.linvel.x * TIME_STEP * BASE_SPEED;
		translation.y += velocity.linvel.y * TIME_STEP * BASE_SPEED;
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

