// publics 
pub mod axis;
pub mod inputmap;
pub mod util;
pub mod bindings;
pub mod stack;
pub mod eventphase;

// crates
mod serde;
mod action;
mod keyboard;
mod mouse;
mod gamepad;
use crate::inputmap::InputMap;
use bevy_app::prelude::*;
use bevy_ecs::IntoQuerySystem;

#[derive(Default)]
pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // input map
            .init_resource::<InputMap>()
            .add_system_to_stage(stage::EVENT_UPDATE, InputMap::action_reset_system.system())
            // joystick
            .add_system_to_stage(stage::UPDATE, InputMap::gamepad_connection_event_system.system())
            .add_system_to_stage(stage::UPDATE, InputMap::gamepad_button_press_input_system.system())
            .add_system_to_stage(stage::UPDATE, InputMap::gamepad_axis_system.system())
            // keyboard
            .add_system_to_stage(stage::UPDATE, InputMap::kb_key_press_input_system.system())
            // mouse
            .add_system_to_stage(stage::UPDATE, InputMap::mouse_button_press_input_system.system())
            .add_system_to_stage(stage::UPDATE, InputMap::mouse_move_event_system.system());
    }
}
