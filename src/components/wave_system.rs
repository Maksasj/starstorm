use bevy::prelude::*;

pub use crate::components::{
    entity_rotation::*,
    player_controller::*,

    bug_enemy::*,
    spike_enemy::*,
    simple_enemy::*,
    enemy::*,
    game_scene_system::*,
};

pub use crate::resources::{
    sprite_sheet::*,
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
    ) {
    
    if targets.is_empty() {
        for mut spawner in wave_spawners.iter_mut() {
            spawner.current += 1;
            spawner.timer = 0.0;

            events.send(WaveSwitchEvent::new(spawner.current));
        }
    }
}

pub fn wave_spawn_system(
        mut commands: Commands,
        mut events: EventReader<WaveSwitchEvent>, 
        asset_server: Res<SpriteSheet>
    ) {

    for event in events.iter() {
        let _to = event.to;

        commands.spawn(BugEnemyBundle::new(&asset_server, Vec2::new(100.0, 330.0)));
        commands.spawn(SimpleEnemyBundle::new(&asset_server, Vec2::new(0.0, 330.0)));
        commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-100.0, 330.0)));
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