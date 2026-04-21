//! Permission System
//!
//! 借鉴 Claude Code 的 5 种权限模式：
//! - Default: 每次工具调用都询问用户
//! - AcceptEdits: 自动批准文件修改，询问 shell 命令
//! - AcceptAll: 自动批准所有工具
//! - PlanMode: 只读模式，不执行任何工具
//! - Sandbox: 在隔离环境执行

pub mod checker;
pub mod mode;
pub mod policy;

pub use checker::{PermissionAction, PermissionChecker, PermissionResult};
pub use mode::PermissionMode;
pub use policy::PermissionPolicy;
