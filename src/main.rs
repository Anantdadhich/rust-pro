use bevy::prelude::*;
pub mod player;
pub mod alien;
pub mod resolution;
pub mod projectile;
pub mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
         .set(WindowPlugin  {
            primary_window: Some (Window  {
                title:String::from("Space invaders"),
                position:WindowPosition::Centered(MonitorSelection::Primary),
                 resolution:Vec2::new(512.,512.).into(),
                 ..Default::default()
                             }),
                   ..Default::default()           
         })
         .set(ImagePlugin::default_nearest()),

         game::GamePlugin,
        )
       
        .run();
}
