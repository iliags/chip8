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
