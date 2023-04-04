use bevy::app::App;
use bevy::prelude::*;

const BASE_SPEED : f32 = 500.;
const TIME_STEP : f32 = 1. / 60.;
const PLAYER_SPRITE: &str = "player.png";
const PLAYER_SIZE: (f32,f32) = (32., 32.);
const SPRITE_SCALE: f32 = 5.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(keyboard_event)
        .add_system(move_player)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bottom = -500.;
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
        .insert(Velocity { x: 0., y: 0. });
}

fn move_player(mut commands: Commands, mut query: Query<(Entity, &Velocity, &mut Transform, /* &Movable */)>) {
    for (entity, velocity, mut transform, /* movable */) in query.iter_mut() {
		let translation = &mut transform.translation;
		translation.x += velocity.x * TIME_STEP * BASE_SPEED;
		translation.y += velocity.y * TIME_STEP * BASE_SPEED;

		// if movable.auto_despawn {
		// 	// despawn when out of screen
		// 	const MARGIN: f32 = 200.;
		// 	if translation.y > win_size.h / 2. + MARGIN
		// 		|| translation.y < -win_size.h / 2. - MARGIN
		// 		|| translation.x > win_size.w / 2. + MARGIN
		// 		|| translation.x < -win_size.w / 2. - MARGIN
		// 	{
		// 		commands.entity(entity).despawn();
		// 	}
		// }
	}
}

fn keyboard_event(kb: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>){
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) { -1. }
                    else if kb.pressed(KeyCode::Right) { 1. }
                    else { 
                0.
            }

        }
}

