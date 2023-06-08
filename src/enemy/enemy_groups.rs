
use bevy::prelude::*;

use crate::resources::{
    sprite_sheet::*,
};

pub use crate::enemy::{
    seraphic_skyrider_enemy::*,
    spike_enemy::*,
    bug_enemy::*,
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
    phoenix_nebulon_enemy::*,
};

#[warn(dead_code)]
pub enum AnyEnemyType {
    SeraphicSkyriderEnemyBundle,
    SpikeEnemyBundle,
    BugEnemyBundle,
    CelestialVoyagerEnemyBundle,
    StellarPhoenixEnemyBundle,
    NebulaSerpentEnemyBundle,
    GalacticHorizonEnemyBundle,
    AstralEclipseEnemyBundle,
    OrionsFuryEnemyBundle,
    SolarisNovaEnemyBundle,
    InterstellarFalconEnemyBundle,
    HyperionVanguardEnemyBundle,
    AndromedaAscendantEnemyBundle,
    NebulaWandererEnemyBundle,
    NovaStarstriderEnemyBundle,
    PhoenixNebulonEnemyBundle,
}

#[warn(dead_code)]
pub struct EnemyGroup<const N: usize>(pub [(AnyEnemyType, Vec2); N]);

#[warn(dead_code)]
impl<const N: usize> EnemyGroup<N> {
    pub fn summon(&self, commands: &mut Commands, asset_server: &Res<SpriteSheet>) {
        for (enemy, pos) in self.0.iter() {
            match enemy {
                AnyEnemyType::SeraphicSkyriderEnemyBundle => {
                    commands.spawn(SeraphicSkyriderEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::SpikeEnemyBundle => {
                    commands.spawn(SpikeEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::BugEnemyBundle => {
                    commands.spawn(BugEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::CelestialVoyagerEnemyBundle => {
                    commands.spawn(CelestialVoyagerEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::StellarPhoenixEnemyBundle => {
                    commands.spawn(StellarPhoenixEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::NebulaSerpentEnemyBundle => {
                    commands.spawn(NebulaSerpentEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::GalacticHorizonEnemyBundle => {
                    commands.spawn(GalacticHorizonEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::AstralEclipseEnemyBundle => {
                    commands.spawn(AstralEclipseEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::OrionsFuryEnemyBundle => {
                    commands.spawn(OrionsFuryEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::SolarisNovaEnemyBundle => {
                    commands.spawn(SolarisNovaEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::InterstellarFalconEnemyBundle => {
                    commands.spawn(InterstellarFalconEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::HyperionVanguardEnemyBundle => {
                    commands.spawn(HyperionVanguardEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::AndromedaAscendantEnemyBundle => {
                    commands.spawn(AndromedaAscendantEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::NebulaWandererEnemyBundle => {
                    commands.spawn(NebulaWandererEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::NovaStarstriderEnemyBundle => {
                    commands.spawn(NovaStarstriderEnemyBundle::new(&asset_server, *pos));
                },
                AnyEnemyType::PhoenixNebulonEnemyBundle => {
                    commands.spawn(PhoenixNebulonEnemyBundle::new(&asset_server, *pos));
                },
            }
        }
    }
}

pub const THREE_SERAPHICS_SKYDRIDERS: EnemyGroup<3> = EnemyGroup{ 0: [
    (AnyEnemyType::SeraphicSkyriderEnemyBundle, Vec2::new(-100.0, 330.0)),
    (AnyEnemyType::SeraphicSkyriderEnemyBundle, Vec2::new(0.0, 330.0)),
    (AnyEnemyType::SeraphicSkyriderEnemyBundle, Vec2::new(100.0, 330.0))
]};