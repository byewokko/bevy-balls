mod svg_test;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use std::f32::consts::TAU;
use bevy_svg::prelude::*;
use svg_test::_player_svg_sprite;


pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_RADIUS: f32 = 32.0;
pub const NUMBER_OF_ENEMIES: i8 = 7;
pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_RADIUS: f32 = 32.0;
pub const BOUNCE_RANDOMNESS: f32 = 0.05;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SvgPlugin)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, animate_player)
        .add_systems(Update, move_player)
        .add_systems(Update, move_enemy)
        .add_systems(Update, confine_player)
        .add_systems(Update, confine_enemy)
        .run();
}

#[derive(Component)]
pub struct Player {

}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

pub fn random_direction() -> Vec3 {
    let angle = random::<f32>() * TAU;
    Vec3::new(angle.cos(), angle.sin(), 0.0)
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn((
            // Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)),
            // GlobalTransform::default(),
            // InheritedVisibility::default(),
            Player {},
            SpriteBundle {
                texture: asset_server.load("sprites/ball_blue_small.png"),
                transform: Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Spawn the SVG as a child of the parent
            parent.spawn((
                _player_svg_sprite(asset_server),
            ));
        });
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * window.width(), 
                    random::<f32>() * window.height(), 
                    0.0
                ),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: random_direction(),
            },
        ));
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }

}

pub fn animate_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.rotate_z(-3.0 * time.delta_seconds());
    }

}

pub fn move_enemy(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
    }

}

pub fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        if transform.translation.x < PLAYER_RADIUS {
            transform.translation.x = PLAYER_RADIUS;
        } else if transform.translation.x > window.width() - PLAYER_RADIUS {
            transform.translation.x = window.width() - PLAYER_RADIUS;
        }

        if transform.translation.y < PLAYER_RADIUS {
            transform.translation.y = PLAYER_RADIUS;
        } else if transform.translation.y > window.height() - PLAYER_RADIUS {
            transform.translation.y = window.height() - PLAYER_RADIUS;
        }
        
    }

}

pub fn confine_enemy(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = ENEMY_RADIUS;
    let x_max = window.width() - ENEMY_RADIUS;
    let y_min = ENEMY_RADIUS;
    let y_max = window.height() - ENEMY_RADIUS;
    
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut bounced = false;
        let mut new_direction = enemy.direction;

        if transform.translation.x < x_min {
            transform.translation.x = x_min;
            new_direction.x = -new_direction.x;
            bounced = true;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
            new_direction.x = -new_direction.x;
            bounced = true;
        }

        if transform.translation.y < y_min {
            transform.translation.y = y_min;
            new_direction.y = -new_direction.y;
            bounced = true;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
            new_direction.y = -new_direction.y;
            bounced = true;
        }

        if bounced {
            enemy.direction = (new_direction + BOUNCE_RANDOMNESS * random_direction()).normalize();
            play_random_bonk(&mut commands, &asset_server);
        }
        
    }
    

}

fn play_random_bonk(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Define a list of sound file paths
    let sound_files = vec![
        "audio/bump.ogg",
        "audio/bonk.ogg",
    ];

    // Select a random sound file
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..sound_files.len());
    let selected_file = sound_files[random_index];

    // Slightly randomize playback speed
    let normal = Normal::new(1.0, 0.1).unwrap();
    let playback_speed: f32 = normal.sample(&mut rng);

    // Load the selected file and play it
    commands.spawn(AudioBundle {
        source: asset_server.load(selected_file),
        settings: PlaybackSettings::DESPAWN.with_speed(playback_speed),
        ..default()
    });
}
