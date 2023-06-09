use bevy::prelude::*;
use rand::Rng;

pub use crate::enemy::{
    enemy::*,
    enemy_groups::*,
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

        let mut rng = rand::thread_rng();
        let value: u8 = rng.gen();

        if to <= 2 {
            match value % 2 {
                0 => { THREE_SERAPHICS_SKYRIDERS.summon(&mut commands, &asset_server); },
                _ => { SOLARIS_NAVY.summon(&mut commands, &asset_server); }
            }
        } else if to <= 5 {
            match value % 4 {
                1 => { STELLAR_NAVY.summon(&mut commands, &asset_server); },
                2 => { HYPERION_NAVY.summon(&mut commands, &asset_server); },
                3 => { FALCON_FLOTILLA.summon(&mut commands, &asset_server); },
                _ => { CELESTIAL_ARMADA.summon(&mut commands, &asset_server); }
            }
        } else  if to <= 10 {
            match value % 3 {
                0 => { NEBULON_ARMADA.summon(&mut commands, &asset_server); },
                1 => { NEBULON_ARMADA_BIG.summon(&mut commands, &asset_server); },
                _ => { INFINITY_ARMADA.summon(&mut commands, &asset_server); }
            }
        } else if to <= 15 {
            match value % 4 {
                0 => { NEBULON_ARMADA.summon(&mut commands, &asset_server); },
                1 => { NEBULON_ARMADA_BIG.summon(&mut commands, &asset_server); },
                2 => { NEBULOUS_LEGION.summon(&mut commands, &asset_server); },
                _ => { INFINITY_ARMADA.summon(&mut commands, &asset_server); }
            }
        } else {
            match value % 4 {
                0 => { 
                    HYPERION_NAVY.summon(&mut commands, &asset_server);
                    NEBULON_ARMADA.summon(&mut commands, &asset_server); 
                },
                1 => { 
                    STELLAR_NAVY.summon(&mut commands, &asset_server);
                    NEBULON_ARMADA_BIG.summon(&mut commands, &asset_server);
                },
                2 => { 
                    NEBULON_ARMADA.summon(&mut commands, &asset_server);
                    NEBULOUS_LEGION.summon(&mut commands, &asset_server);
                },
                _ => { 
                    FALCON_FLOTILLA.summon(&mut commands, &asset_server);
                    INFINITY_ARMADA.summon(&mut commands, &asset_server);
                }
            }
        }
    }

    events.clear();
}
