use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::components::player_shoot::*;

use crate::weapon::{
    weapon::*,

    meteoric_bluster::*,
    onyx_bluster::*,
    mortar_bluster::*,
    ion_bluster::*,
    dual_space_blast_cannon::*,
    nova_burst_cannon::*,
    galactic_plasma_disruptor::*,
    dual_rapid_space_blast_cannon::*,
    quantum_fusion_cannon::*,
    dual_starfire_bluster::*,
    hyperion_particle_beam::*,
    plasma_fusion_mortar::*,
    void_energy_slicer::*,
    quantum_repulsor_cannon::*,
    nova_surge_launcher::*,
    quantum_blaze::*,
    ionized_pulse_blaster::*,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerShootEvent>();

        app.register_component_as::<dyn Weapon, MeteoricBluster>();
        app.register_component_as::<dyn Weapon, OnyxBluster>();
        app.register_component_as::<dyn Weapon, MortarBluster>();
        app.register_component_as::<dyn Weapon, IonBluster>();
        app.register_component_as::<dyn Weapon, DualSpaceBlastCannon>();
        app.register_component_as::<dyn Weapon, NovaBurstCannon>();
        app.register_component_as::<dyn Weapon, GalacticPlasmaDisruptor>();
        app.register_component_as::<dyn Weapon, DualRapidSpaceBlastCannon>();
        app.register_component_as::<dyn Weapon, QuantumFusionCannon>();
        app.register_component_as::<dyn Weapon, DualStarfireBluster>();
        app.register_component_as::<dyn Weapon, HyperionParticleBeam>();
        app.register_component_as::<dyn Weapon, PlasmaFusionMortar>();
        app.register_component_as::<dyn Weapon, VoidEnergySlicer>();
        app.register_component_as::<dyn Weapon, QuantumRepulsorCannon>();
        app.register_component_as::<dyn Weapon, NovaSurgeLauncher>();
        app.register_component_as::<dyn Weapon, QuantumBlaze>();
        app.register_component_as::<dyn Weapon, IonizedPulseBlaster>();
    }
}