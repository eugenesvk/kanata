#![allow(unused_imports,unused_labels,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
// todo: set logging level from an external call

use anyhow::Result;
use anyhow::{anyhow, bail};
use clap::Parser;
use kanata_parser::keys::str_to_oscode;
use kanata_state_machine::{oskbd::*, *};
use log::*;


fn log_init() {
  let _ = log_win::init();
  let a = log_win::set_thread_state(true);
  log::set_max_level(log::LevelFilter::Debug);
}

use std::sync::{Arc,OnceLock};
use parking_lot::Mutex;
static CFG:OnceLock<Arc<Mutex<Kanata>>> = OnceLock::new();

use winapi::shared::minwindef::*;
#[no_mangle] pub extern "C" fn reset_kanata_state() -> LRESULT {
  debug!("                   ext →→→ reset_kanata_state");
  if let Some(cfg) = CFG.get() {
    if kanata::clean_state(&cfg).is_err()	{debug!("✗ @ reset_kanata_state"        );return 1};
  } else                                 	{debug!("✗ @ reset_kanata_state, no CFG");return 2};
  0
}

use std::path::PathBuf;
/// Parse CLI arguments
fn cli_init() -> Result<ValidatedArgs> {
  let cfg_file = PathBuf::from(r#"./sim.kbd"#);
  let sim_file = PathBuf::from(r#"./sim.txt"#);

  if !cfg_file.exists() {bail!("Could not find the config file ({:?})"    ,cfg_file)}
  if !sim_file.exists() {bail!("Could not find the simulation file ({:?})",sim_file)}
  Ok(ValidatedArgs {
    paths       	: vec![cfg_file],
    nodelay     	: true,
    sim_paths   	: vec![sim_file],
    sim_appendix	: None,
    },
  )
}

fn lib_impl() -> Result<()> {
  log_init();
  let args = cli_init()?;
  let cfg_arc = Kanata::new_arc(&args)?; // new configuration from a file

  // Start a processing loop in another thread and run the event loop in this thread
  // The reason for two different event loops is that the "event loop" only listens for keyboard events, which it sends to the "processing loop". The processing loop handles keyboard events while also maintaining `tick()` calls to keyberon.
  let (tx,rx) = std::sync::mpsc::sync_channel(100);
  let ntx = None;
  Kanata::start_processing_loop(cfg_arc.clone(), rx, ntx, args.nodelay); // 2 handles keyboard events while also maintaining `tick()` calls to keyberon

  Kanata::event_loop(cfg_arc, tx)?; // 1 only listens for keyboard events (not a real loop, just registers callback closures for external function to call at will)

  // for config_sim_file in &args.sim_paths {
  //   let mut k = Kanata::new(&args)?;
  //   log::debug!("Evaluating simulation file = {:?}", config_sim_file);
  //   let s = std::fs::read_to_string(config_sim_file)?;
  //   for l in s.lines() {
  //     for pair in l.split_whitespace() {
  //       match pair.split_once(':') {
  //         Some((kind, val)) => match kind {
  //           "tick" | "🕐" | "t"           => {
  //             let tick = str::parse::<u128>(val)?;
  //             k.kbd_out.log.in_tick(tick);
  //             k.tick_ms(tick)?;}
  //           "press" | "↓" | "d" | "down"  => {
  //             let key_code = str_to_oscode(val).ok_or_else(|| anyhow!("unknown key: {val}"))?;
  //             k.kbd_out.log.in_press_key(key_code);
  //             k.handle_input_event(&KeyEvent {code:key_code, value:KeyValue::Press,})?;}
  //           "release" | "↑" | "u" | "up"  => {
  //             let key_code = str_to_oscode(val).ok_or_else(|| anyhow!("unknown key: {val}"))?;
  //             k.kbd_out.log.in_release_key(key_code);
  //             k.handle_input_event(&KeyEvent {code:key_code, value:KeyValue::Release,})?;}
  //           "repeat" | "⟳" | "r"         => {
  //             let key_code = str_to_oscode(val).ok_or_else(|| anyhow!("unknown key: {val}"))?;
  //             k.kbd_out.log.in_repeat_key(key_code);
  //             k.handle_input_event(&KeyEvent {code:key_code, value:KeyValue::Repeat,})?;}
  //           _ => bail!("invalid pair prefix: {kind}"),
  //         },
  //         None => bail!("invalid pair: {l}"),
  //       }
  //     }
  //   }
  //   k.kbd_out.log.end(config_sim_file, args.sim_appendix.clone());
  // }

  Ok(())
}

use kanata_parser::keys::OsCode;
use std::panic::catch_unwind;


use log::{debug, info};
mod log_win;
#[no_mangle] pub extern "C"
fn sim_evt() -> bool {
  let ret = lib_impl();
  if let Err(ref e) = ret {log::error!("{e}\n"); return false}
  // ret
  // let result = catch_unwind(|| {
  //   panic!("Oops!");
  // });
  // match result {
  //   Ok(_) => dbgwin(&format!("successfully caught unwind")),
  //   Err(cause) => dbgwin(&format!("panic! cause={:?}",cause)),
  // }

  // let key_code = str_to_oscode("val").unwrap_or(OsCode::KEY_RESERVED);
  // debug!("key_code={key_code:?}");
  debug!("✗✗✗ sim_evt finished");
  true
}
