// This file is adapted from the original ktrl's `keys.rs` file for Windows.
use super::OsCode;

#[allow(unused)] mod keys { // modified from github.com/microsoft/windows-rs/blob/0.55.0/crates/libs/sys/src/Windows/Win32/UI/Input/KeyboardAndMouse/mod.rs
  // Category             Count  List
  // Reserved             #  6   7 10 11                            94                                     184 185
  // Unassigned/undefined # 19           14 15 58 59 60 61 62 63 64    151 152 153 154 155 156 157 158 159         232
  // Assigned             #230
  // Total                #255
  // 232 unassigned used as a `VK_KPENTER_FAKE`
  pub type VirtualKey = u16; // 247 from previous 197
  pub const vk_kpenter_fake	: VirtualKey = 232u16; // unassigned VK used to enable Interception use
  // Modifiers and sp
  pub const vk_lcontrol	:VirtualKey = 162u16;pub const vk_rcontrol	:VirtualKey = 163u16;pub const vk_control	:VirtualKey =  17u16;
  pub const vk_lshift  	:VirtualKey = 160u16;pub const vk_rshift  	:VirtualKey = 161u16;pub const vk_shift  	:VirtualKey =  16u16;
  pub const vk_lwin    	:VirtualKey =  91u16;pub const vk_rwin    	:VirtualKey =  92u16;
  pub const vk_rmenu   	:VirtualKey = 165u16;
  // Alphanumeric
  pub const vk0 	:VirtualKey =  48u16;pub const vk1 	:VirtualKey =  49u16;pub const vk2 	:VirtualKey =  50u16;pub const vk3 	:VirtualKey =  51u16;pub const vk4 	:VirtualKey =  52u16;pub const vk5 	:VirtualKey =  53u16;pub const vk6 	:VirtualKey =  54u16;pub const vk7 	:VirtualKey =  55u16;pub const vk8 	:VirtualKey =  56u16;pub const vk9 	:VirtualKey =  57u16;
  pub const vk_a	:VirtualKey =  65u16;pub const vk_b	:VirtualKey =  66u16;pub const vk_c	:VirtualKey =  67u16;pub const vk_d	:VirtualKey =  68u16;pub const vk_e	:VirtualKey =  69u16;pub const vk_f	:VirtualKey =  70u16;pub const vk_g	:VirtualKey =  71u16;pub const vk_h	:VirtualKey =  72u16;pub const vk_i	:VirtualKey =  73u16;pub const vk_j	:VirtualKey =  74u16;pub const vk_k	:VirtualKey =  75u16;pub const vk_l	:VirtualKey =  76u16;pub const vk_o	:VirtualKey =  79u16;pub const vk_n	:VirtualKey =  78u16;pub const vk_m	:VirtualKey =  77u16;pub const vk_p	:VirtualKey =  80u16;pub const vk_q	:VirtualKey =  81u16;pub const vk_r	:VirtualKey =  82u16;pub const vk_s	:VirtualKey =  83u16;pub const vk_t	:VirtualKey =  84u16;pub const vk_u	:VirtualKey =  85u16;pub const vk_v	:VirtualKey =  86u16;pub const vk_w	:VirtualKey =  87u16;pub const vk_x	:VirtualKey =  88u16;pub const vk_y	:VirtualKey =  89u16;pub const vk_z	:VirtualKey =  90u16;
  // Num
  pub const vk_numpad0	:VirtualKey =  96u16;pub const vk_numpad1	:VirtualKey =  97u16;pub const vk_numpad2	:VirtualKey =  98u16;pub const vk_numpad3	:VirtualKey =  99u16;pub const vk_numpad4	:VirtualKey = 100u16;pub const vk_numpad5	:VirtualKey = 101u16;pub const vk_numpad6	:VirtualKey = 102u16;pub const vk_numpad7	:VirtualKey = 103u16;pub const vk_numpad8	:VirtualKey = 104u16;pub const vk_numpad9	:VirtualKey = 105u16;
  // Function
  pub const vk_f1               	:VirtualKey = 112u16;pub const vk_f2     	:VirtualKey = 113u16;pub const vk_f3 	:VirtualKey = 114u16;pub const vk_f4 	:VirtualKey = 115u16;pub const vk_f5 	:VirtualKey = 116u16;pub const vk_f6 	:VirtualKey = 117u16;pub const vk_f7 	:VirtualKey = 118u16;pub const vk_f8 	:VirtualKey = 119u16;pub const vk_f9 	:VirtualKey = 120u16;
  pub const vk_f10              	:VirtualKey = 121u16;pub const vk_f11    	:VirtualKey = 122u16;pub const vk_f12	:VirtualKey = 123u16;pub const vk_f13	:VirtualKey = 124u16;pub const vk_f14	:VirtualKey = 125u16;pub const vk_f15	:VirtualKey = 126u16;pub const vk_f16	:VirtualKey = 127u16;pub const vk_f17	:VirtualKey = 128u16;pub const vk_f18	:VirtualKey = 129u16;pub const vk_f19	:VirtualKey = 130u16;
  pub const vk_f20              	:VirtualKey = 131u16;pub const vk_f21    	:VirtualKey = 132u16;pub const vk_f22	:VirtualKey = 133u16;pub const vk_f23	:VirtualKey = 134u16;pub const vk_f24	:VirtualKey = 135u16;
  pub const vk_abnt_c1          	:VirtualKey = 193u16;pub const vk_abnt_c2	:VirtualKey = 194u16;
  pub const vk_accept           	:VirtualKey =  30u16;
  pub const vk_add              	:VirtualKey = 107u16;
  pub const vk_apps             	:VirtualKey =  93u16;
  pub const vk_attn             	:VirtualKey = 246u16;
  pub const vk_back             	:VirtualKey =   8u16;
  pub const vk_browser_back     	:VirtualKey = 166u16;pub const vk_browser_favorites	:VirtualKey = 171u16;pub const vk_browser_forward	:VirtualKey = 167u16;pub const vk_browser_home	:VirtualKey = 172u16;pub const vk_browser_refresh	:VirtualKey = 168u16;pub const vk_browser_search	:VirtualKey = 170u16;pub const vk_browser_stop	:VirtualKey = 169u16;
  pub const vk_cancel           	:VirtualKey =   3u16;
  pub const vk_capital          	:VirtualKey =  20u16;
  pub const vk_clear            	:VirtualKey =  12u16;
  pub const vk_convert          	:VirtualKey =  28u16;
  pub const vk_crsel            	:VirtualKey = 247u16;
  pub const vk_dbe_alphanumeric 	:VirtualKey = 240u16;pub const vk_dbe_codeinput	:VirtualKey = 250u16;pub const vk_dbe_dbcschar	:VirtualKey = 244u16;pub const vk_dbe_determinestring	:VirtualKey = 252u16;pub const vk_dbe_enterdlgconversionmode	:VirtualKey = 253u16;pub const vk_dbe_enterimeconfigmode	:VirtualKey = 248u16;pub const vk_dbe_enterwordregistermode	:VirtualKey = 247u16;pub const vk_dbe_flushstring	:VirtualKey = 249u16;pub const vk_dbe_hiragana	:VirtualKey = 242u16;pub const vk_dbe_katakana	:VirtualKey = 241u16;pub const vk_dbe_nocodeinput	:VirtualKey = 251u16;pub const vk_dbe_noroman	:VirtualKey = 246u16;pub const vk_dbe_roman	:VirtualKey = 245u16;pub const vk_dbe_sbcschar	:VirtualKey = 243u16;
  pub const vk_decimal          	:VirtualKey = 110u16;
  pub const vk_delete           	:VirtualKey =  46u16;
  pub const vk_divide           	:VirtualKey = 111u16;
  pub const vk_down             	:VirtualKey =  40u16;
  pub const vk_end              	:VirtualKey =  35u16;
  pub const vk_ereof            	:VirtualKey = 249u16;
  pub const vk_escape           	:VirtualKey =  27u16;
  pub const vk_execute          	:VirtualKey =  43u16;
  pub const vk_exsel            	:VirtualKey = 248u16;
  pub const vk_final            	:VirtualKey =  24u16;
  pub const vk_hangeul          	:VirtualKey =  21u16;
  pub const vk_hangul           	:VirtualKey =  21u16;
  pub const vk_hanja            	:VirtualKey =  25u16;
  pub const vk_kana             	:VirtualKey =  21u16;
  pub const vk_kanji            	:VirtualKey =  25u16;
  pub const vk_help             	:VirtualKey =  47u16;
  pub const vk_home             	:VirtualKey =  36u16;
  pub const vk_ico00            	:VirtualKey = 228u16;
  pub const vk_ico_clear        	:VirtualKey = 230u16;
  pub const vk_ico_help         	:VirtualKey = 227u16;
  pub const vk_ime_off          	:VirtualKey =  26u16;
  pub const vk_ime_on           	:VirtualKey =  22u16;
  pub const vk_insert           	:VirtualKey =  45u16;
  pub const vk_junja            	:VirtualKey =  23u16;
  pub const vk_launch_app1      	:VirtualKey = 182u16;pub const vk_launch_app2	:VirtualKey = 183u16;pub const vk_launch_mail	:VirtualKey = 180u16;pub const vk_launch_media_select	:VirtualKey = 181u16;
  pub const vk_lbutton          	:VirtualKey =   1u16;
  pub const vk_left             	:VirtualKey =  37u16;
  pub const vk_lmenu            	:VirtualKey = 164u16;
  pub const vk_mbutton          	:VirtualKey =   4u16;
  pub const vk_media_stop       	:VirtualKey = 178u16;pub const vk_media_play_pause	:VirtualKey = 179u16;
  pub const vk_media_prev_track 	:VirtualKey = 177u16;pub const vk_media_next_track	:VirtualKey = 176u16;
  pub const vk_menu             	:VirtualKey =  18u16;
  pub const vk_modechange       	:VirtualKey =  31u16;
  pub const vk_multiply         	:VirtualKey = 106u16;
  pub const vk_navigation_accept	:VirtualKey = 142u16;pub const vk_navigation_cancel	:VirtualKey = 143u16;pub const vk_navigation_menu	:VirtualKey = 137u16;pub const vk_navigation_view 	:VirtualKey = 136u16;
  pub const vk_navigation_down  	:VirtualKey = 139u16;pub const vk_navigation_up    	:VirtualKey = 138u16;pub const vk_navigation_left	:VirtualKey = 140u16;pub const vk_navigation_right	:VirtualKey = 141u16;
  pub const vk_next             	:VirtualKey =  34u16;
  pub const vk_noname           	:VirtualKey = 252u16;
  pub const vk_nonconvert       	:VirtualKey =  29u16;
  pub const vk_numlock          	:VirtualKey = 144u16;
  pub const vk_oem102           	:VirtualKey = 226u16;
  pub const vk_oem1             	:VirtualKey = 186u16;pub const vk_oem2	:VirtualKey = 191u16;pub const vk_oem3	:VirtualKey = 192u16;pub const vk_oem4	:VirtualKey = 219u16;pub const vk_oem5	:VirtualKey = 220u16;pub const vk_oem6	:VirtualKey = 221u16;pub const vk_oem7	:VirtualKey = 222u16;pub const vk_oem8	:VirtualKey = 223u16;
  pub const vk_oem_attn         	:VirtualKey = 240u16;
  pub const vk_oem_auto         	:VirtualKey = 243u16;
  pub const vk_oem_ax           	:VirtualKey = 225u16;
  pub const vk_oem_backtab      	:VirtualKey = 245u16;
  pub const vk_oem_clear        	:VirtualKey = 254u16;
  pub const vk_oem_comma        	:VirtualKey = 188u16;
  pub const vk_oem_copy         	:VirtualKey = 242u16;
  pub const vk_oem_cusel        	:VirtualKey = 239u16;
  pub const vk_oem_enlw         	:VirtualKey = 244u16;
  pub const vk_oem_finish       	:VirtualKey = 241u16;
  pub const vk_oem_fj_jisho     	:VirtualKey = 146u16;
  pub const vk_oem_fj_loya      	:VirtualKey = 149u16;
  pub const vk_oem_fj_masshou   	:VirtualKey = 147u16;
  pub const vk_oem_fj_roya      	:VirtualKey = 150u16;
  pub const vk_oem_fj_touroku   	:VirtualKey = 148u16;
  pub const vk_oem_jump         	:VirtualKey = 234u16;
  pub const vk_oem_minus        	:VirtualKey = 189u16;
  pub const vk_oem_nec_equal    	:VirtualKey = 146u16;
  pub const vk_oem_pa1          	:VirtualKey = 235u16;pub const vk_oem_pa2	:VirtualKey = 236u16;pub const vk_oem_pa3	:VirtualKey = 237u16;
  pub const vk_oem_period       	:VirtualKey = 190u16;
  pub const vk_oem_plus         	:VirtualKey = 187u16;
  pub const vk_oem_reset        	:VirtualKey = 233u16;
  pub const vk_oem_wsctrl       	:VirtualKey = 238u16;
  pub const vk_pa1              	:VirtualKey = 253u16;
  pub const vk_packet           	:VirtualKey = 231u16;
  pub const vk_pause            	:VirtualKey =  19u16;
  pub const vk_play             	:VirtualKey = 250u16;
  pub const vk_print            	:VirtualKey =  42u16;
  pub const vk_prior            	:VirtualKey =  33u16;
  pub const vk_processkey       	:VirtualKey = 229u16;
  pub const vk_rbutton          	:VirtualKey =   2u16;
  pub const vk_return           	:VirtualKey =  13u16;
  pub const vk_right            	:VirtualKey =  39u16;
  pub const vk_scroll           	:VirtualKey = 145u16;
  pub const vk_select           	:VirtualKey =  41u16;
  pub const vk_separator        	:VirtualKey = 108u16;
  pub const vk_sleep            	:VirtualKey =  95u16;
  pub const vk_snapshot         	:VirtualKey =  44u16;
  pub const vk_space            	:VirtualKey =  32u16;
  pub const vk_subtract         	:VirtualKey = 109u16;
  pub const vk_tab              	:VirtualKey =   9u16;
  pub const vk_up               	:VirtualKey =  38u16;
  pub const vk_volume_mute      	:VirtualKey = 173u16;pub const vk_volume_down	:VirtualKey = 174u16;pub const vk_volume_up	:VirtualKey = 175u16;
  pub const vk_xbutton1         	:VirtualKey =   5u16;pub const vk_xbutton2   	:VirtualKey =   6u16;
  pub const vk_zoom             	:VirtualKey = 251u16;
  pub const vk_none             	:VirtualKey = 255u16;
  // Gamepad
  pub const vk_gamepad_a                    	:VirtualKey = 195u16;pub const vk_gamepad_b                  	:VirtualKey = 196u16;pub const vk_gamepad_x	:VirtualKey = 197u16;pub const vk_gamepad_y	:VirtualKey = 198u16;
  pub const vk_gamepad_menu                 	:VirtualKey = 207u16;pub const vk_gamepad_view               	:VirtualKey = 208u16;
  pub const vk_gamepad_dpad_down             	:VirtualKey = 204u16;pub const vk_gamepad_dpad_left           	:VirtualKey = 205u16;pub const vk_gamepad_dpad_right	:VirtualKey = 206u16;pub const vk_gamepad_dpad_up	:VirtualKey = 203u16;
  pub const vk_gamepad_left_trigger          	:VirtualKey = 201u16;pub const vk_gamepad_right_trigger       	:VirtualKey = 202u16;
  pub const vk_gamepad_left_shoulder         	:VirtualKey = 200u16;pub const vk_gamepad_right_shoulder      	:VirtualKey = 199u16;
  pub const vk_gamepad_left_thumbstick_button 	:VirtualKey = 209u16;pub const vk_gamepad_left_thumbstick_down 	:VirtualKey = 212u16;pub const vk_gamepad_left_thumbstick_left 	:VirtualKey = 214u16;pub const vk_gamepad_left_thumbstick_right 	:VirtualKey = 213u16;pub const vk_gamepad_left_thumbstick_up 	:VirtualKey = 211u16;
  pub const vk_gamepad_right_thumbstick_button	:VirtualKey = 210u16;pub const vk_gamepad_right_thumbstick_down	:VirtualKey = 216u16;pub const vk_gamepad_right_thumbstick_left	:VirtualKey = 218u16;pub const vk_gamepad_right_thumbstick_right	:VirtualKey = 217u16;pub const vk_gamepad_right_thumbstick_up	:VirtualKey = 215u16;
}
pub use keys::*;

