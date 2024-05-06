#![cfg_attr(feature = "gui", windows_subsystem = "windows")] // disable default console for a Windows GUI app
#![allow(non_upper_case_globals)]

#[cfg(not(feature = "gui"))] use kanata_state_machine::lib_main::lib_main_cli;
#[cfg(    feature = "gui" )] use kanata_state_machine::gui::lib_main_gui;

#[cfg(not(feature = "gui"))]
use anyhow::{Result};
#[cfg(not(feature = "gui"))]
fn main() -> Result<()> {
    let ret = lib_main_cli();
    ret
}

#[cfg(feature = "gui")]
fn main() {
    lib_main_gui();
}
