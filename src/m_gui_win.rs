#![allow(unused_imports,unused_variables,unreachable_code,dead_code,non_upper_case_globals)]
// #![allow(non_upper_case_globals)]

use std::env::{var_os,current_exe};
use std::path::{Path,PathBuf};
use std::collections::HashMap;
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
  pub icon_dyn     	: RefCell<HashMap<PathBuf,nwg::Icon>>,
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
pub fn get_appdata() -> Option<PathBuf> {var_os("APPDATA").map(PathBuf::from)}
pub fn get_user_home() -> Option<PathBuf> {var_os("USERPROFILE").map(PathBuf::from)}
pub fn get_xdg_home() -> Option<PathBuf> {var_os("XDG_CONFIG_HOME").map(PathBuf::from)}

use crate::lib_main::CFG;
use winapi::shared::windef::{HWND, HMENU};
impl SystemTray {
  fn show_menu(&self) {
    let (x, y) = nwg::GlobalCursor::position();
    self.tray_menu.popup(x, y);  }
  fn get_icon_p<P: AsRef<Path>>(&self, s:P    ) -> Option<String> {self.get_icon_p_impl(s.as_ref())}
  fn get_icon_p_impl           (&self, p:&Path) -> Option<String> {
    // let mut icon_file:PathBuf = [pre,name,ext].iter().collect();
    // info!("searching for an icon in path=¦{:?}¦ n=¦{:?}¦ ext=¦{:?}¦ pre=¦{:?}¦",p,name,ext,pre);
    let mut icon_file = PathBuf::from(p);
    icon_file  .set_extension("ico");
    if ! icon_file.is_file() {icon_file.clear();icon_file.push(p);
      icon_file.set_extension("kbd.ico");}
    if ! icon_file.is_file() {icon_file.clear();
      // let pre_p	= &p.parent()   .unwrap_or_else(||Path::new(""));
      let blank   	= Path::new("");
      let nameext 	= &p.file_name().unwrap_or_else(||OsStr::new(""));
      // let ext  	= &p.extension().unwrap_or_else(||OsStr::new(""));
      // let name 	= &p.file_stem().unwrap_or_else(||OsStr::new(""));
      let     cur_exe  = current_exe  ().unwrap_or_else(|_|PathBuf::new());
      let     xdg_cfg  = get_xdg_home ().unwrap_or_else(||PathBuf::new());
      let     app_data = get_appdata  ().unwrap_or_else(||PathBuf::new());
      let mut user_cfg = get_user_home().unwrap_or_else(||PathBuf::new());
      user_cfg.push(".config");
      'p: for  p_par in [cur_exe,xdg_cfg,app_data,user_cfg] {
        'k:for p_kan in ["kanata","kanata-tray"] {
          for  p_icn in ["","icon","img","icons"] {
            if ! (p_par == blank) {icon_file.push(p_par.clone());} else {break 'k}
            icon_file.push(p_kan);icon_file.push(p_icn); // folders
            icon_file.push(nameext); //file
            icon_file.set_extension("ico");info!("testing icon from {:?}",icon_file);
            if ! icon_file.is_file() {icon_file.clear();} else {break 'p} }  }     }
    }
    if ! icon_file.is_file() {info!("✗ no icon file found");
      return None
    } else {info!("✓ found icon file at: {}",icon_file.display().to_string());
      return Some(icon_file.display().to_string())}
  }
  fn check_active(&self) {
    if let Some(cfg) = CFG.get() {let mut k = cfg.lock();
      let idx_cfg = k.cur_cfg_idx;
      let tray_item_dyn	= &self.tray_item_dyn.borrow(); //
      for (i, h_cfg_i) in tray_item_dyn.iter().enumerate() {
        if   h_cfg_i.checked(){trace!("✓checked {} active {} eq? {} !eq? {}",i,idx_cfg,i==idx_cfg,!(i==idx_cfg));}
        if   h_cfg_i.checked() && !(i==idx_cfg){debug!("uncheck i{} act{}",i,idx_cfg);h_cfg_i.set_checked(false);} // uncheck inactive
        if ! h_cfg_i.checked() &&   i==idx_cfg {debug!("  check i{} act{}",i,idx_cfg);h_cfg_i.set_checked(true );} //   check   active
      };
    } else {error!("no CFG var that contains active kanata config");
    };
  }
  fn reload(&self,i:Option<usize>) {
    use nwg::TrayNotificationFlags as f_tray;
    let mut msg_title  :String = "".to_string();
    let mut msg_content:String = "".to_string();
    let mut flags:f_tray = f_tray::empty();
    if let Some(cfg) = CFG.get() {let mut k = cfg.lock();
      let paths = &k.cfg_paths;
      let idx_cfg = match i {
        Some(idx)	=> {if idx<paths.len() {idx} else {error!("Invalid config index {} while kanata has only {} configs loaded",idx+1,paths.len());k.cur_cfg_idx}},
        None     	=> k.cur_cfg_idx};
      let path_cur = &paths[idx_cfg]; let path_cur_s = path_cur.display().to_string();
      let path_cur_cc = path_cur.clone();
      msg_content += &path_cur_s;
      let cfg_name = &path_cur.file_name().unwrap_or_else(||OsStr::new("")).to_string_lossy().to_string();
      match i {
        Some(idx)	=> {k.request_live_reload_n(idx); msg_title+=&("🔄 \"".to_owned() + cfg_name + "\" reloaded"); flags |= f_tray::USER_ICON;}
        None     	=> {k.request_live_reload  (   ); msg_title+=&("🔄 \"".to_owned() + cfg_name + "\" reloaded"); flags |= f_tray::USER_ICON;}
      };
      info!("{}", msg_title);
      // self.tray.set_visibility(false); // flash the icon, but might be confusing as the app isn't restarting, just reloading
      self.tray.set_tip(&path_cur_s); // update tooltip to point to the newer config
      // self.tray.set_visibility(true);

      let icon_dyn = self.icon_dyn.borrow();
      if let Some(icon) = icon_dyn.get(&path_cur_cc) {self.tray.set_icon(&icon); //
      } else { warn!("couldn't set icon to test {}",path_cur_cc.display().to_string());}
    }   else {msg_title+="✗ Config NOT reloaded, no CFG";warn!("{}", msg_title); flags |= f_tray::ERROR_ICON;
    };
    flags |= f_tray::LARGE_ICON; // todo: fails without this, must have SM_CXICON x SM_CYICON?
    self.tray.show(&msg_content, Some(&msg_title), Some(flags), Some(&self.icon));
  }
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
       let mut icon_dyn       = d.icon_dyn     .borrow_mut();
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
          tray_item_dyn.push(menu_item); // add icons if exists, hashed by config path
          if i == 0	{
            let mut temp_icon = Default::default();
            nwg::Icon::builder().source_embed(Some(&d.embed)).source_embed_str(Some("iconMain")).strict(true).build(&mut temp_icon).unwrap();
            let _ = icon_dyn.insert(cfg_p.clone(),temp_icon);
          } else if let Some(ico_p) = &d.get_icon_p(&cfg_p) {
            let mut temp_icon = Default::default();
            nwg::Icon::builder().source_file(Some(&ico_p)).strict(false).build(&mut temp_icon).unwrap();
            let _ = icon_dyn.insert(cfg_p.clone(),temp_icon);
          }
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
            E::OnMenuHover =>
              if        &handle == &evt_ui.tray_1cfg_m	{SystemTray::check_active(&evt_ui);}
            E::OnMenuItemSelected =>
              if        &handle == &evt_ui.tray_2reload	{SystemTray::reload(&evt_ui,None);
              } else if &handle == &evt_ui.tray_3exit  	{SystemTray::exit  (&evt_ui);
              } else {
                match handle {
                  ControlHandle::MenuItem(parent, id) => {
                    let tray_item_dyn	= &evt_ui.tray_item_dyn.borrow(); //
                    for (i, h_cfg) in tray_item_dyn.iter().enumerate() {
                      if &handle == h_cfg { //info!("CONFIG handle i={:?} {:?}",i,&handle);
                        for (j, h_cfg_j) in tray_item_dyn.iter().enumerate() {
                          if h_cfg_j.checked() {h_cfg_j.set_checked(false);} } // uncheck others
                        h_cfg.set_checked(true); // check self
                        SystemTray::reload(&evt_ui,Some(i));
                      }
                    }
                  },
                  _	=> {},
                }
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
