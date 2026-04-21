//! Hook System
//!
//! 借鉴 Claude Code 的 Hook 系统，支持 pre/post triggers

pub mod trigger;
pub mod action;
pub mod system;

pub use trigger::HookTrigger;
pub use action::{HookAction, HookResult};
pub use system::HookSystem;