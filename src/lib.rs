#![allow(clippy::type_complexity)]

use bevy::prelude::*;

pub mod c8device;
pub mod display;
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

#[derive(Resource, Default)]
pub struct DeviceContext {
    device: c8device::C8Device,
}
