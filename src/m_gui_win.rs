#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
// #![allow(non_upper_case_globals)]

extern crate native_windows_gui    as nwg;
extern crate native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)] pub struct SystemTray {
  #[nwg_resource]                                         	embed     	: nwg::EmbedResource,
  #[nwg_control]                                          	window    	: nwg::MessageWindow,
  #[nwg_resource(source_embed    :Some(&data.embed)       	          	//
    ,            source_embed_str:Some("iconMain"))]      	icon      	: nwg::Icon,
  #[nwg_control(icon:Some(&data.icon)                     	          	//
    ,           tip :Some("Hello"))]                      	          	//
   #[nwg_events(MousePressLeftUp:[SystemTray::show_menu]  	          	//
    ,           OnContextMenu   :[SystemTray::show_menu])]	tray      	: nwg::TrayNotification,
  #[nwg_control(parent:window   , popup: true)]           	tray_menu 	: nwg::Menu,
  #[nwg_control(parent:tray_menu, text:"&1 Hello")]       	          	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::hello1])] 	tray_item1	: nwg::MenuItem,
  #[nwg_control(parent:tray_menu, text:"&2 Popup")]       	          	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::hello2])] 	tray_item2	: nwg::MenuItem,
  #[nwg_control(parent:tray_menu, text:"&X Exit")]        	          	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::exit  ])] 	tray_item3	: nwg::MenuItem,
}
impl SystemTray {
  pub fn add_menu(&self) -> Result<(),nwg::NwgError> {
    // let title 	= self.message_title  .text();
    let menu_text	= "Add_menu New menu item!";

    let mut new_menu = Default::default();
    nwg::MenuItem::builder().text(menu_text).parent(&self.tray_menu).build(&mut new_menu)?;

    let mut tray_item_dyn 	= self.tray_item_dyn.borrow_mut();
    // let mut handler_dyn	= self.handlers_dyn .borrow_mut();

    tray_item_dyn.push(new_menu);
    Ok(())
  }

  fn show_menu(&self) {
    let (x, y) = nwg::GlobalCursor::position();
    self.tray_menu.popup(x, y);  }
  fn hello1(&self) {nwg::simple_message("HelloMsg", "Hello World!");}
  fn hello2(&self) {
    let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
    self.tray.show("Hello World", Some("Welcome to my application"), Some(flags), Some(&self.icon));  }
  fn exit(&self) {nwg::stop_thread_dispatch();}
}

pub use log::*;
pub use winapi::um::wincon::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};
pub use winapi::shared::minwindef::BOOL;
pub use std::io::{stdout, IsTerminal};

use once_cell::sync::Lazy;
pub static IS_TERM:Lazy<bool> = Lazy::new(||stdout().is_terminal());
pub static IS_CONSOLE:Lazy<bool> = Lazy::new(|| unsafe{
  if AttachConsole(ATTACH_PARENT_PROCESS)== 0i32 {return false} else {return true}});
