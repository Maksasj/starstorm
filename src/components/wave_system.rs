use bevy::prelude::*;

use crate::Enemy;
pub use crate::components::{
    entity_rotation::*,
    player_controller::*,
};

#[derive(Component)]
pub struct WaveSpawner {
    pub current: usize,

    timer: f32
}

impl WaveSpawner {
    pub fn new() -> Self {
        WaveSpawner{
            current: 0,
            timer: 0.0,
        }
    } 
}

pub struct WaveSwitchEvent {
    to: usize,
}

impl WaveSwitchEvent {
    pub fn new(to: usize) -> Self {
        WaveSwitchEvent{
            to: to,
        }
    } 
}

pub fn spawn_wave_spawner_system(mut events: EventWriter<WaveSwitchEvent>, mut commands: Commands) {
    commands
        .spawn(VisibilityBundle::default())
        .insert(Name::new("WaveSpawner"))
        .insert(Transform { 
            translation: Vec3::new(0.0, 0.0, 0.0), 
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .insert(WaveSpawner::new());

    events.send(WaveSwitchEvent::new(0));
}

pub fn wave_counting_system(
        mut events: EventWriter<WaveSwitchEvent>, 
        mut wave_spawners: Query<&mut WaveSpawner>, 
        time: Res<Time>
    ) {
    
    for mut spawner in wave_spawners.iter_mut() {
        spawner.timer += time.delta_seconds(); 
        
        if spawner.timer > 50.0 {
            spawner.current += 1;
            spawner.timer = 0.0;

            events.send(WaveSwitchEvent::new(spawner.current));
        }
    }
}

pub fn wave_clear_system(
        mut events: EventWriter<WaveSwitchEvent>, 
        targets: Query<(Entity, &dyn Enemy)>,
        mut wave_spawners: Query<&mut WaveSpawner>, 
        time: Res<Time>
    ) {
    
    if targets.is_empty() {
        for mut spawner in wave_spawners.iter_mut() {
            spawner.timer += time.delta_seconds(); 
            
            if spawner.timer > 50.0 {
                spawner.current += 1;
                spawner.timer = 0.0;
    
                events.send(WaveSwitchEvent::new(spawner.current));
            }
        }
    }
}

pub fn wave_spawn_system(
        _commands: Commands,
        mut events: EventReader<WaveSwitchEvent>, 
    ) {

    for event in events.iter() {
        println!("Switched wavy to {:?}", event.to);
    }

    events.clear();
}

/*
pub fn spawn_player_health_text_system(mut commands: Commands, asset_server: Res<SmallNumberFontSpriteSheet>) {
    let mut childrens = Vec::new();

    for i in 0..7 {
        let mut sprite;
        
        if i == 3 {
            sprite = TextureAtlasSprite::new(10);
        } else {
            sprite = TextureAtlasSprite::new(0);
        }

        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(24.0));
        
        let character = commands.spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: asset_server.handle.clone(),
            transform: Transform { 
                translation: Vec3::new(0.0 + (i as f32) * 15.0, 0.0, 900.0), 
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("PlayerHealthText"))
        .insert(Visibility::Visible)
        .insert(PlayerHealthText::new(i))
        .insert(GameEntity).id();

        childrens.push(character);
    }

    commands
        .spawn(VisibilityBundle::default())
        .insert(Name::new("PlayerHealthTextParent"))
        .insert(Transform { 
            translation: Vec3::new(-210.0, 255.0, 0.0), 
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .push_children(&childrens);
}
 */