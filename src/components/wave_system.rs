use bevy::prelude::*;

pub use crate::enemy::{
    enemy::*,
    bug_enemy::*,
    spike_enemy::*,
    seraphic_skyrider_enemy::*,
};

pub use crate::components::{
    entity_rotation::*,
    player_controller::*,

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
    pub to: usize,
}

impl WaveSwitchEvent {
    pub fn new(to: usize) -> Self {
        WaveSwitchEvent{
            to: to,
        }
    } 
}

pub fn spawn_wave_spawner_system(mut commands: Commands) {
    commands
        .spawn(VisibilityBundle::default())
        .insert(Name::new("WaveSpawner"))
        .insert(Transform { 
            translation: Vec3::new(0.0, 0.0, 0.0), 
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .insert(WaveSpawner::new())
        .insert(GameEntity{});
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
        let to = event.to;

        let spike_center_offset = 60.0;

        if to <= 2 {
            commands.spawn(SeraphicSkyriderEnemyBundle::new(&asset_server, Vec2::new(-100.0, 330.0)));
            commands.spawn(SeraphicSkyriderEnemyBundle::new(&asset_server, Vec2::new(0.0, 330.0)));
            commands.spawn(SeraphicSkyriderEnemyBundle::new(&asset_server, Vec2::new(100.0, 330.0)));
        } else if to <= 5 {
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-150.0 - spike_center_offset, 330.0)));
            commands.spawn(SeraphicSkyriderEnemyBundle::new(&asset_server, Vec2::new(-50.0, 330.0)));
            commands.spawn(SeraphicSkyriderEnemyBundle::new(&asset_server, Vec2::new(50.0, 330.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(150.0 - spike_center_offset, 330.0)));
        } else  if to <= 10 {
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-150.0 - spike_center_offset, 360.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-150.0 - spike_center_offset, 330.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(150.0 - spike_center_offset, 360.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(150.0 - spike_center_offset, 330.0)));

            commands.spawn(SeraphicSkyriderEnemyBundle::new(&asset_server, Vec2::new(-200.0, 330.0)));
            commands.spawn(SeraphicSkyriderEnemyBundle::new(&asset_server, Vec2::new(200.0, 330.0)));
        } else if to <= 15 {
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-75.0 - spike_center_offset, 400.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-150.0 - spike_center_offset, 370.0)));
            commands.spawn(BugEnemyBundle::new(&asset_server, Vec2::new(-130.0, 330.0)));
            commands.spawn(BugEnemyBundle::new(&asset_server, Vec2::new(-130.0, 370.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(150.0 - spike_center_offset, 370.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(75.0 - spike_center_offset, 400.0)));
        } else {
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-75.0 - spike_center_offset, 440.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-75.0 - spike_center_offset, 400.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(-150.0 - spike_center_offset, 370.0)));
            commands.spawn(BugEnemyBundle::new(&asset_server, Vec2::new(-130.0, 330.0)));
            commands.spawn(BugEnemyBundle::new(&asset_server, Vec2::new(-130.0, 370.0)));
            commands.spawn(BugEnemyBundle::new(&asset_server, Vec2::new(-130.0, 400.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(150.0 - spike_center_offset, 370.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(75.0 - spike_center_offset, 400.0)));
            commands.spawn(SpikeEnemyBundle::new(&asset_server, Vec2::new(75.0 - spike_center_offset, 440.0)));
        }
    }

    events.clear();
}
