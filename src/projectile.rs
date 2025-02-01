use std::str;

use bevy::ecs::entity;
use bevy::prelude::*;
use bevy::transform::commands;

use crate::resolution;

use crate::alien;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_projectile, update_alien_interactions))
    }
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
}

fn update_projectile(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
) {
    for (entity, projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.y += projectile.speed * time.delta_seconds();

        if transform.translation.y > resolution.screen_dimension.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}
