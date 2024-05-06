use crate::*
use clap::{CommandFactory,error::ErrorKind};
use anyhow::Context;
use native_windows_gui    as nwg;

use parking_lot::Mutex;
use std::sync::{Arc, OnceLock};
pub static CFG: OnceLock<Arc<Mutex<Kanata>>> = OnceLock::new();

pub fn lib_main_gui() {
  let _attach_console = *IS_CONSOLE;
  let ret = main_impl();
  if let Err(ref e) = ret {log::error!("{e}\n");}
  unsafe {FreeConsole();}
}
