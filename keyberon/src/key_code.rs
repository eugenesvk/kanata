//! Key code definitions.
pub const KEY_MAX: u16 = 850; // Used for switch opcode purposes. Keys should not exceed this amount

#[test]
fn keycode_max_test() {assert!((KeyCode::KeyMax as u16) < KEY_MAX);}

#[allow(missing_docs)]
/// Define a key code according to the HID specification.  Their names correspond to the american QWERTY layout.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(u16)]
pub enum KeyCode {
  /// The "no" key, a placeholder to express nothing.
  No = 0x00,
  /// Error if too much keys are pressed at the same time.
  ErrorRollOver,
  /// The POST fail error.
  PostFail,
  /// An undefined error occured.
  ErrorUndefined,
  /// `a` and `A`.
  A,B,C,D,E,F,G,H,I,J,K,L,
  M, // 0x10
  N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
  /// `1` and `!`.
  Kb1,
  /// `2` and `@`.
  Kb2,
  /// `3` and `#`.
  Kb3, // 0x20
  /// `4` and `$`.
  Kb4,
  /// `5` and `%`.
  Kb5,
  /// `6` and `^`.
  Kb6,
  /// `7` and `&`.
  Kb7,
  /// `8` and `*`.
  Kb8,
  /// `9` and `(`.
  Kb9,
  /// `0` and `)`.
  Kb0,
  Enter,
  Escape,
  BSpace,
  Tab,
  Space,
  /// `-` and `_`.
  Minus,
  /// `=` and `+`.
  Equal,
  /// `[` and `{`.
  LBracket,
  /// `]` and `}`.
  RBracket, // 0x30
  /// `\` and `|`.
  Bslash,
  /// Non-US `#` and `~` (Typically near the Enter key).
  NonUsHash,
  /// `;` and `:`.
  SColon,
  /// `'` and `"`.
  Quote,
  // How to have ` as code?
  /// \` and `~`.
  Grave,
  /// `,` and `<`.
  Comma,
  /// `.` and `>`.
  Dot,
  /// `/` and `?`.
  Slash,
  CapsLock,
  F1,F2,F3,F4,F5,F6,
  F7, // 0x40
  F8,F9,
  F10,F11,F12,
  PScreen,
  ScrollLock,
  Pause,
  PgDown,PgUp,
  Insert,Delete,
  Home,End,
  Down,Up,Right,
  Left, // 0x50
  NumLock,
  /// Keypad `/`
  KpSlash,
  /// Keypad `*`
  KpAsterisk,
  /// Keypad `-`.
  KpMinus,
  /// Keypad `+`.
  KpPlus,
  /// Keypad enter.
  KpEnter,
  /// Keypad 0
  Kp0,Kp1,Kp2,Kp3,Kp4,Kp5,Kp6,Kp7,
  Kp8, // 0x60
  Kp9,
  KpDot,
  /// Non-US `\` and `|` (Typically near the Left-Shift key)
  NonUsBslash,
  Application, // 0x65
  /// not a key, used for errors
  Power,
  /// Keypad `=`.
  KpEqual,
  F13,F14,F15,F16,F17,F18,F19,
  F20,
  F21, // 0x70
  F22,F23,F24,
  Execute,
  Help,
  Menu,
  Select,
  Stop,
  Again,
  Undo,
  Cut,
  Copy,
  Paste,
  Find,
  Mute,
  VolUp, // 0x80
  VolDown,
  /// Deprecated.
  LockingCapsLock,
  /// Deprecated.
  LockingNumLock,
  /// Deprecated.
  LockingScrollLock,
  /// Keypad `,`, also used for the brazilian keypad period (.) key.
  KpComma,
  /// Used on AS/400 keyboard
  KpEqualSign,
  Intl1,Intl2,Intl3,Intl4,Intl5,Intl6,Intl7,Intl8,Intl9,
  Lang1, // 0x90
  Lang2,Lang3,Lang4,Lang5,Lang6,Lang7,Lang8,Lang9,
  AltErase,
  SysReq,
  Cancel,
  Clear,
  Prior,
  Return,
  Separator,
  Out, // 0xA0
  Oper,
  ClearAgain,
  CrSel,
  ExSel,

  // According to QMK, 0xA5-0xDF are not usable on modern keyboards. The keys below are
  // not real keys for USB codes; they are used only for kanata.
  Wakeup, // 0xA5
  BrightnessDown,BrightnessUp,
  KbdIllumDown,KbdIllumUp,
  K0xAA,K0xAB,K0xAC,K0xAD,K0xAE,K0xAF,
  K0xB0,K0xB1,K0xB2,K0xB3,K0xB4,K0xB5,K0xB6,K0xB7,K0xB8,K0xB9,K0xBA,K0xBB,K0xBC,K0xBD,K0xBE,K0xBF,
  K0xC0,K0xC1,K0xC2,K0xC3,K0xC4,K0xC5,K0xC6,K0xC7,K0xC8,K0xC9,K0xCA,K0xCB,K0xCC,K0xCD,K0xCE,K0xCF,
  K0xD0,K0xD1,K0xD2,K0xD3,K0xD4,K0xD5,K0xD6,K0xD7,K0xD8,K0xD9,K0xDA,K0xDB,K0xDC,K0xDD,K0xDE,K0xDF,