impl OsCode {
  pub(super) const fn from_u16_windows(code	:u16) -> Option<Self> {
    match code {
      0x30                  	=> Some(OsCode::KEY_0),
      0x31                  	=> Some(OsCode::KEY_1),0x32           	=> Some(OsCode::KEY_2),0x33       	=> Some(OsCode::KEY_3),0x34       	=> Some(OsCode::KEY_4),0x35           	=> Some(OsCode::KEY_5),0x36           	=> Some(OsCode::KEY_6),0x37	=> Some(OsCode::KEY_7),0x38	=> Some(OsCode::KEY_8),0x39	=> Some(OsCode::KEY_9),
      0x41                  	=> Some(OsCode::KEY_A),0x42           	=> Some(OsCode::KEY_B),0x43       	=> Some(OsCode::KEY_C),0x44       	=> Some(OsCode::KEY_D),0x45           	=> Some(OsCode::KEY_E),0x46           	=> Some(OsCode::KEY_F),0x47	=> Some(OsCode::KEY_G),0x48	=> Some(OsCode::KEY_H),0x49	=> Some(OsCode::KEY_I),0x4A	=> Some(OsCode::KEY_J),
      0x4B                  	=> Some(OsCode::KEY_K),0x4C           	=> Some(OsCode::KEY_L),0x4D       	=> Some(OsCode::KEY_M),0x4E       	=> Some(OsCode::KEY_N),0x4F           	=> Some(OsCode::KEY_O),0x50           	=> Some(OsCode::KEY_P),0x51	=> Some(OsCode::KEY_Q),0x52	=> Some(OsCode::KEY_R),0x53	=> Some(OsCode::KEY_S),0x54	=> Some(OsCode::KEY_T),
      0x55                  	=> Some(OsCode::KEY_U),0x56           	=> Some(OsCode::KEY_V),0x57       	=> Some(OsCode::KEY_W),0x58       	=> Some(OsCode::KEY_X),0x59           	=> Some(OsCode::KEY_Y),0x5A           	=> Some(OsCode::KEY_Z),
      vk_oem1               	=> Some(OsCode::KEY_SEMICOLON),vk_oem2	=> Some(OsCode::KEY_SLASH),vk_oem3	=> Some(OsCode::KEY_GRAVE),vk_oem4	=> Some(OsCode::KEY_LEFTBRACE),vk_oem5	=> Some(OsCode::KEY_BACKSLASH),vk_oem6	=> Some(OsCode::KEY_RIGHTBRACE),vk_oem7	=> Some(OsCode::KEY_APOSTROPHE),
      vk_oem_minus          	=> Some(OsCode::KEY_MINUS),
      vk_oem_period         	=> Some(OsCode::KEY_DOT),
      vk_oem_plus           	=> Some(OsCode::KEY_EQUAL),
      vk_back               	=> Some(OsCode::KEY_BACKSPACE),
      vk_escape             	=> Some(OsCode::KEY_ESC),
      vk_tab                	=> Some(OsCode::KEY_TAB),
      vk_return             	=> Some(OsCode::KEY_ENTER),
      vk_lcontrol           	=> Some(OsCode::KEY_LEFTCTRL),vk_rcontrol	=> Some(OsCode::KEY_RIGHTCTRL),
      vk_lshift             	=> Some(OsCode::KEY_LEFTSHIFT),vk_rshift 	=> Some(OsCode::KEY_RIGHTSHIFT),
      vk_hangeul            	=> Some(OsCode::KEY_HANGEUL),
      vk_hanja              	=> Some(OsCode::KEY_HANJA),
      vk_oem_comma          	=> Some(OsCode::KEY_COMMA),
      vk_multiply           	=> Some(OsCode::KEY_KPASTERISK),
      vk_lmenu              	=> Some(OsCode::KEY_LEFTALT),
      vk_space              	=> Some(OsCode::KEY_SPACE),
      vk_capital            	=> Some(OsCode::KEY_CAPSLOCK),
      vk_numlock            	=> Some(OsCode::KEY_NUMLOCK),
      vk_clear              	=> Some(OsCode::KEY_CLEAR),
      vk_scroll             	=> Some(OsCode::KEY_SCROLLLOCK),
      vk_numpad0            	=> Some(OsCode::KEY_KP0),
      vk_numpad1            	=> Some(OsCode::KEY_KP1),vk_numpad2	=> Some(OsCode::KEY_KP2),vk_numpad3	=> Some(OsCode::KEY_KP3),vk_numpad4	=> Some(OsCode::KEY_KP4),vk_numpad5	=> Some(OsCode::KEY_KP5),vk_numpad6	=> Some(OsCode::KEY_KP6),vk_numpad7	=> Some(OsCode::KEY_KP7),vk_numpad8	=> Some(OsCode::KEY_KP8),vk_numpad9	=> Some(OsCode::KEY_KP9),
      vk_subtract           	=> Some(OsCode::KEY_KPMINUS),
      vk_add                	=> Some(OsCode::KEY_KPPLUS),
      vk_decimal            	=> Some(OsCode::KEY_KPDOT),
      vk_none               	=> Some(OsCode::KEY_KPENTER),
      vk_divide             	=> Some(OsCode::KEY_KPSLASH),
      vk_rmenu              	=> Some(OsCode::KEY_RIGHTALT),
      vk_home               	=> Some(OsCode::KEY_HOME),
      vk_up                 	=> Some(OsCode::KEY_UP),
      vk_prior              	=> Some(OsCode::KEY_PAGEUP),
      vk_left               	=> Some(OsCode::KEY_LEFT),
      vk_right              	=> Some(OsCode::KEY_RIGHT),
      vk_end                	=> Some(OsCode::KEY_END),
      vk_down               	=> Some(OsCode::KEY_DOWN),
      vk_next               	=> Some(OsCode::KEY_PAGEDOWN),
      vk_insert             	=> Some(OsCode::KEY_INSERT),
      vk_delete             	=> Some(OsCode::KEY_DELETE),
      vk_volume_mute        	=> Some(OsCode::KEY_MUTE),
      vk_volume_down        	=> Some(OsCode::KEY_VOLUMEDOWN),
      vk_volume_up          	=> Some(OsCode::KEY_VOLUMEUP),
      vk_pause              	=> Some(OsCode::KEY_PAUSE),
      vk_lwin               	=> Some(OsCode::KEY_LEFTMETA),
      vk_rwin               	=> Some(OsCode::KEY_RIGHTMETA),
      vk_apps               	=> Some(OsCode::KEY_COMPOSE),
      vk_browser_back       	=> Some(OsCode::KEY_BACK),
      vk_browser_forward    	=> Some(OsCode::KEY_FORWARD),
      vk_media_next_track   	=> Some(OsCode::KEY_NEXTSONG),
      vk_media_play_pause   	=> Some(OsCode::KEY_PLAYPAUSE),
      vk_media_prev_track   	=> Some(OsCode::KEY_PREVIOUSSONG),
      vk_media_stop         	=> Some(OsCode::KEY_STOP),
      vk_browser_home       	=> Some(OsCode::KEY_HOMEPAGE),
      vk_launch_mail        	=> Some(OsCode::KEY_MAIL),
      vk_launch_media_select	=> Some(OsCode::KEY_MEDIA),
      vk_browser_refresh    	=> Some(OsCode::KEY_REFRESH),
      vk_f1                 	=> Some(OsCode::KEY_F1),vk_f2  	=> Some(OsCode::KEY_F2),vk_f3  	=> Some(OsCode::KEY_F3),vk_f4  	=> Some(OsCode::KEY_F4),vk_f5  	=> Some(OsCode::KEY_F5),vk_f6  	=> Some(OsCode::KEY_F6),vk_f7  	=> Some(OsCode::KEY_F7),vk_f8  	=> Some(OsCode::KEY_F8),vk_f9  	=> Some(OsCode::KEY_F9),vk_f10 	=> Some(OsCode::KEY_F10),
      vk_f11                	=> Some(OsCode::KEY_F11),vk_f12	=> Some(OsCode::KEY_F12),vk_f13	=> Some(OsCode::KEY_F13),vk_f14	=> Some(OsCode::KEY_F14),vk_f15	=> Some(OsCode::KEY_F15),vk_f16	=> Some(OsCode::KEY_F16),vk_f17	=> Some(OsCode::KEY_F17),vk_f18	=> Some(OsCode::KEY_F18),vk_f19	=> Some(OsCode::KEY_F19),vk_f20	=> Some(OsCode::KEY_F20),
      vk_f21                	=> Some(OsCode::KEY_F21),vk_f22	=> Some(OsCode::KEY_F22),vk_f23	=> Some(OsCode::KEY_F23),vk_f24	=> Some(OsCode::KEY_F24),
      // KEY_252 is nonsensical, but just use it anyway. No idea what Linux OsCode this is.
      // As long as it's not an existing key and the mapping round-trips, this works fine.
      vk_oem8             	=> Some(OsCode::KEY_252),
      vk_oem102           	=> Some(OsCode::KEY_102ND),
      vk_play             	=> Some(OsCode::KEY_PLAY),
      vk_snapshot         	=> Some(OsCode::KEY_PRINT),
      vk_browser_search   	=> Some(OsCode::KEY_SEARCH),
      vk_browser_favorites	=> Some(OsCode::KEY_FAVORITES),
      0xC1                	=> Some(OsCode::KEY_RO),
      vk_convert          	=> Some(OsCode::KEY_HENKAN),
      vk_nonconvert       	=> Some(OsCode::KEY_MUHENKAN),
      256                 	=> Some(OsCode::BTN_0),
      257                 	=> Some(OsCode::BTN_1),258	=> Some(OsCode::BTN_2),259	=> Some(OsCode::BTN_3),260	=> Some(OsCode::BTN_4),261	=> Some(OsCode::BTN_5),262	=> Some(OsCode::BTN_6),263	=> Some(OsCode::BTN_7),264	=> Some(OsCode::BTN_8),265	=> Some(OsCode::BTN_9),
      266                 	=> Some(OsCode::KEY_266),
      267                 	=> Some(OsCode::KEY_267),
      268                 	=> Some(OsCode::KEY_268),
      269                 	=> Some(OsCode::KEY_269),
      270                 	=> Some(OsCode::KEY_270),
      271                 	=> Some(OsCode::KEY_271),
      272                 	=> Some(OsCode::BTN_LEFT),
      273                 	=> Some(OsCode::BTN_RIGHT),
      274                 	=> Some(OsCode::BTN_MIDDLE),
      275                 	=> Some(OsCode::BTN_SIDE),
      276                 	=> Some(OsCode::BTN_EXTRA),
      277                 	=> Some(OsCode::BTN_FORWARD),
      278                 	=> Some(OsCode::BTN_BACK),
      279                 	=> Some(OsCode::BTN_TASK),
      280                 	=> Some(OsCode::KEY_280),
      281                 	=> Some(OsCode::KEY_281),
      282                 	=> Some(OsCode::KEY_282),
      283                 	=> Some(OsCode::KEY_283),
      284                 	=> Some(OsCode::KEY_284),
      285                 	=> Some(OsCode::KEY_285),
      286                 	=> Some(OsCode::KEY_286),
      287                 	=> Some(OsCode::KEY_287),
      288                 	=> Some(OsCode::BTN_TRIGGER),
      289                 	=> Some(OsCode::BTN_THUMB),
      290                 	=> Some(OsCode::BTN_THUMB2),
      291                 	=> Some(OsCode::BTN_TOP),
      292                 	=> Some(OsCode::BTN_TOP2),
      293                 	=> Some(OsCode::BTN_PINKIE),
      294                 	=> Some(OsCode::BTN_BASE),
      295                 	=> Some(OsCode::BTN_BASE2),
      296                 	=> Some(OsCode::BTN_BASE3),
      297                 	=> Some(OsCode::BTN_BASE4),
      298                 	=> Some(OsCode::BTN_BASE5),
      299                 	=> Some(OsCode::BTN_BASE6),
      300                 	=> Some(OsCode::KEY_300),
      301                 	=> Some(OsCode::KEY_301),
      302                 	=> Some(OsCode::KEY_302),
      303                 	=> Some(OsCode::BTN_DEAD),
      304                 	=> Some(OsCode::BTN_SOUTH),
      305                 	=> Some(OsCode::BTN_EAST),
      306                 	=> Some(OsCode::BTN_C),
      307                 	=> Some(OsCode::BTN_NORTH),
      308                 	=> Some(OsCode::BTN_WEST),
      309                 	=> Some(OsCode::BTN_Z),
      310                 	=> Some(OsCode::BTN_TL),
      311                 	=> Some(OsCode::BTN_TR),
      312                 	=> Some(OsCode::BTN_TL2),
      313                 	=> Some(OsCode::BTN_TR2),
      314                 	=> Some(OsCode::BTN_SELECT),
      315                 	=> Some(OsCode::BTN_START),
      316                 	=> Some(OsCode::BTN_MODE),
      317                 	=> Some(OsCode::BTN_THUMBL),
      318                 	=> Some(OsCode::BTN_THUMBR),
      319                 	=> Some(OsCode::KEY_319),
      320                 	=> Some(OsCode::BTN_TOOL_PEN),
      321                 	=> Some(OsCode::BTN_TOOL_RUBBER),
      322                 	=> Some(OsCode::BTN_TOOL_BRUSH),
      323                 	=> Some(OsCode::BTN_TOOL_PENCIL),
      324                 	=> Some(OsCode::BTN_TOOL_AIRBRUSH),
      325                 	=> Some(OsCode::BTN_TOOL_FINGER),
      326                 	=> Some(OsCode::BTN_TOOL_MOUSE),
      327                 	=> Some(OsCode::BTN_TOOL_LENS),
      328                 	=> Some(OsCode::BTN_TOOL_QUINTTAP),
      329                 	=> Some(OsCode::BTN_STYLUS3),
      330                 	=> Some(OsCode::BTN_TOUCH),
      331                 	=> Some(OsCode::BTN_STYLUS),
      332                 	=> Some(OsCode::BTN_STYLUS2),
      333                 	=> Some(OsCode::BTN_TOOL_DOUBLETAP),
      334                 	=> Some(OsCode::BTN_TOOL_TRIPLETAP),
      335                 	=> Some(OsCode::BTN_TOOL_QUADTAP),
      336                 	=> Some(OsCode::BTN_GEAR_DOWN),
      337                 	=> Some(OsCode::BTN_GEAR_UP),
      338                 	=> Some(OsCode::KEY_338),
      339                 	=> Some(OsCode::KEY_339),
      340                 	=> Some(OsCode::KEY_340),
      341                 	=> Some(OsCode::KEY_341),
      342                 	=> Some(OsCode::KEY_342),
      343                 	=> Some(OsCode::KEY_343),
      344                 	=> Some(OsCode::KEY_344),
      345                 	=> Some(OsCode::KEY_345),
      346                 	=> Some(OsCode::KEY_346),
      347                 	=> Some(OsCode::KEY_347),
      348                 	=> Some(OsCode::KEY_348),
      349                 	=> Some(OsCode::KEY_349),
      350                 	=> Some(OsCode::KEY_350),
      351                 	=> Some(OsCode::KEY_351),
      352                 	=> Some(OsCode::KEY_OK),
      353                 	=> Some(OsCode::KEY_SELECT),
      354                 	=> Some(OsCode::KEY_GOTO),
      355                 	=> Some(OsCode::KEY_CLEAR),
      356                 	=> Some(OsCode::KEY_POWER2),
      357                 	=> Some(OsCode::KEY_OPTION),
      358                 	=> Some(OsCode::KEY_INFO),
      359                 	=> Some(OsCode::KEY_TIME),
      360                 	=> Some(OsCode::KEY_VENDOR),
      361                 	=> Some(OsCode::KEY_ARCHIVE),
      362                 	=> Some(OsCode::KEY_PROGRAM),
      363                 	=> Some(OsCode::KEY_CHANNEL),
      364                 	=> Some(OsCode::KEY_FAVORITES),
      365                 	=> Some(OsCode::KEY_EPG),
      366                 	=> Some(OsCode::KEY_PVR),
      367                 	=> Some(OsCode::KEY_MHP),
      368                 	=> Some(OsCode::KEY_LANGUAGE),
      369                 	=> Some(OsCode::KEY_TITLE),
      370                 	=> Some(OsCode::KEY_SUBTITLE),
      371                 	=> Some(OsCode::KEY_ANGLE),
      372                 	=> Some(OsCode::KEY_FULL_SCREEN),
      373                 	=> Some(OsCode::KEY_MODE),
      374                 	=> Some(OsCode::KEY_KEYBOARD),
      375                 	=> Some(OsCode::KEY_ASPECT_RATIO),
      376                 	=> Some(OsCode::KEY_PC),
      377                 	=> Some(OsCode::KEY_TV),
      378                 	=> Some(OsCode::KEY_TV2),
      379                 	=> Some(OsCode::KEY_VCR),
      380                 	=> Some(OsCode::KEY_VCR2),
      381                 	=> Some(OsCode::KEY_SAT),
      382                 	=> Some(OsCode::KEY_SAT2),
      383                 	=> Some(OsCode::KEY_CD),
      384                 	=> Some(OsCode::KEY_TAPE),
      385                 	=> Some(OsCode::KEY_RADIO),
      386                 	=> Some(OsCode::KEY_TUNER),
      387                 	=> Some(OsCode::KEY_PLAYER),
      388                 	=> Some(OsCode::KEY_TEXT),
      389                 	=> Some(OsCode::KEY_DVD),
      390                 	=> Some(OsCode::KEY_AUX),
      391                 	=> Some(OsCode::KEY_MP3),
      392                 	=> Some(OsCode::KEY_AUDIO),
      393                 	=> Some(OsCode::KEY_VIDEO),
      394                 	=> Some(OsCode::KEY_DIRECTORY),
      395                 	=> Some(OsCode::KEY_LIST),
      396                 	=> Some(OsCode::KEY_MEMO),
      397                 	=> Some(OsCode::KEY_CALENDAR),
      398                 	=> Some(OsCode::KEY_RED),
      399                 	=> Some(OsCode::KEY_GREEN),
      400                 	=> Some(OsCode::KEY_YELLOW),
      401                 	=> Some(OsCode::KEY_BLUE),
      402                 	=> Some(OsCode::KEY_CHANNELUP),
      403                 	=> Some(OsCode::KEY_CHANNELDOWN),
      404                 	=> Some(OsCode::KEY_FIRST),
      405                 	=> Some(OsCode::KEY_LAST),
      406                 	=> Some(OsCode::KEY_AB),
      407                 	=> Some(OsCode::KEY_NEXT),
      408                 	=> Some(OsCode::KEY_RESTART),
      409                 	=> Some(OsCode::KEY_SLOW),
      410                 	=> Some(OsCode::KEY_SHUFFLE),
      411                 	=> Some(OsCode::KEY_BREAK),
      412                 	=> Some(OsCode::KEY_PREVIOUS),
      413                 	=> Some(OsCode::KEY_DIGITS),
      414                 	=> Some(OsCode::KEY_TEEN),
      415                 	=> Some(OsCode::KEY_TWEN),
      416                 	=> Some(OsCode::KEY_VIDEOPHONE),
      417                 	=> Some(OsCode::KEY_GAMES),
      418                 	=> Some(OsCode::KEY_ZOOMIN),
      419                 	=> Some(OsCode::KEY_ZOOMOUT),
      420                 	=> Some(OsCode::KEY_ZOOMRESET),
      421                 	=> Some(OsCode::KEY_WORDPROCESSOR),
      422                 	=> Some(OsCode::KEY_EDITOR),
      423                 	=> Some(OsCode::KEY_SPREADSHEET),
      424                 	=> Some(OsCode::KEY_GRAPHICSEDITOR),
      425                 	=> Some(OsCode::KEY_PRESENTATION),
      426                 	=> Some(OsCode::KEY_DATABASE),
      427                 	=> Some(OsCode::KEY_NEWS),
      428                 	=> Some(OsCode::KEY_VOICEMAIL),
      429                 	=> Some(OsCode::KEY_ADDRESSBOOK),
      430                 	=> Some(OsCode::KEY_MESSENGER),
      431                 	=> Some(OsCode::KEY_DISPLAYTOGGLE),
      432                 	=> Some(OsCode::KEY_SPELLCHECK),
      433                 	=> Some(OsCode::KEY_LOGOFF),
      434                 	=> Some(OsCode::KEY_DOLLAR),
      435                 	=> Some(OsCode::KEY_EURO),
      436                 	=> Some(OsCode::KEY_FRAMEBACK),
      437                 	=> Some(OsCode::KEY_FRAMEFORWARD),
      438                 	=> Some(OsCode::KEY_CONTEXT_MENU),
      439                 	=> Some(OsCode::KEY_MEDIA_REPEAT),
      440                 	=> Some(OsCode::KEY_10CHANNELSUP),
      441                 	=> Some(OsCode::KEY_10CHANNELSDOWN),
      442                 	=> Some(OsCode::KEY_IMAGES),
      443                 	=> Some(OsCode::KEY_443),
      444                 	=> Some(OsCode::KEY_444),
      445                 	=> Some(OsCode::KEY_445),
      446                 	=> Some(OsCode::KEY_446),
      447                 	=> Some(OsCode::KEY_447),
      448                 	=> Some(OsCode::KEY_DEL_EOL),
      449                 	=> Some(OsCode::KEY_DEL_EOS),
      450                 	=> Some(OsCode::KEY_INS_LINE),
      451                 	=> Some(OsCode::KEY_DEL_LINE),
      452                 	=> Some(OsCode::KEY_452),
      453                 	=> Some(OsCode::KEY_453),
      454                 	=> Some(OsCode::KEY_454),
      455                 	=> Some(OsCode::KEY_455),
      456                 	=> Some(OsCode::KEY_456),
      457                 	=> Some(OsCode::KEY_457),
      458                 	=> Some(OsCode::KEY_458),
      459                 	=> Some(OsCode::KEY_459),
      460                 	=> Some(OsCode::KEY_460),
      461                 	=> Some(OsCode::KEY_461),
      462                 	=> Some(OsCode::KEY_462),
      463                 	=> Some(OsCode::KEY_463),
      464                 	=> Some(OsCode::KEY_FN),
      465                 	=> Some(OsCode::KEY_FN_ESC),
      466                 	=> Some(OsCode::KEY_FN_F1),
      467                 	=> Some(OsCode::KEY_FN_F2),
      468                 	=> Some(OsCode::KEY_FN_F3),
      469                 	=> Some(OsCode::KEY_FN_F4),
      470                 	=> Some(OsCode::KEY_FN_F5),
      471                 	=> Some(OsCode::KEY_FN_F6),
      472                 	=> Some(OsCode::KEY_FN_F7),
      473                 	=> Some(OsCode::KEY_FN_F8),
      474                 	=> Some(OsCode::KEY_FN_F9),
      475                 	=> Some(OsCode::KEY_FN_F10),
      476                 	=> Some(OsCode::KEY_FN_F11),
      477                 	=> Some(OsCode::KEY_FN_F12),
      478                 	=> Some(OsCode::KEY_FN_1),
      479                 	=> Some(OsCode::KEY_FN_2),
      480                 	=> Some(OsCode::KEY_FN_D),
      481                 	=> Some(OsCode::KEY_FN_E),
      482                 	=> Some(OsCode::KEY_FN_F),
      483                 	=> Some(OsCode::KEY_FN_S),
      484                 	=> Some(OsCode::KEY_FN_B),
      485                 	=> Some(OsCode::KEY_485),
      486                 	=> Some(OsCode::KEY_486),
      487                 	=> Some(OsCode::KEY_487),
      488                 	=> Some(OsCode::KEY_488),
      489                 	=> Some(OsCode::KEY_489),
      490                 	=> Some(OsCode::KEY_490),
      491                 	=> Some(OsCode::KEY_491),
      492                 	=> Some(OsCode::KEY_492),
      493                 	=> Some(OsCode::KEY_493),
      494                 	=> Some(OsCode::KEY_494),
      495                 	=> Some(OsCode::KEY_495),
      496                 	=> Some(OsCode::KEY_496),
      497                 	=> Some(OsCode::KEY_BRL_DOT1),
      498                 	=> Some(OsCode::KEY_BRL_DOT2),
      499                 	=> Some(OsCode::KEY_BRL_DOT3),
      500                 	=> Some(OsCode::KEY_BRL_DOT4),
      501                 	=> Some(OsCode::KEY_BRL_DOT5),
      502                 	=> Some(OsCode::KEY_BRL_DOT6),
      503                 	=> Some(OsCode::KEY_BRL_DOT7),
      504                 	=> Some(OsCode::KEY_BRL_DOT8),
      505                 	=> Some(OsCode::KEY_BRL_DOT9),
      506                 	=> Some(OsCode::KEY_BRL_DOT10),
      507                 	=> Some(OsCode::KEY_507),
      508                 	=> Some(OsCode::KEY_508),
      509                 	=> Some(OsCode::KEY_509),
      510                 	=> Some(OsCode::KEY_510),
      511                 	=> Some(OsCode::KEY_511),
      512                 	=> Some(OsCode::KEY_NUMERIC_0),
      513                 	=> Some(OsCode::KEY_NUMERIC_1),
      514                 	=> Some(OsCode::KEY_NUMERIC_2),
      515                 	=> Some(OsCode::KEY_NUMERIC_3),
      516                 	=> Some(OsCode::KEY_NUMERIC_4),
      517                 	=> Some(OsCode::KEY_NUMERIC_5),
      518                 	=> Some(OsCode::KEY_NUMERIC_6),
      519                 	=> Some(OsCode::KEY_NUMERIC_7),
      520                 	=> Some(OsCode::KEY_NUMERIC_8),
      521                 	=> Some(OsCode::KEY_NUMERIC_9),
      522                 	=> Some(OsCode::KEY_NUMERIC_STAR),
      523                 	=> Some(OsCode::KEY_NUMERIC_POUND),
      524                 	=> Some(OsCode::KEY_NUMERIC_A),
      525                 	=> Some(OsCode::KEY_NUMERIC_B),
      526                 	=> Some(OsCode::KEY_NUMERIC_C),
      527                 	=> Some(OsCode::KEY_NUMERIC_D),
      528                 	=> Some(OsCode::KEY_CAMERA_FOCUS),
      529                 	=> Some(OsCode::KEY_WPS_BUTTON),
      530                 	=> Some(OsCode::KEY_TOUCHPAD_TOGGLE),
      531                 	=> Some(OsCode::KEY_TOUCHPAD_ON),
      532                 	=> Some(OsCode::KEY_TOUCHPAD_OFF),
      533                 	=> Some(OsCode::KEY_CAMERA_ZOOMIN),
      534                 	=> Some(OsCode::KEY_CAMERA_ZOOMOUT),
      535                 	=> Some(OsCode::KEY_CAMERA_UP),
      536                 	=> Some(OsCode::KEY_CAMERA_DOWN),
      537                 	=> Some(OsCode::KEY_CAMERA_LEFT),
      538                 	=> Some(OsCode::KEY_CAMERA_RIGHT),
      539                 	=> Some(OsCode::KEY_ATTENDANT_ON),
      540                 	=> Some(OsCode::KEY_ATTENDANT_OFF),
      541                 	=> Some(OsCode::KEY_ATTENDANT_TOGGLE),
      542                 	=> Some(OsCode::KEY_LIGHTS_TOGGLE),
      543                 	=> Some(OsCode::KEY_543),
      544                 	=> Some(OsCode::BTN_DPAD_UP),
      545                 	=> Some(OsCode::BTN_DPAD_DOWN),
      546                 	=> Some(OsCode::BTN_DPAD_LEFT),
      547                 	=> Some(OsCode::BTN_DPAD_RIGHT),
      548                 	=> Some(OsCode::KEY_548),
      549                 	=> Some(OsCode::KEY_549),
      550                 	=> Some(OsCode::KEY_550),
      551                 	=> Some(OsCode::KEY_551),
      552                 	=> Some(OsCode::KEY_552),
      553                 	=> Some(OsCode::KEY_553),
      554                 	=> Some(OsCode::KEY_554),
      555                 	=> Some(OsCode::KEY_555),
      556                 	=> Some(OsCode::KEY_556),
      557                 	=> Some(OsCode::KEY_557),
      558                 	=> Some(OsCode::KEY_558),
      559                 	=> Some(OsCode::KEY_559),
      560                 	=> Some(OsCode::KEY_ALS_TOGGLE),
      561                 	=> Some(OsCode::KEY_ROTATE_LOCK_TOGGLE),
      562                 	=> Some(OsCode::KEY_562),563	=> Some(OsCode::KEY_563),564	=> Some(OsCode::KEY_564),565	=> Some(OsCode::KEY_565),566	=> Some(OsCode::KEY_566),567	=> Some(OsCode::KEY_567),568	=> Some(OsCode::KEY_568),569	=> Some(OsCode::KEY_569),570	=> Some(OsCode::KEY_570),
      571                 	=> Some(OsCode::KEY_571),572	=> Some(OsCode::KEY_572),573	=> Some(OsCode::KEY_573),574	=> Some(OsCode::KEY_574),575	=> Some(OsCode::KEY_575),
      576                 	=> Some(OsCode::KEY_BUTTONCONFIG),
      577                 	=> Some(OsCode::KEY_TASKMANAGER),
      578                 	=> Some(OsCode::KEY_JOURNAL),
      579                 	=> Some(OsCode::KEY_CONTROLPANEL),
      580                 	=> Some(OsCode::KEY_APPSELECT),
      581                 	=> Some(OsCode::KEY_SCREENSAVER),
      582                 	=> Some(OsCode::KEY_VOICECOMMAND),
      583                 	=> Some(OsCode::KEY_ASSISTANT),
      584                 	=> Some(OsCode::KEY_KBD_LAYOUT_NEXT),
      585                 	=> Some(OsCode::KEY_585),
      586                 	=> Some(OsCode::KEY_586),
      587                 	=> Some(OsCode::KEY_587),
      588                 	=> Some(OsCode::KEY_588),
      589                 	=> Some(OsCode::KEY_589),
      590                 	=> Some(OsCode::KEY_590),
      591                 	=> Some(OsCode::KEY_591),
      592                 	=> Some(OsCode::KEY_BRIGHTNESS_MIN),
      593                 	=> Some(OsCode::KEY_BRIGHTNESS_MAX),
      594                 	=> Some(OsCode::KEY_594),
      595                 	=> Some(OsCode::KEY_595),
      596                 	=> Some(OsCode::KEY_596),
      597                 	=> Some(OsCode::KEY_597),
      598                 	=> Some(OsCode::KEY_598),
      599                 	=> Some(OsCode::KEY_599),
      600                 	=> Some(OsCode::KEY_600),
      601                 	=> Some(OsCode::KEY_601),
      602                 	=> Some(OsCode::KEY_602),
      603                 	=> Some(OsCode::KEY_603),
      604                 	=> Some(OsCode::KEY_604),
      605                 	=> Some(OsCode::KEY_605),
      606                 	=> Some(OsCode::KEY_606),
      607                 	=> Some(OsCode::KEY_607),
      608                 	=> Some(OsCode::KEY_KBDINPUTASSIST_PREV),
      609                 	=> Some(OsCode::KEY_KBDINPUTASSIST_NEXT),
      610                 	=> Some(OsCode::KEY_KBDINPUTASSIST_PREVGROUP),
      611                 	=> Some(OsCode::KEY_KBDINPUTASSIST_NEXTGROUP),
      612                 	=> Some(OsCode::KEY_KBDINPUTASSIST_ACCEPT),
      613                 	=> Some(OsCode::KEY_KBDINPUTASSIST_CANCEL),
      614                 	=> Some(OsCode::KEY_RIGHT_UP),
      615                 	=> Some(OsCode::KEY_RIGHT_DOWN),
      616                 	=> Some(OsCode::KEY_LEFT_UP),
      617                 	=> Some(OsCode::KEY_LEFT_DOWN),
      618                 	=> Some(OsCode::KEY_ROOT_MENU),
      619                 	=> Some(OsCode::KEY_MEDIA_TOP_MENU),
      620                 	=> Some(OsCode::KEY_NUMERIC_11),
      621                 	=> Some(OsCode::KEY_NUMERIC_12),
      622                 	=> Some(OsCode::KEY_AUDIO_DESC),
      623                 	=> Some(OsCode::KEY_3D_MODE),
      624                 	=> Some(OsCode::KEY_NEXT_FAVORITE),
      625                 	=> Some(OsCode::KEY_STOP_RECORD),
      626                 	=> Some(OsCode::KEY_PAUSE_RECORD),
      627                 	=> Some(OsCode::KEY_VOD),
      628                 	=> Some(OsCode::KEY_UNMUTE),
      629                 	=> Some(OsCode::KEY_FASTREVERSE),
      630                 	=> Some(OsCode::KEY_SLOWREVERSE),
      631                 	=> Some(OsCode::KEY_DATA),
      632                 	=> Some(OsCode::KEY_ONSCREEN_KEYBOARD),
      633                 	=> Some(OsCode::KEY_633            ),634	=> Some(OsCode::KEY_634),635            	=> Some(OsCode::KEY_635),636            	=> Some(OsCode::KEY_636),637	=> Some(OsCode::KEY_637),638	=> Some(OsCode::KEY_638),639	=> Some(OsCode::KEY_639),
      640                 	=> Some(OsCode::KEY_640            ),641	=> Some(OsCode::KEY_641),642            	=> Some(OsCode::KEY_642),643            	=> Some(OsCode::KEY_643),644	=> Some(OsCode::KEY_644),645	=> Some(OsCode::KEY_645),646	=> Some(OsCode::KEY_646),647	=> Some(OsCode::KEY_647),648	=> Some(OsCode::KEY_648),649	=> Some(OsCode::KEY_649),
      650                 	=> Some(OsCode::KEY_650            ),651	=> Some(OsCode::KEY_651),652            	=> Some(OsCode::KEY_652),653            	=> Some(OsCode::KEY_653),654	=> Some(OsCode::KEY_654),655	=> Some(OsCode::KEY_655),656	=> Some(OsCode::KEY_656),657	=> Some(OsCode::KEY_657),658	=> Some(OsCode::KEY_658),659	=> Some(OsCode::KEY_659),
      660                 	=> Some(OsCode::KEY_660            ),661	=> Some(OsCode::KEY_661),662            	=> Some(OsCode::KEY_662),663            	=> Some(OsCode::KEY_663),664	=> Some(OsCode::KEY_664),665	=> Some(OsCode::KEY_665),666	=> Some(OsCode::KEY_666),667	=> Some(OsCode::KEY_667),668	=> Some(OsCode::KEY_668),669	=> Some(OsCode::KEY_669),
      670                 	=> Some(OsCode::KEY_670            ),671	=> Some(OsCode::KEY_671),672            	=> Some(OsCode::KEY_672),673            	=> Some(OsCode::KEY_673),674	=> Some(OsCode::KEY_674),675	=> Some(OsCode::KEY_675),676	=> Some(OsCode::KEY_676),677	=> Some(OsCode::KEY_677),678	=> Some(OsCode::KEY_678),679	=> Some(OsCode::KEY_679),
      680                 	=> Some(OsCode::KEY_680            ),681	=> Some(OsCode::KEY_681),682            	=> Some(OsCode::KEY_682),683            	=> Some(OsCode::KEY_683),684	=> Some(OsCode::KEY_684),685	=> Some(OsCode::KEY_685),686	=> Some(OsCode::KEY_686),687	=> Some(OsCode::KEY_687),688	=> Some(OsCode::KEY_688),689	=> Some(OsCode::KEY_689),
      690                 	=> Some(OsCode::KEY_690            ),691	=> Some(OsCode::KEY_691),692            	=> Some(OsCode::KEY_692),693            	=> Some(OsCode::KEY_693),694	=> Some(OsCode::KEY_694),695	=> Some(OsCode::KEY_695),696	=> Some(OsCode::KEY_696),697	=> Some(OsCode::KEY_697),698	=> Some(OsCode::KEY_698),699	=> Some(OsCode::KEY_699),
      700                 	=> Some(OsCode::KEY_700            ),701	=> Some(OsCode::KEY_701),702            	=> Some(OsCode::KEY_702),703            	=> Some(OsCode::KEY_703),
      704                 	=> Some(OsCode::BTN_TRIGGER_HAPPY1 ),705	=> Some(OsCode::BTN_TRIGGER_HAPPY2),706 	=> Some(OsCode::BTN_TRIGGER_HAPPY3),707 	=> Some(OsCode::BTN_TRIGGER_HAPPY4),708 	=> Some(OsCode::BTN_TRIGGER_HAPPY5),709 	=> Some(OsCode::BTN_TRIGGER_HAPPY6),710 	=> Some(OsCode::BTN_TRIGGER_HAPPY7),711 	=> Some(OsCode::BTN_TRIGGER_HAPPY8),712 	=> Some(OsCode::BTN_TRIGGER_HAPPY9),
      713                 	=> Some(OsCode::BTN_TRIGGER_HAPPY10),714	=> Some(OsCode::BTN_TRIGGER_HAPPY11),715	=> Some(OsCode::BTN_TRIGGER_HAPPY12),716	=> Some(OsCode::BTN_TRIGGER_HAPPY13),717	=> Some(OsCode::BTN_TRIGGER_HAPPY14),718	=> Some(OsCode::BTN_TRIGGER_HAPPY15),719	=> Some(OsCode::BTN_TRIGGER_HAPPY16),720	=> Some(OsCode::BTN_TRIGGER_HAPPY17),721	=> Some(OsCode::BTN_TRIGGER_HAPPY18),722	=> Some(OsCode::BTN_TRIGGER_HAPPY19),
      723                 	=> Some(OsCode::BTN_TRIGGER_HAPPY20),724	=> Some(OsCode::BTN_TRIGGER_HAPPY21),725	=> Some(OsCode::BTN_TRIGGER_HAPPY22),726	=> Some(OsCode::BTN_TRIGGER_HAPPY23),727	=> Some(OsCode::BTN_TRIGGER_HAPPY24),728	=> Some(OsCode::BTN_TRIGGER_HAPPY25),729	=> Some(OsCode::BTN_TRIGGER_HAPPY26),730	=> Some(OsCode::BTN_TRIGGER_HAPPY27),731	=> Some(OsCode::BTN_TRIGGER_HAPPY28),732	=> Some(OsCode::BTN_TRIGGER_HAPPY29),
      733                 	=> Some(OsCode::BTN_TRIGGER_HAPPY30),734	=> Some(OsCode::BTN_TRIGGER_HAPPY31),735	=> Some(OsCode::BTN_TRIGGER_HAPPY32),736	=> Some(OsCode::BTN_TRIGGER_HAPPY33),737	=> Some(OsCode::BTN_TRIGGER_HAPPY34),738	=> Some(OsCode::BTN_TRIGGER_HAPPY35),739	=> Some(OsCode::BTN_TRIGGER_HAPPY36),740	=> Some(OsCode::BTN_TRIGGER_HAPPY37),741	=> Some(OsCode::BTN_TRIGGER_HAPPY38),742	=> Some(OsCode::BTN_TRIGGER_HAPPY39),
      743                 	=> Some(OsCode::BTN_TRIGGER_HAPPY40),
      744                 	=> Some(OsCode::BTN_MAX),
      767                 	=> Some(OsCode::KEY_MAX),
      _ => None,
    }
  }

