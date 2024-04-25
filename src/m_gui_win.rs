#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
// #![allow(non_upper_case_globals)]

use std::ffi::OsStr;
use crate::{Kanata,kanata};
use parking_lot::Mutex;
use anyhow::{Result,Context};
extern crate native_windows_gui    as nwg;
extern crate native_windows_derive as nwd;
use std::sync::Arc;
use core::cell::RefCell;
use nwd::NwgUi;
use nwg::{NativeUi,ControlHandle};

use std::path::PathBuf;
#[derive(Default,Debug,Clone)] pub struct SystemTrayData {
  pub tooltip:String,
  pub cfg_p  :Vec<PathBuf>,
}
#[derive(Default)] pub struct SystemTray {
  pub app_data     	: RefCell<SystemTrayData>,
  ///              	Store dynamically created tray menu items
  pub tray_item_dyn	: RefCell<Vec<nwg::MenuItem>>,
  ///              	Store dynamically created tray menu items' handlers
  pub handlers_dyn 	: RefCell<Vec<nwg::EventHandler>>,
  ///              	Store embedded-in-the-binary resources like icons not to load them from a file
  pub embed        	: nwg::EmbedResource,
  pub icon         	: nwg::Icon,
  pub window       	: nwg::MessageWindow,
  pub tray         	: nwg::TrayNotification,
  pub tray_menu    	: nwg::Menu,
  pub tray_1cfg_m  	: nwg::Menu,
  pub tray_2reload 	: nwg::MenuItem,
  pub tray_3exit   	: nwg::MenuItem,
}
use crate::lib_main::CFG;
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
  //   match &self.tray_1cfg_m.handle {
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
  fn load_cfg(&self) {nwg::simple_message("HelloMsg", "Hello World!");}
  fn reload(&self) {
    use nwg::TrayNotificationFlags as f_tray;
    let mut msg_title  :String = "".to_string();
    let mut msg_content:String = "".to_string();
    let mut flags:f_tray = f_tray::empty();
    if let Some(cfg) = CFG.get() {let mut k = cfg.lock();
      let paths = &k.cfg_paths; let path_cur = &paths[0]; msg_content += &path_cur.display().to_string();
      let cfg_name = &path_cur.file_name().unwrap_or_else(||OsStr::new("")).to_string_lossy().to_string();
      if k.request_live_reload().is_err() {
        msg_title      +=&("✗ \"".to_owned() + cfg_name + "\" NOT reloaded");warn!("{}", msg_title); flags |= f_tray::ERROR_ICON;
      } else {msg_title+=&("🔄 \"".to_owned() + cfg_name + "\" reloaded")   ;info!("{}", msg_title)}; flags |= f_tray::USER_ICON;
    }   else {msg_title+="✗ Config NOT reloaded, no CFG";warn!("{}", msg_title); flags |= f_tray::ERROR_ICON;
    };
    flags |= f_tray::LARGE_ICON; // todo: fails without this, must have SM_CXICON x SM_CYICON?
    self.tray.show(&msg_content, Some(&msg_title), Some(flags), Some(&self.icon));}
  fn exit(&self) {
    let handlers = self.handlers_dyn.borrow();
    for handler in handlers.iter() {nwg::unbind_event_handler(&handler);}
    nwg::stop_thread_dispatch();}
}

pub mod system_tray_ui {
  use super::*;
  use core::cmp;
  use std::rc::Rc;
  use std::cell::RefCell;
  use std::ops::{Deref, DerefMut};
  use native_windows_gui::{self as nwg, MousePressEvent};

  pub struct SystemTrayUi {
    inner      	: Rc<SystemTray>,
    handler_def	: RefCell<Vec<nwg::EventHandler>>
  }

