use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::enemy::{
    enemy::*,

    seraphic_skyrider_enemy::*,
    bug_enemy::*,
    spike_enemy::*,
    celestial_voyager_enemy::*,
    stellar_phoenix_enemy::*,
    nebula_serpent_enemy::*,
    galactic_horizon_enemy::*,
    astral_eclipse_enemy::*,
    orions_fury_enemy::*,
    solaris_nova_enemy::*,
    interstellar_falcon_enemy::*,
    hyperion_vanguard_enemy::*,
    andromeda_ascendant_enemy::*,
    nebula_wanderer_enemy::*,
    nova_starstrider_enemy::*,
    phoenix_nebulon_enemy::*
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_component_as::<dyn Enemy, SeraphicSkyriderEnemy>();
        app.register_component_as::<dyn Enemy, SpikeEnemy>();
        app.register_component_as::<dyn Enemy, BugEnemy>();
        app.register_component_as::<dyn Enemy, CelestialVoyagerEnemy>();

        app.register_component_as::<dyn Enemy, StellarPhoenixEnemy>();
        app.register_component_as::<dyn Enemy, NebulaSerpentEnemy>();
        app.register_component_as::<dyn Enemy, GalacticHorizonEnemy>();
        app.register_component_as::<dyn Enemy, AstralEclipseEnemy>();
        
        app.register_component_as::<dyn Enemy, OrionsFuryEnemy>();
        app.register_component_as::<dyn Enemy, HyperionVanguardEnemy>();
        app.register_component_as::<dyn Enemy, SolarisNovaEnemy>();
        app.register_component_as::<dyn Enemy, InterstellarFalconEnemy>();

        app.register_component_as::<dyn Enemy, AndromedaAscendantEnemy>();
        app.register_component_as::<dyn Enemy, NovaStarstriderEnemy>();
        app.register_component_as::<dyn Enemy, NebulaWandererEnemy>();
        app.register_component_as::<dyn Enemy, PhoenixNebulonEnemy>();
    }
}