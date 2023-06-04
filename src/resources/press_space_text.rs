use bevy::prelude::*;

use crate::components::{
    menu_scene_system::*,
};

#[derive(Resource)]
pub struct PressSpaceTextImage {
    pub handle: Handle<Image>
}

#[derive(Component)]
pub struct Wavy {
    amplitude: f32,
    timer: f32,
    speed: f32,
}

impl Wavy {
    pub fn new(amplitude: f32, speed: f32) -> Self {
        Wavy { 
            amplitude: amplitude, 
            timer: 0.0,
            speed: speed,
        }
    }
}

pub fn load_press_space_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image: Handle<Image> = asset_server.load("press-space-text.png");
    commands.insert_resource(PressSpaceTextImage{handle: image});
}

pub fn spawn_press_space_text_system(mut commands: Commands, image: Res<PressSpaceTextImage>) {
    let sprite = Sprite{
        color: Color::rgb(1.0, 1.0, 1.0),
        custom_size: Some(Vec2::new(129.0, 36.0)),
        ..default()
    };

    commands.spawn(SpriteBundle {
        sprite: sprite,
        texture: image.handle.clone(),
        transform: Transform{
            translation: Vec3::new(0.0, -240.0, 0.0),
            ..default()
        },
        ..default()
    }).insert(MenuEntity).insert(Wavy::new(5.0, 5.0));
}

pub fn wavy_update_system(mut targets: Query<(&mut Wavy, &mut Transform)>, time: Res<Time>) {
    for (mut wavy, mut transform) in targets.iter_mut() {
        wavy.timer += time.delta_seconds();
        transform.translation.y -= ((wavy.timer * wavy.speed).sin() / 600.0) * wavy.amplitude;
    }
}
