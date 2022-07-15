pub mod clone;
pub use clone::Clone;

pub mod install;
pub use install::Install;

pub mod link;
pub use link::Link;

pub mod sync;
pub use sync::Sync;

pub trait Command {
  type Args;
  type Result;

  fn execute(&self, args: Self::Args) -> Self::Result;
}
