use bevy::prelude::*;

use crate::projectile;
use crate::resolution;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
    }
}

#[derive(Component)]
struct Player {
    pub shoot_timer: f32,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    let player_image = asset_server.load("player.png");
    commands.spawn((
        SpriteBundle {
            texture: player_image,
            transform: Transform::from_xyz(
                0.,
                -(resolution.screen_dimension.y * 0.5) + (resolution.pixel_ratio * 5.0),
                0.,
            )
            .with_scale(Vec3::splat(resolution.pixel_ratio)),
            ..Default::default()
        },
        Player { shoot_timer: 0. },
    ));
}
const SPEED: f32 = 200.;
const BULLET: f32 = 400.;
const SHOOT_COOL: f32 = 0.5;

fn update_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<resolution::Resolution>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let mut horizontal = 0.;

    if keys.pressed(KeyCode::KeyA) {
        horizontal += -1.;
    }

    if keys.pressed(KeyCode::KeyD) {
        horizontal += 1.;
    }

    transform.translation.x += horizontal * time.delta_seconds() * SPEED;

    let leftbound = -resolution.screen_dimension.x * 0.5;

    let rightbound = resolution.screen_dimension.x * 0.5;

    if transform.translation.x < leftbound {
        transform.translation.x = leftbound
    }

    if transform.translation.x > rightbound {
        transform.translation.x = rightbound
    }

    player.shoot_timer -= time.delta_seconds();

    if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0. {
        player.shoot_timer = SHOOT_COOL;

        let bullet_text = asset_server.load("bullet.png");
        commands.spawn((
            SpriteBundle {
                texture: bullet_text,
                transform: Transform::from_translation(transform.translation)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                ..Default::default()
            },
            projectile::Projectile { speed: BULLET },
        ));
    }
}
