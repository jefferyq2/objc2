mod fixes;
#[path = "../generated/AppKit/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}
