use bevy::prelude::*;

// #[derive(Component, Debug)]
// struct Position {
//     x: f32,
//     y: f32,
// }

// #[derive(Component, Debug)]
// struct Velocity {
//     x: f32,
//     y: f32,
// }

pub struct Velocity {
    pub value: Vec3,
}

pub struct SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship)
    }
}

fn main() {
    App::new()
        .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run();
    println!("Hello, world!");
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        Velocity {
            value: Vec3::new(0., 0., 0.),
        },
    ));
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        position.translation.x += velocity.value.x;
        position.translation.y += velocity.value.y;
        position.translation.z += velocity.value.z
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?}",
            entity, transform.translation
        );
    }
}
