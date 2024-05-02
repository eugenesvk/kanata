#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]
// based on https://github.com/lynxnb/wsl-usb-manager/blob/master/src/gui/nwg_ext.rs
use native_windows_gui as nwg;

use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Graphics::Gdi::{DeleteObject,HBRUSH};
use windows_sys::Win32::UI::Shell::{
    SHGetStockIconInfo, SHGSI_ICON, SHGSI_SMALLICON, SHSTOCKICONID, SHSTOCKICONINFO,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
  CopyImage, DestroyIcon, GetIconInfoExW, SetMenuInfo, SetMenuItemInfoW, SetMenuItemBitmaps,
  HMENU, ICONINFOEXW, IMAGE_BITMAP,
  LR_CREATEDIBSECTION, MENUINFO, MENUITEMINFOW, MF_BYCOMMAND, MIIM_BITMAP,MIM_STYLE,MIM_BACKGROUND
    ,MNS_NOTIFYBYPOS,MNS_CHECKORBMP,MNS_AUTODISMISS,MF_BYPOSITION,MENU_ITEM_FLAGS
};

/// Extends [`nwg::Bitmap`] with additional functionality.
pub trait BitmapEx {
    fn from_system_icon(icon: SHSTOCKICONID) -> nwg::Bitmap;
}

impl BitmapEx for nwg::Bitmap {
    /// Creates a bitmap from a [`SHSTOCKICONID`] system icon ID.
    fn from_system_icon(icon: SHSTOCKICONID) -> nwg::Bitmap {
        // Retrieve the icon
        let mut stock_icon_info = SHSTOCKICONINFO {
            cbSize: std::mem::size_of::<SHSTOCKICONINFO>() as u32,
            hIcon: 0,
            iSysImageIndex: 0,
            iIcon: 0,
            szPath: [0; 260],
        };
        unsafe {
            SHGetStockIconInfo(
                icon,
                SHGSI_ICON | SHGSI_SMALLICON,
                &mut stock_icon_info as *mut _,
            );
        }

        // Retrieve the bitmap for the icon
        let mut icon_info = ICONINFOEXW {
            cbSize: std::mem::size_of::<ICONINFOEXW>() as u32,
            fIcon: 0,
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: 0,
            hbmColor: 0,
            wResID: 0,
            szModName: [0; 260],
            szResName: [0; 260],
        };
        unsafe {
            GetIconInfoExW(stock_icon_info.hIcon, &mut icon_info as *mut _);
        }

        // Create a copy of the bitmap with transparent background from the icon bitmap
        let hbitmap = unsafe {
            CopyImage(
                icon_info.hbmColor as HANDLE,
                IMAGE_BITMAP,
                0,
                0,
                LR_CREATEDIBSECTION,
            )
        };

        // Delete the unused icon and bitmaps
        unsafe {
            DeleteObject(icon_info.hbmMask);
            DeleteObject(icon_info.hbmColor);
            DestroyIcon(stock_icon_info.hIcon);
        };

        if hbitmap == 0 {
            panic!("Failed to create bitmap from system icon");
        } else {
            #[allow(unused)]
            struct Bitmap {
                handle: HANDLE,
                owned: bool,
            }

            let bitmap = Bitmap {
                handle: hbitmap as HANDLE,
                owned: true,
            };

            // Ugly hack to set the private `owned` field inside nwg::Bitmap to true
            unsafe { std::mem::transmute(bitmap) }
        }
    }
}

