use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        // .add_system(hello_world_system)
        // .add_system(greed_people)
        // .add_startup_system(add_people)
        .add_startup_system(setup)
        .add_system(change_clear_color)
        .run();
}

// fn hello_world_system() {
//     println!("Hello, world!");
// }

// fn greed_people(query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("Hello {}!", name.0);
//     }
// }

// fn add_people(mut commands: Commands) {
//     commands.spawn((Person, Name("Elaina Proctor".to_string())));
//     commands.spawn((Person, Name("Renzo Hume".to_string())));
//     commands.spawn((Person, Name("Zayna Nieves".to_string())));
// }

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::J) {
        clear_color.0 = Color::PURPLE;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// #[derive(Component)]
// struct Person;

// #[derive(Component)]
// struct Name(String);
