#![allow(non_upper_case_globals)]
// destroy tray on exit
// hide main window without flashing
// show main window on popup

extern crate native_windows_gui    as nwg;
extern crate native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)] pub struct SystemTray {
  #[nwg_control]                                           	window	: nwg::MessageWindow,
  #[nwg_resource(source_file:Some("../assets/kanata.ico"))]	icon  	: nwg::Icon,
  #[nwg_control(icon:Some(&data.icon), tip: Some("Hello"))]	//
   #[nwg_events(MousePressLeftUp:[SystemTray::show_menu]   	//
    ,           OnContextMenu   :[SystemTray::show_menu])] 	tray     	: nwg::TrayNotification,
  #[nwg_control(parent:window   , popup: true)]            	tray_menu	: nwg::Menu,
  #[nwg_control(parent:tray_menu, text:"&1 Hello")]        	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::hello1])]  	tray_item1	: nwg::MenuItem,
  #[nwg_control(parent:tray_menu, text:"&2 Popup")]        	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::hello2])]  	tray_item2	: nwg::MenuItem,
  #[nwg_control(parent:tray_menu, text:"&X Exit")]         	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::exit  ])]  	tray_item3	: nwg::MenuItem,
}
impl SystemTray {
  fn show_menu(&self) {
    let (x, y) = nwg::GlobalCursor::position();
    self.tray_menu.popup(x, y);  }
  fn hello1(&self) {nwg::simple_message("Hello", "Hello World!");}
  fn hello2(&self) {
    let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
    self.tray.show("Hello World", Some("Welcome to my application"), Some(flags), Some(&self.icon));  }
  fn exit(&self) {nwg::stop_thread_dispatch();}
}
pub fn main_gui() {
  nwg::init().expect("Failed to init Native Windows GUI");
  let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
  nwg::dispatch_thread_events();
}
