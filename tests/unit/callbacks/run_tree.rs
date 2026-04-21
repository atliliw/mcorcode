//! RunTree 运行追踪单元测试

use mcorcode::callbacks::{RunTree, RunType};

/// 测试 RunTree 的创建
/// 新建的 RunTree 应有唯一的 UUID、开始时间、无结束时间
#[test]
fn test_run_tree_new() {
    let run = RunTree::new(RunType::LLM, "test_llm_run");
    assert!(!run.run_id.is_empty());
    assert_eq!(run.name, "test_llm_run");
    assert!(run.end_time.is_none());
    assert!(run.parent_run_id.is_none());
}

/// 测试不同 RunType 的创建
/// 包括 LLM、Tool、Chain、Agent 四种类型
#[test]
fn test_run_types() {
    let llm_run = RunTree::new(RunType::LLM, "llm");
    assert!(matches!(llm_run.run_type, RunType::LLM));

    let tool_run = RunTree::new(RunType::Tool, "bash_tool");
    assert!(matches!(tool_run.run_type, RunType::Tool));

    let chain_run = RunTree::new(RunType::Chain, "my_chain");
    assert!(matches!(chain_run.run_type, RunType::Chain));

    let agent_run = RunTree::new(RunType::Agent, "my_agent");
    assert!(matches!(agent_run.run_type, RunType::Agent));
}

/// 测试带父级 ID 的 RunTree 创建
/// 父级 ID 应正确设置，用于追踪嵌套调用
#[test]
fn test_run_tree_with_parent() {
    let parent_id = "parent-run-123";
    let run = RunTree::with_parent(RunType::Tool, "child_tool", parent_id);
    assert_eq!(run.parent_run_id, Some(parent_id.to_string()));
}

/// 测试 RunTree 的结束方法
/// end() 后应有结束时间，duration_ms 可计算时长
#[test]
fn test_run_tree_end() {
    let mut run = RunTree::new(RunType::LLM, "test");

    // 结束前无 duration
    assert!(run.duration_ms().is_none());

    // 结束运行
    run.end();

    // 结束后应有结束时间和 duration
    assert!(run.end_time.is_some());
    assert!(run.duration_ms().is_some());

    // duration 应为非负数
    let duration = run.duration_ms().unwrap();
    assert!(duration >= 0);
}

/// 测试 RunTree 的 UUID 格式
/// run_id 应是有效的 UUID v4 格式字符串
#[test]
fn test_run_tree_uuid_format() {
    let run = RunTree::new(RunType::LLM, "test");

    // UUID v4 格式: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    // y 应为 8, 9, a, 或 b
    let parts: Vec<&str> = run.run_id.split('-').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0].len(), 8);
    assert_eq!(parts[1].len(), 4);
    assert_eq!(parts[2].len(), 4);
    assert_eq!(parts[3].len(), 4);
    assert_eq!(parts[4].len(), 12);
}

/// 测试 RunTree 开始时间正确设置
/// start_time 应接近当前时间
#[test]
fn test_run_tree_start_time() {
    use chrono::Utc;

    let before = Utc::now();
    let run = RunTree::new(RunType::LLM, "test");
    let after = Utc::now();

    // start_time 应在 before 和 after 之间
    assert!(run.start_time >= before);
    assert!(run.start_time <= after);
}
