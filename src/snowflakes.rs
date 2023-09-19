use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
struct Snowflake {
    angular_velocity: f32,
    linear_x_velocity: f32,
    linear_y_velocity: f32,
}

pub struct SnowflakesPlugin;

impl Plugin for SnowflakesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TitleScreen), setup_snowflakes)
            .add_systems(OnEnter(GameState::ShowScore), setup_snowflakes)
            .add_systems(
                Update,
                update_snowflakes.run_if(in_state(GameState::TitleScreen)),
            )
            .add_systems(
                Update,
                update_snowflakes.run_if(in_state(GameState::Instructions)),
            )
            .add_systems(
                Update,
                update_snowflakes.run_if(in_state(GameState::ShowScore)),
            )
            .add_systems(OnExit(GameState::Instructions), cleanup_snowflakes);
    }
}

fn setup_snowflakes(mut commands: Commands, asset_server: Res<AssetServer>) {
    // the use of the rand library aborts is WASM so this list was created in excel using
    // the RAND() function and the copied here.
    // by accessing a modulo index below, this gives the random effect for each attribute.
    let mut random_numbers: Vec<f32> = Vec::new();
    random_numbers.push(0.487);
    random_numbers.push(0.195);
    random_numbers.push(0.544);
    random_numbers.push(0.004);
    random_numbers.push(0.217);
    random_numbers.push(0.031);
    random_numbers.push(0.466);
    random_numbers.push(0.828);
    random_numbers.push(0.235);
    random_numbers.push(0.690);
    random_numbers.push(0.903);
    random_numbers.push(0.787);
    random_numbers.push(0.550);
    random_numbers.push(0.042);
    random_numbers.push(0.603);
    random_numbers.push(0.657);
    random_numbers.push(0.247);
    random_numbers.push(0.505);
    random_numbers.push(0.237);
    random_numbers.push(0.150);
    random_numbers.push(0.357);
    random_numbers.push(0.624);
    random_numbers.push(0.611);
    random_numbers.push(0.411);
    random_numbers.push(0.323);
    random_numbers.push(0.269);
    random_numbers.push(0.574);
    random_numbers.push(0.771);
    random_numbers.push(0.998);
    random_numbers.push(0.610);
    random_numbers.push(0.735);
    random_numbers.push(0.503);
    random_numbers.push(0.828);
    random_numbers.push(0.995);
    random_numbers.push(0.104);
    random_numbers.push(0.489);
    random_numbers.push(0.027);
    random_numbers.push(0.263);
    random_numbers.push(0.681);
    random_numbers.push(0.490);
    random_numbers.push(0.253);
    random_numbers.push(0.525);
    random_numbers.push(0.102);
    random_numbers.push(0.269);
    random_numbers.push(0.406);
    random_numbers.push(0.892);
    random_numbers.push(0.326);
    random_numbers.push(0.357);
    random_numbers.push(0.069);
    random_numbers.push(0.160);
    random_numbers.push(0.462);
    random_numbers.push(0.625);
    random_numbers.push(0.639);
    random_numbers.push(0.883);
    random_numbers.push(0.975);
    random_numbers.push(0.506);
    random_numbers.push(0.272);
    random_numbers.push(0.567);
    random_numbers.push(0.462);
    random_numbers.push(0.072);
    random_numbers.push(0.971);
    random_numbers.push(0.628);
    random_numbers.push(0.043);
    random_numbers.push(0.179);
    random_numbers.push(0.140);
    random_numbers.push(0.048);
    random_numbers.push(0.485);
    random_numbers.push(0.505);
    random_numbers.push(0.610);
    random_numbers.push(0.432);
    random_numbers.push(0.024);
    random_numbers.push(0.018);
    random_numbers.push(0.676);
    random_numbers.push(0.680);
    random_numbers.push(0.721);
    random_numbers.push(0.807);
    random_numbers.push(0.797);
    random_numbers.push(0.758);
    random_numbers.push(0.050);
    random_numbers.push(0.043);
    random_numbers.push(0.861);
    random_numbers.push(0.159);
    random_numbers.push(0.162);
    random_numbers.push(0.952);
    random_numbers.push(0.629);
    random_numbers.push(0.999);
    random_numbers.push(0.782);
    random_numbers.push(0.638);
    random_numbers.push(0.939);
    random_numbers.push(0.031);
    random_numbers.push(0.579);
    random_numbers.push(0.464);
    random_numbers.push(0.929);
    random_numbers.push(0.181);
    random_numbers.push(0.479);
    random_numbers.push(0.144);
    random_numbers.push(0.798);
    random_numbers.push(0.971);
    random_numbers.push(0.948);
    random_numbers.push(0.693);

    let mut x_positions: Vec<f32> = Vec::new();
    let mut y_positions: Vec<f32> = Vec::new();
    let mut angular_velocities: Vec<f32> = Vec::new();
    let mut linear_x_velocities: Vec<f32> = Vec::new();
    let mut linear_y_velocities: Vec<f32> = Vec::new();
    let mut rotations: Vec<f32> = Vec::new();
    let mut sizes: Vec<f32> = Vec::new();

    for index in 0..100 {
        x_positions.push((random_numbers[index] * 1600.) - 800.);
        y_positions.push((random_numbers[(index + 30) % 100] * 800.) - 400.);
        angular_velocities.push((random_numbers[(index + 50) % 100] * 0.05) - 0.025);
        linear_x_velocities.push(random_numbers[(index + 70) % 100] * 0.65);
        linear_y_velocities.push((random_numbers[(index + 20) % 100] * -2.00).min(-1.));
        rotations.push(random_numbers[(index + 90) % 100] * 180.);
        sizes.push(random_numbers[index] + 30.);
    }

    for n in 1..100 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("snowglobe/backgrounds/snowflake - white.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2 {
                        x: sizes[n],
                        y: sizes[n],
                    }),
                    color: Color::rgba(0.85, 0.85, 1., 0.1),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x_positions[n], y_positions[n], 0.0),
                    rotation: Quat::from_rotation_z((rotations[n]).to_radians()),
                    ..default()
                },

                ..default()
            },
            Snowflake {
                angular_velocity: angular_velocities[n],
                linear_x_velocity: linear_x_velocities[n],
                linear_y_velocity: linear_y_velocities[n],
            },
        ));
    }
}

fn update_snowflakes(mut transforms: Query<(&mut Transform, &Snowflake)>) {
    for (mut transform, snowflake) in &mut transforms {
        transform.translation +=
            Vec3::new(snowflake.linear_x_velocity, snowflake.linear_y_velocity, 0.);
        transform.rotate_y(snowflake.angular_velocity * 0.02);
        transform.rotate_z(snowflake.angular_velocity);

        if transform.translation.y < -400. {
            transform.translation.y = 400.
        }
        if transform.translation.x > 800. {
            transform.translation.x = -800.;
        }
    }
}

fn cleanup_snowflakes(mut commands: Commands, snowflakes: Query<Entity, With<Snowflake>>) {
    for snowflake in snowflakes.iter() {
        commands.entity(snowflake).despawn();
    }
}
