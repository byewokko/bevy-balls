use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_svg::prelude::*;

pub fn _spawn_svg(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Svg2dBundle {
        svg: asset_server.load("svg/shape.svg"),
        // transform: Transform::from_scale(Vec3::splat(0.01)),
        transform: Transform::from_xyz(
            window.width() / 2.0, 
            window.height() / 2.0, 
            0.0
        ),
        ..default()
    });
}

pub fn _enemy_svg_sprite(
	asset_server: Res<AssetServer>,
	position: Vec3,
) -> Svg2dBundle {
	Svg2dBundle {
		svg: asset_server.load("svg/kibana.svg"),
		transform: Transform::from_translation(position).with_scale(Vec3::splat(0.04)),
		origin: Origin::Center,
		..default()
	}
}

pub fn _player_svg_sprite(
	asset_server: Res<AssetServer>,
) -> Svg2dBundle {
	Svg2dBundle {
		svg: asset_server.load("svg/shape.svg"),
		// transform: Transform::
		// 	from_scale(Vec3::splat(0.64))
		// 	.with_translation(Vec3::new(-16.0,-16.0,0.0)),
		// origin: Origin::Center,
		..default()
	}
}
