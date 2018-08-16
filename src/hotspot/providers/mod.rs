#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "osx")]
mod osx;

pub mod prelude {
  #[cfg(target_os = "linux")]
  pub use super::linux::*;
  #[cfg(target_os = "osx")]
  pub use super::osx::*;
  #[cfg(target_os = "windows")]
  pub use super::windows::*;
}
