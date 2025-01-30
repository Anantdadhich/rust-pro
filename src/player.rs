use bevy::prelude::*;
use create::resolution;
use create::projectile;
 


impl Plugin for PlayerPlugin {
    fn build(&self, app:&mut App){
        app.add_systems()
    }
}



#[derive(Component)]
struct Player{
    pub shoot_timer :f32,
}

fn setup_palyer{
    
}

