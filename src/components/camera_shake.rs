use bevy::{
    prelude::*, 
};

use rand::Rng;

pub struct CameraShakeEvent {
    amplitude: f32,
    time: f32
}

impl CameraShakeEvent {
    pub fn new(amplitude: f32, time: f32) -> Self {
        CameraShakeEvent {
            amplitude: amplitude,
            time: time,
        }
    }
}

#[derive(Resource)]
pub struct CameraShaking {
    timer: f32,
    shake_time: f32, 
    amplitude: f32,
    enabled: bool
}

pub fn setup_camera_shake_system(mut commands: Commands) {
    commands.insert_resource(CameraShaking{    
        timer: 0.0,
        shake_time:  0.0,
        amplitude: 0.0,
        enabled: false,
    });
}

pub fn camera_shake_system(
        mut camera_shake: ResMut<CameraShaking>, 
        mut cameras: Query<&mut Transform, With<Camera>>, 
        mut events: EventReader<CameraShakeEvent>,
        time: Res<Time>
    ) {

    if camera_shake.enabled == true {
        camera_shake.timer += time.delta_seconds();
        
        if camera_shake.timer > camera_shake.shake_time {
            camera_shake.enabled = false;
            camera_shake.timer = 0.0;
        }

        for mut camera_transfor in cameras.iter_mut() {
            let mut rng = rand::thread_rng();

            let x = rng.gen_range(-1.0..1.0) * camera_shake.amplitude;
            let y = rng.gen_range(-1.0..1.0) * camera_shake.amplitude;

            camera_transfor.translation.x = x;
            camera_transfor.translation.y = y;
        }
    }
    
    for event in events.iter() {
        camera_shake.enabled = true;
        camera_shake.shake_time = event.time;
        camera_shake.timer = 0.0;
        camera_shake.amplitude = event.amplitude;
    }
    
    events.clear();
}

