use bevy::prelude::*;

pub struct HelloPlugin;

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Player>>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("List of Spawned in Players:");
        for name in query.iter() {
            println!("{}", name.0);
        }
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Name("Henry S Hunt".to_string()));
}

fn setup_env(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/ElonScam.png"),
        ..default()
    });
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_startup_system(setup_env)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);
