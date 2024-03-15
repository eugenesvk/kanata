// This file is adapted from the original ktrl's `keys.rs` file for Windows.

use super::OsCode;

#[allow(unused)]
mod keys { // Taken from: github.com/retep998/winapi-rs/blob/0.3/src/um/winuser.rs#L253
  pub const VK_CANCEL                        	:u16 = 0x03;
  pub const VK_LBUTTON                       	:u16 = 0x01; pub const VK_RBUTTON 	:u16 = 0x02; pub const VK_MBUTTON	:u16 = 0x04;
  pub const VK_XBUTTON1                      	:u16 = 0x05; pub const VK_XBUTTON2	:u16 = 0x06;
  pub const VK_BACK                          	:u16 = 0x08;
  pub const VK_TAB                           	:u16 = 0x09;
  pub const VK_CLEAR                         	:u16 = 0x0C;
  pub const VK_RETURN                        	:u16 = 0x0D;
  pub const VK_MENU                          	:u16 = 0x12; pub const VK_LMENU	:u16 = 0xA4; pub const VK_RMENU	:u16 = 0xA5;
  pub const VK_PAUSE                         	:u16 = 0x13;
  pub const VK_CAPITAL                       	:u16 = 0x14;
  pub const VK_LWIN                          	:u16 = 0x5B; pub const VK_RWIN    	:u16 = 0x5C;
  pub const VK_LSHIFT                        	:u16 = 0xA0; pub const VK_RSHIFT  	:u16 = 0xA1; pub const VK_SHIFT  	:u16 = 0x10;
  pub const VK_LCONTROL                      	:u16 = 0xA2; pub const VK_RCONTROL	:u16 = 0xA3; pub const VK_CONTROL	:u16 = 0x11;
  pub const VK_KANA                          	:u16 = 0x15;
  pub const VK_HANGEUL                       	:u16 = 0x15;
  pub const VK_HANGUL                        	:u16 = 0x15;
  pub const VK_NUMLOCK                       	:u16 = 0x90; pub const VK_SCROLL     	:u16 = 0x91;
  pub const VK_OEM_FJ_LOYA                   	:u16 = 0x95; pub const VK_OEM_FJ_ROYA	:u16 = 0x96;
  pub const VK_JUNJA                         	:u16 = 0x17;
  pub const VK_FINAL                         	:u16 = 0x18;
  pub const VK_HANJA                         	:u16 = 0x19;
  pub const VK_KANJI                         	:u16 = 0x19;
  pub const VK_ESCAPE                        	:u16 = 0x1B;
  pub const VK_CONVERT                       	:u16 = 0x1C;
  pub const VK_NONCONVERT                    	:u16 = 0x1D;
  pub const VK_ACCEPT                        	:u16 = 0x1E;
  pub const VK_MODECHANGE                    	:u16 = 0x1F;
  pub const VK_SPACE                         	:u16 = 0x20;
  pub const VK_PRIOR                         	:u16 = 0x21; pub const VK_NEXT	:u16 = 0x22;
  pub const VK_HOME                          	:u16 = 0x24; pub const VK_END 	:u16 = 0x23;
  pub const VK_DOWN                          	:u16 = 0x28; pub const VK_UP  	:u16 = 0x26; pub const VK_LEFT	:u16 = 0x25; pub const VK_RIGHT	:u16 = 0x27;
  pub const VK_SELECT                        	:u16 = 0x29;
  pub const VK_PRINT                         	:u16 = 0x2A;
  pub const VK_EXECUTE                       	:u16 = 0x2B;
  pub const VK_SNAPSHOT                      	:u16 = 0x2C;
  pub const VK_INSERT                        	:u16 = 0x2D;
  pub const VK_DELETE                        	:u16 = 0x2E;
  pub const VK_HELP                          	:u16 = 0x2F;
  pub const VK_APPS                          	:u16 = 0x5D;
  pub const VK_SLEEP                         	:u16 = 0x5F;
  pub const VK_NUMPAD0                       	:u16 = 0x60; pub const VK_NUMPAD1 	:u16 = 0x61; pub const VK_NUMPAD2	:u16 = 0x62; pub const VK_NUMPAD3	:u16 = 0x63; pub const VK_NUMPAD4	:u16 = 0x64; pub const VK_NUMPAD5	:u16 = 0x65; pub const VK_NUMPAD6	:u16 = 0x66; pub const VK_NUMPAD7	:u16 = 0x67; pub const VK_NUMPAD8	:u16 = 0x68; pub const VK_NUMPAD9	:u16 = 0x69;
  pub const VK_MULTIPLY                      	:u16 = 0x6A; pub const VK_DIVIDE  	:u16 = 0x6F;
  pub const VK_ADD                           	:u16 = 0x6B; pub const VK_SUBTRACT	:u16 = 0x6D;
  pub const VK_SEPARATOR                     	:u16 = 0x6C;
  pub const VK_DECIMAL                       	:u16 = 0x6E;
  pub const VK_F1                            	:u16 = 0x70; pub const VK_F2 	:u16 = 0x71; pub const VK_F3 	:u16 = 0x72; pub const VK_F4 	:u16 = 0x73; pub const VK_F5 	:u16 = 0x74; pub const VK_F6 	:u16 = 0x75; pub const VK_F7 	:u16 = 0x76; pub const VK_F8 	:u16 = 0x77; pub const VK_F9 	:u16 = 0x78; pub const VK_F10	:u16 = 0x79;
  pub const VK_F11                           	:u16 = 0x7A; pub const VK_F12	:u16 = 0x7B; pub const VK_F13	:u16 = 0x7C; pub const VK_F14	:u16 = 0x7D; pub const VK_F15	:u16 = 0x7E; pub const VK_F16	:u16 = 0x7F; pub const VK_F17	:u16 = 0x80; pub const VK_F18	:u16 = 0x81; pub const VK_F19	:u16 = 0x82; pub const VK_F20	:u16 = 0x83;
  pub const VK_F21                           	:u16 = 0x84; pub const VK_F22	:u16 = 0x85; pub const VK_F23	:u16 = 0x86; pub const VK_F24	:u16 = 0x87;
  pub const VK_NAVIGATION_VIEW               	:u16 = 0x88;
  pub const VK_NAVIGATION_MENU               	:u16 = 0x89;
  pub const VK_NAVIGATION_DOWN               	:u16 = 0x8B; pub const VK_NAVIGATION_UP	:u16 = 0x8A; pub const VK_NAVIGATION_LEFT	:u16 = 0x8C; pub const VK_NAVIGATION_RIGHT	:u16 = 0x8D;
  pub const VK_NAVIGATION_ACCEPT             	:u16 = 0x8E;
  pub const VK_NAVIGATION_CANCEL             	:u16 = 0x8F;
  pub const VK_OEM_NEC_EQUAL                 	:u16 = 0x92;
  pub const VK_OEM_FJ_JISHO                  	:u16 = 0x92; pub const VK_OEM_FJ_MASSHOU	:u16 = 0x93; pub const VK_OEM_FJ_TOUROKU	:u16 = 0x94;
  pub const VK_BROWSER_BACK                  	:u16 = 0xA6;
  pub const VK_BROWSER_FORWARD               	:u16 = 0xA7;
  pub const VK_BROWSER_REFRESH               	:u16 = 0xA8;
  pub const VK_BROWSER_STOP                  	:u16 = 0xA9;
  pub const VK_BROWSER_SEARCH                	:u16 = 0xAA;
  pub const VK_BROWSER_FAVORITES             	:u16 = 0xAB;
  pub const VK_BROWSER_HOME                  	:u16 = 0xAC;
  pub const VK_VOLUME_DOWN                   	:u16 = 0xAE; pub const VK_VOLUME_UP       	:u16 = 0xAF; pub const VK_VOLUME_MUTE	:u16 = 0xAD;
  pub const VK_MEDIA_NEXT_TRACK              	:u16 = 0xB0; pub const VK_MEDIA_PREV_TRACK	:u16 = 0xB1;
  pub const VK_MEDIA_PLAY_PAUSE              	:u16 = 0xB3; pub const VK_MEDIA_STOP      	:u16 = 0xB2;
  pub const VK_LAUNCH_MAIL                   	:u16 = 0xB4;
  pub const VK_LAUNCH_MEDIA_SELECT           	:u16 = 0xB5;
  pub const VK_LAUNCH_APP1                   	:u16 = 0xB6; pub const VK_LAUNCH_APP2	:u16 = 0xB7;
  pub const VK_OEM_PLUS                      	:u16 = 0xBB;
  pub const VK_OEM_COMMA                     	:u16 = 0xBC;
  pub const VK_OEM_MINUS                     	:u16 = 0xBD;
  pub const VK_OEM_PERIOD                    	:u16 = 0xBE;
  pub const VK_GAMEPAD_A                     	:u16 = 0xC3; pub const VK_GAMEPAD_B             	:u16 = 0xC4; pub const VK_GAMEPAD_X	:u16 = 0xC5; pub const VK_GAMEPAD_Y	:u16 = 0xC6;
  pub const VK_GAMEPAD_LEFT_SHOULDER         	:u16 = 0xC8; pub const VK_GAMEPAD_RIGHT_SHOULDER	:u16 = 0xC7;
  pub const VK_GAMEPAD_LEFT_TRIGGER          	:u16 = 0xC9; pub const VK_GAMEPAD_RIGHT_TRIGGER 	:u16 = 0xCA;
  pub const VK_GAMEPAD_DPAD_DOWN             	:u16 = 0xCC; pub const VK_GAMEPAD_DPAD_UP       	:u16 = 0xCB;
  pub const VK_GAMEPAD_DPAD_LEFT             	:u16 = 0xCD; pub const VK_GAMEPAD_DPAD_RIGHT    	:u16 = 0xCE;
  pub const VK_GAMEPAD_MENU                  	:u16 = 0xCF;
  pub const VK_GAMEPAD_VIEW                  	:u16 = 0xD0;
  pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON	:u16 = 0xD1; pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON	:u16 = 0xD2;
  pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN  	:u16 = 0xD4; pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP     	:u16 = 0xD3; pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT 	:u16 = 0xD6; pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT 	:u16 = 0xD5;
  pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN 	:u16 = 0xD8; pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP    	:u16 = 0xD7; pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT	:u16 = 0xDA; pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT	:u16 = 0xD9;
  pub const VK_OEM_1                         	:u16 = 0xBA; pub const VK_OEM_2                          	:u16 = 0xBF; pub const VK_OEM_3                        	:u16 = 0xC0; pub const VK_OEM_4                         	:u16 = 0xDB;pub const VK_OEM_5	:u16 = 0xDC;pub const VK_OEM_6	:u16 = 0xDD;pub const VK_OEM_7	:u16 = 0xDE;pub const VK_OEM_8	:u16 = 0xDF;
  pub const VK_OEM_AX                        	:u16 = 0xE1;
  pub const VK_OEM_102                       	:u16 = 0xE2;
  pub const VK_ICO_HELP                      	:u16 = 0xE3;
  pub const VK_ICO_00                        	:u16 = 0xE4;
  pub const VK_PROCESSKEY                    	:u16 = 0xE5;
  pub const VK_ICO_CLEAR                     	:u16 = 0xE6;
  pub const VK_PACKET                        	:u16 = 0xE7;
  pub const VK_OEM_RESET                     	:u16 = 0xE9;
  pub const VK_OEM_JUMP                      	:u16 = 0xEA;
  pub const VK_OEM_PA1                       	:u16 = 0xEB;
  pub const VK_OEM_PA2                       	:u16 = 0xEC;
  pub const VK_OEM_PA3                       	:u16 = 0xED;
  pub const VK_OEM_WSCTRL                    	:u16 = 0xEE;
  pub const VK_OEM_CUSEL                     	:u16 = 0xEF;
  pub const VK_OEM_ATTN                      	:u16 = 0xF0;
  pub const VK_OEM_FINISH                    	:u16 = 0xF1;
  pub const VK_OEM_COPY                      	:u16 = 0xF2;
  pub const VK_OEM_AUTO                      	:u16 = 0xF3;
  pub const VK_OEM_ENLW                      	:u16 = 0xF4;
  pub const VK_OEM_BACKTAB                   	:u16 = 0xF5;
  pub const VK_ATTN                          	:u16 = 0xF6;
  pub const VK_CRSEL                         	:u16 = 0xF7;
  pub const VK_EXSEL                         	:u16 = 0xF8;
  pub const VK_EREOF                         	:u16 = 0xF9;
  pub const VK_PLAY                          	:u16 = 0xFA;
  pub const VK_ZOOM                          	:u16 = 0xFB;
  pub const VK_NONAME                        	:u16 = 0xFC;
  pub const VK_PA1                           	:u16 = 0xFD;
  pub const VK_OEM_CLEAR                     	:u16 = 0xFE;
  pub const VK_KPENTER_FAKE                  	:u16 = 0xFF;
}
pub use keys::*;

