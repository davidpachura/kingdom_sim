use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Napoleon Bonaparte".to_string())));
    commands.spawn((Person, Name("Nicholas Romanov".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);


fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Nicholas Romanov" {
            name.0 = "Vladimir Lenin".to_string();
            break;
        }
    }
}