// pub use reexports for use in other files
pub use crate::helper::print_type_of as pt;
use crate::helper::*;
pub use std::any::type_name; // for type_of
pub use std::println as p; // requires text editor's syntax theme override to retain syntax highlighting

#[macro_export]
macro_rules! pb { // println! during build
  ($($tokens:tt)*) => {println!("cargo:warning={}", format!($($tokens)*))}}
