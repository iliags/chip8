#![allow(clippy::type_complexity)]

use bevy::prelude::*;

pub mod c8device;
pub mod input;
pub mod ui;

//use crate::ui::UIPlugin;

#[derive(Resource)]
pub struct KeysPressed {
    keys: [bool; 16],
}

impl Default for KeysPressed {
    fn default() -> Self {
        Self { keys: [false; 16] }
    }
}

#[derive(Resource)]
pub struct DeviceContext {
    device: c8device::C8Device,
}