  pub(super) const fn as_u16_windows(self) -> u16 {
    match self {
      OsCode::KEY_0           	=> 0x30,
      OsCode::KEY_1           	=> 0x31,OsCode::KEY_2	=> 0x32,OsCode::KEY_3	=> 0x33,OsCode::KEY_4	=> 0x34,OsCode::KEY_5	=> 0x35,OsCode::KEY_6	=> 0x36,OsCode::KEY_7	=> 0x37,OsCode::KEY_8	=> 0x38,OsCode::KEY_9	=> 0x39,
      OsCode::KEY_A           	=> 0x41,OsCode::KEY_B	=> 0x42,OsCode::KEY_C	=> 0x43,OsCode::KEY_D	=> 0x44,OsCode::KEY_E	=> 0x45,OsCode::KEY_F	=> 0x46,OsCode::KEY_G	=> 0x47,OsCode::KEY_H	=> 0x48,OsCode::KEY_I	=> 0x49,OsCode::KEY_J	=> 0x4A,
      OsCode::KEY_K           	=> 0x4B,OsCode::KEY_L	=> 0x4C,OsCode::KEY_M	=> 0x4D,OsCode::KEY_N	=> 0x4E,OsCode::KEY_O	=> 0x4F,OsCode::KEY_P	=> 0x50,OsCode::KEY_Q	=> 0x51,OsCode::KEY_R	=> 0x52,OsCode::KEY_S	=> 0x53,OsCode::KEY_T	=> 0x54,
      OsCode::KEY_U           	=> 0x55,OsCode::KEY_V	=> 0x56,OsCode::KEY_W	=> 0x57,OsCode::KEY_X	=> 0x58,OsCode::KEY_Y	=> 0x59,OsCode::KEY_Z	=> 0x5A,
      OsCode::KEY_SEMICOLON   	=> vk_oem1,
      OsCode::KEY_SLASH       	=> vk_oem2,
      OsCode::KEY_GRAVE       	=> vk_oem3,
      OsCode::KEY_LEFTBRACE   	=> vk_oem4,
      OsCode::KEY_BACKSLASH   	=> vk_oem5,
      OsCode::KEY_RIGHTBRACE  	=> vk_oem6,
      OsCode::KEY_APOSTROPHE  	=> vk_oem7,
      OsCode::KEY_MINUS       	=> vk_oem_minus,
      OsCode::KEY_DOT         	=> vk_oem_period,
      OsCode::KEY_EQUAL       	=> vk_oem_plus,
      OsCode::KEY_BACKSPACE   	=> vk_back,
      OsCode::KEY_ESC         	=> vk_escape,
      OsCode::KEY_TAB         	=> vk_tab,
      OsCode::KEY_ENTER       	=> vk_return,
      OsCode::KEY_LEFTCTRL    	=> vk_lcontrol,OsCode::KEY_RIGHTCTRL	=> vk_rcontrol,
      OsCode::KEY_LEFTSHIFT   	=> vk_lshift,OsCode::KEY_RIGHTSHIFT 	=> vk_rshift,
      OsCode::KEY_COMMA       	=> vk_oem_comma,
      OsCode::KEY_KPASTERISK  	=> vk_multiply,
      OsCode::KEY_LEFTALT     	=> vk_lmenu,
      OsCode::KEY_SPACE       	=> vk_space,
      OsCode::KEY_CAPSLOCK    	=> vk_capital,
      OsCode::KEY_F1          	=> vk_f1,OsCode::KEY_F2  	=> vk_f2,OsCode::KEY_F3	=> vk_f3,OsCode::KEY_F4	=> vk_f4,OsCode::KEY_F5	=> vk_f5,OsCode::KEY_F6	=> vk_f6,OsCode::KEY_F7	=> vk_f7,OsCode::KEY_F8	=> vk_f8,OsCode::KEY_F9	=> vk_f9,OsCode::KEY_F10	=> vk_f10,
      OsCode::KEY_F11         	=> vk_f11,OsCode::KEY_F12	=> vk_f12,
      OsCode::KEY_NUMLOCK     	=> vk_numlock,
      OsCode::KEY_CLEAR       	=> vk_clear,
      OsCode::KEY_SCROLLLOCK  	=> vk_scroll,
      OsCode::KEY_KP0         	=> vk_numpad0,
      OsCode::KEY_KP1         	=> vk_numpad1,OsCode::KEY_KP2	=> vk_numpad2,OsCode::KEY_KP3	=> vk_numpad3,OsCode::KEY_KP4	=> vk_numpad4,OsCode::KEY_KP5	=> vk_numpad5,OsCode::KEY_KP6	=> vk_numpad6,OsCode::KEY_KP7	=> vk_numpad7,OsCode::KEY_KP8	=> vk_numpad8,OsCode::KEY_KP9	=> vk_numpad9,
      OsCode::KEY_KPMINUS     	=> vk_subtract,
      OsCode::KEY_KPPLUS      	=> vk_add,
      OsCode::KEY_KPDOT       	=> vk_decimal,
      OsCode::KEY_KPENTER     	=> vk_none,
      OsCode::KEY_KPSLASH     	=> vk_divide,
      OsCode::KEY_RIGHTALT    	=> vk_rmenu,
      OsCode::KEY_HOME        	=> vk_home,
      OsCode::KEY_PAGEUP      	=> vk_prior,
      OsCode::KEY_END         	=> vk_end,
      OsCode::KEY_DOWN        	=> vk_down,OsCode::KEY_UP	=> vk_up,OsCode::KEY_LEFT	=> vk_left,OsCode::KEY_RIGHT	=> vk_right,
      OsCode::KEY_PAGEDOWN    	=> vk_next,
      OsCode::KEY_INSERT      	=> vk_insert,
      OsCode::KEY_DELETE      	=> vk_delete,
      OsCode::KEY_MUTE        	=> vk_volume_mute,
      OsCode::KEY_VOLUMEDOWN  	=> vk_volume_down,
      OsCode::KEY_VOLUMEUP    	=> vk_volume_up,
      OsCode::KEY_PAUSE       	=> vk_pause,
      OsCode::KEY_LEFTMETA    	=> vk_lwin,
      OsCode::KEY_RIGHTMETA   	=> vk_rwin,
      OsCode::KEY_COMPOSE     	=> vk_apps,
      OsCode::KEY_BACK        	=> vk_browser_back,
      OsCode::KEY_FORWARD     	=> vk_browser_forward,
      OsCode::KEY_NEXTSONG    	=> vk_media_next_track,
      OsCode::KEY_PLAYPAUSE   	=> vk_media_play_pause,
      OsCode::KEY_PREVIOUSSONG	=> vk_media_prev_track,
      OsCode::KEY_STOP        	=> vk_media_stop,
      OsCode::KEY_HOMEPAGE    	=> vk_browser_home,
      OsCode::KEY_MAIL        	=> vk_launch_mail,
      OsCode::KEY_MEDIA       	=> vk_launch_media_select,
      OsCode::KEY_REFRESH     	=> vk_browser_refresh,
      OsCode::KEY_F13         	=> vk_f13,
      OsCode::KEY_F14         	=> vk_f14,
      OsCode::KEY_F15         	=> vk_f15,
      OsCode::KEY_F16         	=> vk_f16,
      OsCode::KEY_F17         	=> vk_f17,
      OsCode::KEY_F18         	=> vk_f18,
      OsCode::KEY_F19         	=> vk_f19,
      OsCode::KEY_F20         	=> vk_f20,
      OsCode::KEY_F21         	=> vk_f21,
      OsCode::KEY_F22         	=> vk_f22,
      OsCode::KEY_F23         	=> vk_f23,
      OsCode::KEY_F24         	=> vk_f24,
      OsCode::KEY_HANGEUL     	=> vk_hangeul,
      OsCode::KEY_HANJA       	=> vk_hanja,
      OsCode::KEY_252         	=> vk_oem8,
      OsCode::KEY_102ND       	=> vk_oem102,
      OsCode::KEY_PLAY        	=> vk_play,
      OsCode::KEY_PRINT       	=> vk_snapshot,
      OsCode::KEY_SEARCH      	=> vk_browser_search,
      OsCode::KEY_FAVORITES   	=> vk_browser_favorites,
      OsCode::KEY_RO          	=> 0xC1,
      OsCode::KEY_HENKAN      	=> vk_convert,
      OsCode::KEY_MUHENKAN    	=> vk_nonconvert,
      OsCode::BTN_LEFT        	=> vk_lbutton,
      OsCode::BTN_RIGHT       	=> vk_rbutton,
      OsCode::BTN_MIDDLE      	=> vk_mbutton,
      OsCode::BTN_SIDE        	=> vk_xbutton1,
      OsCode::BTN_EXTRA       	=> vk_xbutton2,
      osc                     	=> osc as u16,
    }
  }
}
