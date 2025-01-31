use crate::resolution;
use bevy::prelude::*;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens)
            .add_systems(Update, (update_aliens, manage_alien_logic))
    }
}

#[derive(Component)]
pub struct Alien {
    pub dead: bool,              //track if alien is dead
    pub original_position: Vec3, //reset
}

#[derive(Component)]
pub struct Dead; //filter out the destroyed aliens

#[derive(Resource)]
pub struct AlienManger {
    pub direction: f32,

    pub shift_aliens_down: bool, //for vertical movement

    pub dist_from_boundary: f32, //how far aliens went

    pub reset: bool,
}

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.;
const SPEED: f32 = 100.0;
const ALIEN_SHIFT_AMOUNT: f32 = 32.;

fn setup_aliens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    commands.insert_resource(AlienManager {
        reset: false,
        dist_from_boundary: 0.,
        shift_aliens_down: false,
        direction: 1.,
    });
}
