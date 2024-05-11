use core::ffi::c_int;
use crate::gui::win::HWND;
use winapi::shared::minwindef::{BYTE,DWORD,UINT};
use winapi::shared::windef::COLORREF;
use winapi::shared::basetsd::LONG_PTR;

#[inline(always)] #[cfg(target_pointer_width = "64")]
pub fn get_window_long(handle: HWND, index: c_int) -> LONG_PTR {
  unsafe{ ::winapi::um::winuser::GetWindowLongPtrW(handle, index) } }
#[inline(always)] #[cfg(target_pointer_width = "64")]
pub fn set_window_long(handle: HWND, index: c_int, v: usize) {
  unsafe{ ::winapi::um::winuser::SetWindowLongPtrW(handle, index, v as LONG_PTR); }}
pub fn set_style(handle: HWND, style: u32) {use ::winapi::um::winuser::GWL_STYLE;
  set_window_long(handle, GWL_STYLE, style as usize);}
pub fn get_style(handle: HWND) -> UINT {use ::winapi::um::winuser::GWL_STYLE;
  get_window_long(handle, GWL_STYLE) as UINT}
pub fn set_window_visibility(handle:HWND, visible:bool) {
  use winapi::um::winuser::ShowWindow;
  use winapi::um::winuser::{SW_HIDE, SW_SHOW};
  let visible = if visible {SW_SHOW} else {SW_HIDE};
  unsafe{ShowWindow(handle, visible)};
}