/// Extends [`nwg::Menu`] with additional functionality.
pub trait MenuEx {fn set_bitmap(&self, bitmap: Option<&nwg::Bitmap>);}
impl MenuEx for nwg::Menu { /// Sets a bitmap to be displayed on a menu. Pass `None` to remove the bitmap
  fn set_bitmap(&self, bitmap: Option<&nwg::Bitmap>) {
    let (hmenu_par, hmenu) = self.handle.hmenu().unwrap();
    let hbitmap = match bitmap {Some(b) => b.handle as HANDLE, None => 0,};
    let menu_info = MENUINFO {
      cbSize                	: std::mem::size_of::<MENUINFO>() as u32,
      fMask                 	: MIM_BACKGROUND	,//
       //MIM_STYLE          	get/set member dwStyle
       //MIM_BACKGROUND     	get/set member hbrBack
       //MIM_HELPID         	get/set member dwContextHelpID
       //MIM_MAXHEIGHT      	get/set member cyMax
       //MIM_MENUDATA       	get/set member dwMenuData
       //MIM_APPLYTOSUBMENUS	Settings apply to the menu and all of its submenus. SetMenuInfo uses this flag and GetMenuInfo ignores this flag
      dwStyle               	: MNS_NOTIFYBYPOS	,//
       //MNS_NOTIFYBYPOS    	Menu owner receives WM_MENUCOMMAND instead of WM_COMMAND message when the user makes a selection. (header style, no effect on individual sub menus)
       //MNS_AUTODISMISS    	Menu automatically ends when mouse is outside the menu for approximately 10 seconds.
       //MNS_CHECKORBMP     	Same space is reserved for check mark & bitmap, which are aligned and mutually exclusive (check beats bit). Used for menus where some items use checkmarks and some use bitmaps
       //MNS_NOCHECK        	No   space is reserved to the left of an item for a check mark. The item can still be selected, but the check mark will not appear next to the item.
       //MNS_MODELESS       	No menu modal message loop while active
       //MNS_DRAGDROP       	Menu items are OLE drop targets or drag sources. Menu owner receives WM_MENUDRAG and WM_MENUGETOBJECT messages
      cyMax                 	: 0                	,//|0| pixels  max menu↕ (0=screen↕), if exceeds, use scroll bars
      hbrBack               	: hbitmap as HBRUSH	,//handle to the brush to be used for the menu's background
      dwContextHelpID       	: 0                	,//context help identifier
      dwMenuData            	: 0                	,//application-defined value
    };
    unsafe {SetMenuInfo(hmenu as HMENU, &menu_info as *const _,);}

    // unsafe {SetMenuItemBitmaps(
    //   hmenu        	as HMENU,
    //   0,           	// uposition                        	as u32,
    //   MF_BYPOSITION	as MENU_ITEM_FLAGS,                 	// uflags	as MENU_ITEM_FLAGS,
    //   hbitmap      	, // as *const _, //hbitmapunchecked	         	as HBITMAP,
    //   hbitmap      	, // as *const _, //hbitmapchecked  	         	as HBITMAP
    // );}
  }
}


/// Extends [`nwg::MenuItem`] with additional functionality.
pub trait MenuItemEx {
    fn set_bitmap(&self, bitmap: Option<&nwg::Bitmap>);
}

impl MenuItemEx for nwg::MenuItem {
    /// Sets a bitmap to be displayed on a menu item. Pass `None` to remove the bitmap.
    fn set_bitmap(&self, bitmap: Option<&nwg::Bitmap>) {
        let (hmenu, item_id) = self.handle.hmenu_item().unwrap();
        let hbitmap = match bitmap {
            Some(b) => b.handle as HANDLE,
            None => 0,
        };

        let menu_item_info = MENUITEMINFOW {
            cbSize: std::mem::size_of::<MENUITEMINFOW>() as u32,
            fMask: MIIM_BITMAP,
            fType: 0,
            fState: 0,
            wID: 0,
            hSubMenu: 0,
            hbmpChecked: 0,
            hbmpUnchecked: 0,
            dwItemData: 0,
            dwTypeData: std::ptr::null_mut(),
            cch: 0,
            hbmpItem: hbitmap,
        };

        unsafe {
            SetMenuItemInfoW(
                hmenu as HMENU,
                item_id,
                MF_BYCOMMAND as i32,
                &menu_item_info as *const _,
            );
        }
    }
}
