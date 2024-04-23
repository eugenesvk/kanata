#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
// #![allow(non_upper_case_globals)]

extern crate native_windows_gui    as nwg;
extern crate native_windows_derive as nwd;
use core::cell::RefCell;
use nwd::NwgUi;
use nwg::{NativeUi,ControlHandle};

#[derive(Default,Debug,Clone)] pub struct SystemTrayData {
  pub ttt:String,
}
#[derive(Default, NwgUi)] pub struct SystemTray {
  pub                                                     	handle        	: ControlHandle,
  pub                                                     	data          	: RefCell<SystemTrayData>,
  pub                                                     	tray_item_dyn 	: RefCell<Vec<nwg::MenuItem>>,
  pub                                                     	handlers_dyn  	: RefCell<Vec<nwg::EventHandler>>,
  #[nwg_resource]                                         	pub embed     	: nwg::EmbedResource,
  #[nwg_control]                                          	pub window    	: nwg::MessageWindow,
  #[nwg_resource(source_embed    :Some(&data.embed)       	              	//
    ,            source_embed_str:Some("iconMain"))]      	pub icon      	: nwg::Icon,
  #[nwg_control(icon:Some(&data.icon)                     	              	//
    ,           tip :Some("TipHello"))]                   	              	//
   #[nwg_events(MousePressLeftUp:[SystemTray::show_menu]  	              	//
    ,           OnContextMenu   :[SystemTray::show_menu])]	pub tray      	: nwg::TrayNotification,
  #[nwg_control(parent:window   , popup: true)]           	pub tray_menu 	: nwg::Menu,
  #[nwg_control(parent:tray_menu, text:"&1 Hello")]       	              	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::hello1])] 	pub tray_item1	: nwg::MenuItem,
  #[nwg_control(parent:tray_menu, text:"&2 Popup")]       	              	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::hello2])] 	pub tray_item2	: nwg::MenuItem,
  #[nwg_control(parent:tray_menu, text:"&X Exit\t‹⎈␠⎋")]  	              	//
   #[nwg_events(OnMenuItemSelected:[SystemTray::exit  ])] 	pub tray_item3	: nwg::MenuItem,
}
use winapi::shared::windef::{HWND, HMENU};
///fn change_menu_item_text(menu: &nwg::Menu, item_id: u32, new_text: &str) {
///  let mut item_info = nwg::MenuItemInfo::default(); // Get the current menu item info
///  item_info.text = Some(String::new()); // Initialize with an empty string to get the current text
///  menu.get_item_info(item_id, &mut item_info).expect("Failed to get menu item info");
///  item_info.text = Some(new_text.to_string()); // Modify the text
///  menu.set_item_info(item_id, &item_info).expect("Failed to set menu item info"); // Set the modified menu item info
///  // Optionally, redraw the menu or the window containing the menu This step might be necessary depending on your application's behavior
///}
impl SystemTray {
  // fn update_menu(&self) {
  //   self.nwg::TextInput::builder()
  //       .text(&data.form_data)
  //       .parent(&parent)
  //       .build(&mut data.value)?;
  // }
  // pub fn hmenu_item(&self) -> Option<(HMENU, u32)> {
  //   match &self.tray_item1.handle {
  //     &ControlHandle::MenuItem(h, i) => Some((h, i)),
  //     _ => None,
  //   }
  // }
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
