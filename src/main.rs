use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, WindowMode},
};
use rand::prelude::*;
#[derive(Component, PartialEq, Debug)]
struct Circle {
    radius: f32,
    velocity: Vec2,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                title: "gaming".into(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (update, close_on_esc, collide))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    let colors = vec![
        Color::RED,
        Color::ORANGE,
        Color::GREEN,
        Color::PURPLE,
        Color::PINK,
        Color::BLUE,
        Color::LIME_GREEN,
        Color::CRIMSON,
        Color::SILVER,
        Color::GOLD,
    ];
    let mut rng = rand::thread_rng();
    for i in 0..25 {
        let radius = rng.gen::<f32>() * 25. + 10.;
        let transform = Transform::from_translation(Vec3::new(
            rng.gen_range(-200.0..200.0),
            rng.gen_range(-200.0..200.0),
            0.,
        ));
        commands.spawn((
            Circle {
                radius,
                velocity: Vec2::new(rng.gen_range(-200.0..200.0), rng.gen_range(-200.0..200.0)),
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(colors[i % colors.len()])),
                transform,
                ..default()
            },
        ));
    }
}

fn update(
    time: Res<Time>,
    mut query: Query<(&mut Circle, &mut Transform)>,
    window: Query<&Window>,
) {
    let window = window.single();
    let width = window.width() / 2.;
    let height = window.height() / 2.;
    for (mut circle, mut transform) in &mut query {
        if transform.translation.x >= width - circle.radius
            || transform.translation.x <= -width + circle.radius
        {
            circle.velocity.x *= -1.;
        }
        if transform.translation.y >= height - circle.radius
            || transform.translation.y <= -height + circle.radius
        {
            circle.velocity.y *= -1.;
        }
        circle.velocity.x = circle.velocity.x.clamp(-1000., 1000.);
        circle.velocity.y = circle.velocity.y.clamp(-1000., 1000.);
        // info!("{:?}", circle.velocity);
        transform.translation.x += circle.velocity.x * time.delta_seconds();
        transform.translation.y += circle.velocity.y * time.delta_seconds();
    }
}

fn collide(mut query: Query<(&mut Circle, &Transform)>) {
    let mut combo = query.iter_combinations_mut();
    let mut rng = rand::thread_rng();
    while let Some([(mut circle, transform), (mut other, other_transform)]) = combo.fetch_next() {
        let pt1 = Vec2::new(transform.translation.x, transform.translation.y);
        let pt2 = Vec2::new(other_transform.translation.x, other_transform.translation.y);
        let dist = pt1.distance(pt2);
        if dist < circle.radius + other.radius {
            // let dot = circle.velocity.dot(other.velocity);
            (circle.velocity, other.velocity) = (other.velocity, circle.velocity);
        }
    }
}