  impl nwg::NativeUi<SystemTrayUi> for SystemTray {
    fn build_ui(mut d: SystemTray) -> Result<SystemTrayUi, nwg::NwgError> {
      use nwg::Event as E;

      let app_data = d.app_data.borrow().clone();
      // d.app_data  	= RefCell::new(Default::default());
      d.tray_item_dyn	=	RefCell::new(Default::default());
      d.handlers_dyn 	=	RefCell::new(Default::default());
      // Resources
      d.embed	= Default::default();
      d.embed	= nwg::EmbedResource::load(Some("kanata.exe"))?;
      nwg::Icon::builder().source_embed(Some(&d.embed)).source_embed_str(Some("iconMain")).strict(true)/*use sys, not panic, if missing*/
        .build(&mut d.icon)?;


      // Controls
      nwg::MessageWindow   	::builder()
        .                  	  build(       &mut d.window      	)?                          	;
      nwg::TrayNotification	::builder().parent(&d.window)     	.icon(Some(&d.icon))        	.tip(Some(&app_data.tooltip))
        .                  	  build(       &mut d.tray        	)?                          	;
      nwg::Menu            	::builder().parent(&d.window)     	.popup(true)/*context menu*/	//
        .                  	  build(       &mut d.tray_menu   	)?                          	;
      nwg::Menu            	::builder().parent(&d.tray_menu)  	.text("&F Load config")     	//
        .                  	  build(       &mut d.tray_1cfg_m 	)?                          	;
      nwg::MenuItem        	::builder().parent(&d.tray_menu)  	.text("&R Reload config")   	//
        .                  	  build(       &mut d.tray_2reload	)?                          	;
      nwg::MenuItem        	::builder().parent(&d.tray_menu)  	.text("&X Exit\t‹⎈␠⎋")      	//
        .                  	  build(       &mut d.tray_3exit  	)?                          	;

      {let mut tray_item_dyn	= d.tray_item_dyn.borrow_mut(); //extra scope to drop borrowed mut
      const menu_acc:&str = "ASDFGQWERTZXCVBYUIOPHJKLNM";
      if (app_data.cfg_p).len() > 0 {
        for (i, cfg_p) in app_data.cfg_p.iter().enumerate() {
          let i_acc = match i { // menu accelerators from 1–0 then A–Z starting from home row for easier presses
            0..= 8	=> format!("&{} ",i+1),
            9     	=> format!("&{} ",0),
           10..=35	=> format!("&{} ",&menu_acc[(i-10)..cmp::min(i-10+1,menu_acc.len())]),
            _     	=> format!("  "),
          };
          let cfg_name = &cfg_p.file_name().unwrap_or_else(||OsStr::new("")).to_string_lossy().to_string(); //kanata.kbd
          // let menu_text	= i_acc + cfg_name; // &1 kanata.kbd
          let menu_text   	= format!("{cfg_name}\t{i_acc}"); // kanata.kbd &1
          let mut menu_item = Default::default();
          if i == 0	{nwg::MenuItem::builder().parent(&d.tray_1cfg_m).text(&menu_text).check(true)	.build(&mut menu_item)?;
          } else   	{nwg::MenuItem::builder().parent(&d.tray_1cfg_m).text(&menu_text)            	.build(&mut menu_item)?;
          }
          tray_item_dyn.push(menu_item);
        }
      } else {warn!("Didn't get any config paths from Kanata!")}
      }

      let ui = SystemTrayUi { // Wrap-up
        inner      	: Rc::new(d),
        handler_def	: Default::default(),
      };

      let evt_ui = Rc::downgrade(&ui.inner); // Events
      let handle_events = move |evt, _evt_data, handle| {
        if let Some(evt_ui) = evt_ui.upgrade() {
          match evt {
            E::OnWindowClose                                  	=> if &handle == &evt_ui.window {SystemTray::exit  (&evt_ui);}
            E::OnMousePress(MousePressEvent::MousePressLeftUp)	=> if &handle == &evt_ui.tray {SystemTray::show_menu(&evt_ui);}
            E::OnContextMenu/*🖰›*/                            	=> if &handle == &evt_ui.tray {SystemTray::show_menu(&evt_ui);}
            E::OnMenuItemSelected =>
              if        &handle == &evt_ui.tray_1cfg_m 	{SystemTray::load_cfg(&evt_ui);
              } else if &handle == &evt_ui.tray_2reload	{SystemTray::reload(&evt_ui);
              } else if &handle == &evt_ui.tray_3exit  	{SystemTray::exit  (&evt_ui);
              },
            _ => {}
          }
        }
      };
      ui.handler_def.borrow_mut().push(nwg::full_bind_event_handler(&ui.window.handle, handle_events));
      return Ok(ui);
    }
  }

  impl Drop for SystemTrayUi { /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
      let mut handlers = self.handler_def.borrow_mut();
      for handler in handlers.drain(0..) {nwg::unbind_event_handler(&handler);}
    }
  }
  impl Deref    for SystemTrayUi {type Target = SystemTray;fn deref    (&    self) -> &    Self::Target {&    self.inner}}
}

pub fn build_tray(cfg: &Arc<Mutex<Kanata>>) -> Result<system_tray_ui::SystemTrayUi> {
  let k       	= cfg.lock();
  let paths   	= &k.cfg_paths;
  let path_cur	= &paths[0];
  let app_data	= SystemTrayData {
    tooltip   	: path_cur.display().to_string(),
    cfg_p     	: paths.clone()};
  let app     	= SystemTray {app_data:RefCell::new(app_data), ..Default::default()};
  SystemTray::build_ui(app).context("Failed to build UI")
}

pub use log::*;
pub use winapi::um::wincon::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS};
pub use winapi::shared::minwindef::BOOL;
pub use std::io::{stdout, IsTerminal};

use once_cell::sync::Lazy;
pub static IS_TERM:Lazy<bool> = Lazy::new(||stdout().is_terminal());
pub static IS_CONSOLE:Lazy<bool> = Lazy::new(|| unsafe{
  if AttachConsole(ATTACH_PARENT_PROCESS)== 0i32 {return false} else {return true}});