impl OsCode {
  pub(super) const fn from_u16_windows(code	:u16) -> Option<Self> {
    match code {
      0x30                  	=> Some(OsCode::KEY_0),
      0x31                  	=> Some(OsCode::KEY_1),0x32            	=> Some(OsCode::KEY_2),0x33        	=> Some(OsCode::KEY_3),0x34        	=> Some(OsCode::KEY_4),0x35            	=> Some(OsCode::KEY_5),0x36            	=> Some(OsCode::KEY_6),0x37	=> Some(OsCode::KEY_7),0x38	=> Some(OsCode::KEY_8),0x39	=> Some(OsCode::KEY_9),
      0x41                  	=> Some(OsCode::KEY_A),0x42            	=> Some(OsCode::KEY_B),0x43        	=> Some(OsCode::KEY_C),0x44        	=> Some(OsCode::KEY_D),0x45            	=> Some(OsCode::KEY_E),0x46            	=> Some(OsCode::KEY_F),0x47	=> Some(OsCode::KEY_G),0x48	=> Some(OsCode::KEY_H),0x49	=> Some(OsCode::KEY_I),0x4A	=> Some(OsCode::KEY_J),
      0x4B                  	=> Some(OsCode::KEY_K),0x4C            	=> Some(OsCode::KEY_L),0x4D        	=> Some(OsCode::KEY_M),0x4E        	=> Some(OsCode::KEY_N),0x4F            	=> Some(OsCode::KEY_O),0x50            	=> Some(OsCode::KEY_P),0x51	=> Some(OsCode::KEY_Q),0x52	=> Some(OsCode::KEY_R),0x53	=> Some(OsCode::KEY_S),0x54	=> Some(OsCode::KEY_T),
      0x55                  	=> Some(OsCode::KEY_U),0x56            	=> Some(OsCode::KEY_V),0x57        	=> Some(OsCode::KEY_W),0x58        	=> Some(OsCode::KEY_X),0x59            	=> Some(OsCode::KEY_Y),0x5A            	=> Some(OsCode::KEY_Z),
      VK_OEM_1              	=> Some(OsCode::KEY_SEMICOLON),VK_OEM_2	=> Some(OsCode::KEY_SLASH),VK_OEM_3	=> Some(OsCode::KEY_GRAVE),VK_OEM_4	=> Some(OsCode::KEY_LEFTBRACE),VK_OEM_5	=> Some(OsCode::KEY_BACKSLASH),VK_OEM_6	=> Some(OsCode::KEY_RIGHTBRACE),VK_OEM_7	=> Some(OsCode::KEY_APOSTROPHE),
      VK_OEM_MINUS          	=> Some(OsCode::KEY_MINUS),
      VK_OEM_PERIOD         	=> Some(OsCode::KEY_DOT),
      VK_OEM_PLUS           	=> Some(OsCode::KEY_EQUAL),
      VK_BACK               	=> Some(OsCode::KEY_BACKSPACE),
      VK_ESCAPE             	=> Some(OsCode::KEY_ESC),
      VK_TAB                	=> Some(OsCode::KEY_TAB),
      VK_RETURN             	=> Some(OsCode::KEY_ENTER),
      VK_LCONTROL           	=> Some(OsCode::KEY_LEFTCTRL),VK_RCONTROL           	=> Some(OsCode::KEY_RIGHTCTRL),
      VK_LSHIFT             	=> Some(OsCode::KEY_LEFTSHIFT),VK_RSHIFT	=> Some(OsCode::KEY_RIGHTSHIFT),
      VK_HANGEUL            	=> Some(OsCode::KEY_HANGEUL),
      VK_HANJA              	=> Some(OsCode::KEY_HANJA),
      VK_OEM_COMMA          	=> Some(OsCode::KEY_COMMA),
      VK_MULTIPLY           	=> Some(OsCode::KEY_KPASTERISK),
      VK_LMENU              	=> Some(OsCode::KEY_LEFTALT),
      VK_SPACE              	=> Some(OsCode::KEY_SPACE),
      VK_CAPITAL            	=> Some(OsCode::KEY_CAPSLOCK),
      VK_NUMLOCK            	=> Some(OsCode::KEY_NUMLOCK),
      VK_CLEAR              	=> Some(OsCode::KEY_CLEAR),
      VK_SCROLL             	=> Some(OsCode::KEY_SCROLLLOCK),
      VK_NUMPAD0            	=> Some(OsCode::KEY_KP0),
      VK_NUMPAD1            	=> Some(OsCode::KEY_KP1),VK_NUMPAD2	=> Some(OsCode::KEY_KP2),VK_NUMPAD3	=> Some(OsCode::KEY_KP3),VK_NUMPAD4	=> Some(OsCode::KEY_KP4),VK_NUMPAD5	=> Some(OsCode::KEY_KP5),VK_NUMPAD6	=> Some(OsCode::KEY_KP6),VK_NUMPAD7	=> Some(OsCode::KEY_KP7),VK_NUMPAD8	=> Some(OsCode::KEY_KP8),VK_NUMPAD9	=> Some(OsCode::KEY_KP9),
      VK_SUBTRACT           	=> Some(OsCode::KEY_KPMINUS),
      VK_ADD                	=> Some(OsCode::KEY_KPPLUS),
      VK_DECIMAL            	=> Some(OsCode::KEY_KPDOT),
      VK_KPENTER_FAKE       	=> Some(OsCode::KEY_KPENTER),
      VK_DIVIDE             	=> Some(OsCode::KEY_KPSLASH),
      VK_RMENU              	=> Some(OsCode::KEY_RIGHTALT),
      VK_HOME               	=> Some(OsCode::KEY_HOME),
      VK_UP                 	=> Some(OsCode::KEY_UP),
      VK_PRIOR              	=> Some(OsCode::KEY_PAGEUP),
      VK_LEFT               	=> Some(OsCode::KEY_LEFT),
      VK_RIGHT              	=> Some(OsCode::KEY_RIGHT),
      VK_END                	=> Some(OsCode::KEY_END),
      VK_DOWN               	=> Some(OsCode::KEY_DOWN),
      VK_NEXT               	=> Some(OsCode::KEY_PAGEDOWN),
      VK_INSERT             	=> Some(OsCode::KEY_INSERT),
      VK_DELETE             	=> Some(OsCode::KEY_DELETE),
      VK_VOLUME_MUTE        	=> Some(OsCode::KEY_MUTE),
      VK_VOLUME_DOWN        	=> Some(OsCode::KEY_VOLUMEDOWN),
      VK_VOLUME_UP          	=> Some(OsCode::KEY_VOLUMEUP),
      VK_PAUSE              	=> Some(OsCode::KEY_PAUSE),
      VK_LWIN               	=> Some(OsCode::KEY_LEFTMETA),
      VK_RWIN               	=> Some(OsCode::KEY_RIGHTMETA),
      VK_APPS               	=> Some(OsCode::KEY_COMPOSE),
      VK_BROWSER_BACK       	=> Some(OsCode::KEY_BACK),
      VK_BROWSER_FORWARD    	=> Some(OsCode::KEY_FORWARD),
      VK_MEDIA_NEXT_TRACK   	=> Some(OsCode::KEY_NEXTSONG),
      VK_MEDIA_PLAY_PAUSE   	=> Some(OsCode::KEY_PLAYPAUSE),
      VK_MEDIA_PREV_TRACK   	=> Some(OsCode::KEY_PREVIOUSSONG),
      VK_MEDIA_STOP         	=> Some(OsCode::KEY_STOP),
      VK_BROWSER_HOME       	=> Some(OsCode::KEY_HOMEPAGE),
      VK_LAUNCH_MAIL        	=> Some(OsCode::KEY_MAIL),
      VK_LAUNCH_MEDIA_SELECT	=> Some(OsCode::KEY_MEDIA),
      VK_BROWSER_REFRESH    	=> Some(OsCode::KEY_REFRESH),
      VK_F1                 	=> Some(OsCode::KEY_F1),VK_F2  	=> Some(OsCode::KEY_F2),VK_F3  	=> Some(OsCode::KEY_F3),VK_F4  	=> Some(OsCode::KEY_F4),VK_F5  	=> Some(OsCode::KEY_F5),VK_F6  	=> Some(OsCode::KEY_F6),VK_F7  	=> Some(OsCode::KEY_F7),VK_F8  	=> Some(OsCode::KEY_F8),VK_F9  	=> Some(OsCode::KEY_F9),VK_F10 	=> Some(OsCode::KEY_F10),
      VK_F11                	=> Some(OsCode::KEY_F11),VK_F12	=> Some(OsCode::KEY_F12),VK_F13	=> Some(OsCode::KEY_F13),VK_F14	=> Some(OsCode::KEY_F14),VK_F15	=> Some(OsCode::KEY_F15),VK_F16	=> Some(OsCode::KEY_F16),VK_F17	=> Some(OsCode::KEY_F17),VK_F18	=> Some(OsCode::KEY_F18),VK_F19	=> Some(OsCode::KEY_F19),VK_F20	=> Some(OsCode::KEY_F20),
      VK_F21                	=> Some(OsCode::KEY_F21),VK_F22	=> Some(OsCode::KEY_F22),VK_F23	=> Some(OsCode::KEY_F23),VK_F24	=> Some(OsCode::KEY_F24),
      // KEY_252 is nonsensical, but just use it anyway. No idea what Linux OsCode this is.
      // As long as it's not an existing key and the mapping round-trips, this works fine.
      VK_OEM_8            	=> Some(OsCode::KEY_252),
      VK_OEM_102          	=> Some(OsCode::KEY_102ND),
      VK_PLAY             	=> Some(OsCode::KEY_PLAY),
      VK_SNAPSHOT         	=> Some(OsCode::KEY_PRINT),
      VK_BROWSER_SEARCH   	=> Some(OsCode::KEY_SEARCH),
      VK_BROWSER_FAVORITES	=> Some(OsCode::KEY_FAVORITES),
      0xC1                	=> Some(OsCode::KEY_RO),
      VK_CONVERT          	=> Some(OsCode::KEY_HENKAN),
      VK_NONCONVERT       	=> Some(OsCode::KEY_MUHENKAN),
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
      OsCode::KEY_SEMICOLON   	=> VK_OEM_1,
      OsCode::KEY_SLASH       	=> VK_OEM_2,
      OsCode::KEY_GRAVE       	=> VK_OEM_3,
      OsCode::KEY_LEFTBRACE   	=> VK_OEM_4,
      OsCode::KEY_BACKSLASH   	=> VK_OEM_5,
      OsCode::KEY_RIGHTBRACE  	=> VK_OEM_6,
      OsCode::KEY_APOSTROPHE  	=> VK_OEM_7,
      OsCode::KEY_MINUS       	=> VK_OEM_MINUS,
      OsCode::KEY_DOT         	=> VK_OEM_PERIOD,
      OsCode::KEY_EQUAL       	=> VK_OEM_PLUS,
      OsCode::KEY_BACKSPACE   	=> VK_BACK,
      OsCode::KEY_ESC         	=> VK_ESCAPE,
      OsCode::KEY_TAB         	=> VK_TAB,
      OsCode::KEY_ENTER       	=> VK_RETURN,
      OsCode::KEY_LEFTCTRL    	=> VK_LCONTROL,OsCode::KEY_RIGHTCTRL	=> VK_RCONTROL,
      OsCode::KEY_LEFTSHIFT   	=> VK_LSHIFT,OsCode::KEY_RIGHTSHIFT 	=> VK_RSHIFT,
      OsCode::KEY_COMMA       	=> VK_OEM_COMMA,
      OsCode::KEY_KPASTERISK  	=> VK_MULTIPLY,
      OsCode::KEY_LEFTALT     	=> VK_LMENU,
      OsCode::KEY_SPACE       	=> VK_SPACE,
      OsCode::KEY_CAPSLOCK    	=> VK_CAPITAL,
      OsCode::KEY_F1          	=> VK_F1,OsCode::KEY_F2  	=> VK_F2,OsCode::KEY_F3	=> VK_F3,OsCode::KEY_F4	=> VK_F4,OsCode::KEY_F5	=> VK_F5,OsCode::KEY_F6	=> VK_F6,OsCode::KEY_F7	=> VK_F7,OsCode::KEY_F8	=> VK_F8,OsCode::KEY_F9	=> VK_F9,OsCode::KEY_F10	=> VK_F10,
      OsCode::KEY_F11         	=> VK_F11,OsCode::KEY_F12	=> VK_F12,
      OsCode::KEY_NUMLOCK     	=> VK_NUMLOCK,
      OsCode::KEY_CLEAR       	=> VK_CLEAR,
      OsCode::KEY_SCROLLLOCK  	=> VK_SCROLL,
      OsCode::KEY_KP0         	=> VK_NUMPAD0,
      OsCode::KEY_KP1         	=> VK_NUMPAD1,OsCode::KEY_KP2	=> VK_NUMPAD2,OsCode::KEY_KP3	=> VK_NUMPAD3,OsCode::KEY_KP4	=> VK_NUMPAD4,OsCode::KEY_KP5	=> VK_NUMPAD5,OsCode::KEY_KP6	=> VK_NUMPAD6,OsCode::KEY_KP7	=> VK_NUMPAD7,OsCode::KEY_KP8	=> VK_NUMPAD8,OsCode::KEY_KP9	=> VK_NUMPAD9,
      OsCode::KEY_KPMINUS     	=> VK_SUBTRACT,
      OsCode::KEY_KPPLUS      	=> VK_ADD,
      OsCode::KEY_KPDOT       	=> VK_DECIMAL,
      OsCode::KEY_KPENTER     	=> VK_KPENTER_FAKE,
      OsCode::KEY_KPSLASH     	=> VK_DIVIDE,
      OsCode::KEY_RIGHTALT    	=> VK_RMENU,
      OsCode::KEY_HOME        	=> VK_HOME,
      OsCode::KEY_PAGEUP      	=> VK_PRIOR,
      OsCode::KEY_END         	=> VK_END,
      OsCode::KEY_DOWN        	=> VK_DOWN,OsCode::KEY_UP	=> VK_UP,OsCode::KEY_LEFT	=> VK_LEFT,OsCode::KEY_RIGHT	=> VK_RIGHT,
      OsCode::KEY_PAGEDOWN    	=> VK_NEXT,
      OsCode::KEY_INSERT      	=> VK_INSERT,
      OsCode::KEY_DELETE      	=> VK_DELETE,
      OsCode::KEY_MUTE        	=> VK_VOLUME_MUTE,
      OsCode::KEY_VOLUMEDOWN  	=> VK_VOLUME_DOWN,
      OsCode::KEY_VOLUMEUP    	=> VK_VOLUME_UP,
      OsCode::KEY_PAUSE       	=> VK_PAUSE,
      OsCode::KEY_LEFTMETA    	=> VK_LWIN,
      OsCode::KEY_RIGHTMETA   	=> VK_RWIN,
      OsCode::KEY_COMPOSE     	=> VK_APPS,
      OsCode::KEY_BACK        	=> VK_BROWSER_BACK,
      OsCode::KEY_FORWARD     	=> VK_BROWSER_FORWARD,
      OsCode::KEY_NEXTSONG    	=> VK_MEDIA_NEXT_TRACK,
      OsCode::KEY_PLAYPAUSE   	=> VK_MEDIA_PLAY_PAUSE,
      OsCode::KEY_PREVIOUSSONG	=> VK_MEDIA_PREV_TRACK,
      OsCode::KEY_STOP        	=> VK_MEDIA_STOP,
      OsCode::KEY_HOMEPAGE    	=> VK_BROWSER_HOME,
      OsCode::KEY_MAIL        	=> VK_LAUNCH_MAIL,
      OsCode::KEY_MEDIA       	=> VK_LAUNCH_MEDIA_SELECT,
      OsCode::KEY_REFRESH     	=> VK_BROWSER_REFRESH,
      OsCode::KEY_F13         	=> VK_F13,
      OsCode::KEY_F14         	=> VK_F14,
      OsCode::KEY_F15         	=> VK_F15,
      OsCode::KEY_F16         	=> VK_F16,
      OsCode::KEY_F17         	=> VK_F17,
      OsCode::KEY_F18         	=> VK_F18,
      OsCode::KEY_F19         	=> VK_F19,
      OsCode::KEY_F20         	=> VK_F20,
      OsCode::KEY_F21         	=> VK_F21,
      OsCode::KEY_F22         	=> VK_F22,
      OsCode::KEY_F23         	=> VK_F23,
      OsCode::KEY_F24         	=> VK_F24,
      OsCode::KEY_HANGEUL     	=> VK_HANGEUL,
      OsCode::KEY_HANJA       	=> VK_HANJA,
      OsCode::KEY_252         	=> VK_OEM_8,
      OsCode::KEY_102ND       	=> VK_OEM_102,
      OsCode::KEY_PLAY        	=> VK_PLAY,
      OsCode::KEY_PRINT       	=> VK_SNAPSHOT,
      OsCode::KEY_SEARCH      	=> VK_BROWSER_SEARCH,
      OsCode::KEY_FAVORITES   	=> VK_BROWSER_FAVORITES,
      OsCode::KEY_RO          	=> 0xC1,
      OsCode::KEY_HENKAN      	=> VK_CONVERT,
      OsCode::KEY_MUHENKAN    	=> VK_NONCONVERT,
      OsCode::BTN_LEFT        	=> VK_LBUTTON,
      OsCode::BTN_RIGHT       	=> VK_RBUTTON,
      OsCode::BTN_MIDDLE      	=> VK_MBUTTON,
      OsCode::BTN_SIDE        	=> VK_XBUTTON1,
      OsCode::BTN_EXTRA       	=> VK_XBUTTON2,
      osc                     	=> osc as u16,
    }
  }
}
