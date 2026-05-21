use bevy::prelude::*;
use bevy_seedling::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SeedlingPlugins))
        .add_systems(Startup, startup)
        .run();
}

fn startup(server: Res<AssetServer>, mut commands: Commands) {
    // Spawning a `SamplePlayer` component will play a sample
    // once as soon as it's loaded. If no pool is specified
    // and no effects are applied, the sample will be played in
    // the `DefaultPool`.
    commands.spawn(SamplePlayer::new(server.load("caw.ogg")));
}
