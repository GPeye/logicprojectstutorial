use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::player::EncounterTracker;
use crate::player::Player;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_type::<EncounterTracker>()
                .register_inspectable::<Player>();
        }
    }
}
