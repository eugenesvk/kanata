/*! A application that customize a window to display a splash screen. Click the splash screen to close the app.
cargo run --example splash_screen_d --features "image-decoder"
*/
#![allow(unused_imports,dead_code)]
use native_windows_gui as nwg;
use native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)] pub struct SplashScreen {
  #[nwg_control(size: (500, 215), position: (500, 300), flags: "MAIN_WINDOW|VISIBLE", topmost: true)]
  window: nwg::Window,
  #[nwg_resource(source_file:Some(r".\test_rc\splash.png"))]
  splash: nwg::Bitmap,
  #[nwg_control(size: (500, 215), bitmap: Some(&data.splash) )]
  #[nwg_events(OnImageFrameClick: [SplashScreen::exit])]
  image_frame:nwg::ImageFrame
}
impl SplashScreen {fn exit(&self) {nwg::stop_thread_dispatch();}}

use log::*;
pub fn lib_main_gui2() {
  info!("dsfsdaf");
  println!("dsfsdaf");
  nwg::init().expect("Failed to init Native Windows GUI");
  let _app = SplashScreen::build_ui(Default::default()).expect("Failed to build UI");
  nwg::dispatch_thread_events();
}