  // Modifiers
  LCtrl=0xE0,RCtrl,
  LShift    ,RShift,
  LAlt      ,RAlt, // Right Alt (or Alt Gr)
  LGui      ,RGui, // Right GUI 0xE7 (the Windows key)

  // Unofficial
  MediaPlayPause = 0xE8,
  MediaPreviousSong,MediaNextSong,
  MediaStopCD,MediaEjectCD,
  MediaVolUp,MediaVolDown,MediaMute,
  MediaWWW, // 0xF0
  MediaBack,MediaForward,
  MediaStop,
  MediaFind,
  MediaScrollDown,MediaScrollUp,
  MediaEdit,
  MediaSleep,
  MediaCoffee,
  MediaRefresh,
  MediaCalc, // 0xFB

  K252,K253,K254,K255,K256,K257,K258,K259,
  K260,K261,K262,K263,K264,K265,K266,K267,K268,K269,
  K270,K271,K272,K273,K274,K275,K276,K277,K278,K279,
  K280,K281,K282,K283,K284,K285,K286,K287,K288,K289,
  K290,K291,K292,K293,K294,K295,K296,K297,K298,K299,
  K300,K301,K302,K303,K304,K305,K306,K307,K308,K309,
  K310,K311,K312,K313,K314,K315,K316,K317,K318,K319,
  K320,K321,K322,K323,K324,K325,K326,K327,K328,K329,
  K330,K331,K332,K333,K334,K335,K336,K337,K338,K339,
  K340,K341,K342,K343,K344,K345,K346,K347,K348,K349,
  K350,K351,K352,K353,K354,K355,K356,K357,K358,K359,
  K360,K361,K362,K363,K364,K365,K366,K367,K368,K369,
  K370,K371,K372,K373,K374,K375,K376,K377,K378,K379,
  K380,K381,K382,K383,K384,K385,K386,K387,K388,K389,
  K390,K391,K392,K393,K394,K395,K396,K397,K398,K399,
  K400,K401,K402,K403,K404,K405,K406,K407,K408,K409,
  K410,K411,K412,K413,K414,K415,K416,K417,K418,K419,
  K420,K421,K422,K423,K424,K425,K426,K427,K428,K429,
  K430,K431,K432,K433,K434,K435,K436,K437,K438,K439,
  K440,K441,K442,K443,K444,K445,K446,K447,K448,K449,
  K450,K451,K452,K453,K454,K455,K456,K457,K458,K459,
  K460,K461,K462,K463,K464,K465,K466,K467,K468,K469,
  K470,K471,K472,K473,K474,K475,K476,K477,K478,K479,
  K480,K481,K482,K483,K484,K485,K486,K487,K488,K489,
  K490,K491,K492,K493,K494,K495,K496,K497,K498,K499,
  K500,K501,K502,K503,K504,K505,K506,K507,K508,K509,
  K510,K511,K512,K513,K514,K515,K516,K517,K518,K519,
  K520,K521,K522,K523,K524,K525,K526,K527,K528,K529,
  K530,K531,K532,K533,K534,K535,K536,K537,K538,K539,
  K540,K541,K542,K543,K544,K545,K546,K547,K548,K549,
  K550,K551,K552,K553,K554,K555,K556,K557,K558,K559,
  K560,K561,K562,K563,K564,K565,K566,K567,K568,K569,
  K570,K571,K572,K573,K574,K575,K576,K577,K578,K579,
  K580,K581,K582,K583,K584,K585,K586,K587,K588,K589,
  K590,K591,K592,K593,K594,K595,K596,K597,K598,K599,
  K600,K601,K602,K603,K604,K605,K606,K607,K608,K609,
  K610,K611,K612,K613,K614,K615,K616,K617,K618,K619,
  K620,K621,K622,K623,K624,K625,K626,K627,K628,K629,
  K630,K631,K632,K633,K634,K635,K636,K637,K638,K639,
  K640,K641,K642,K643,K644,K645,K646,K647,K648,K649,
  K650,K651,K652,K653,K654,K655,K656,K657,K658,K659,
  K660,K661,K662,K663,K664,K665,K666,K667,K668,K669,
  K670,K671,K672,K673,K674,K675,K676,K677,K678,K679,
  K680,K681,K682,K683,K684,K685,K686,K687,K688,K689,
  K690,K691,K692,K693,K694,K695,K696,K697,K698,K699,
  K700,K701,K702,K703,K704,K705,K706,K707,K708,K709,
  K710,K711,K712,K713,K714,K715,K716,K717,K718,K719,
  K720,K721,K722,K723,K724,K725,K726,K727,K728,K729,
  K730,K731,K732,K733,K734,K735,K736,K737,K738,K739,
  K740,K741,K742,K743,K744,
  KeyMax,
}

use core::fmt;
impl fmt::Display for KeyCode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{:?}",self)
  }
}
