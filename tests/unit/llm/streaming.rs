//! Streaming 类型单元测试
//! 测试流式响应的累积和解析

use mcorcode::llm::{PartialToolCall, StreamingChunk, StreamingState};
use mcorcode::schema::FinishReason;

/// 测试 StreamingChunk 的创建
/// new() 应创建空的 chunk
#[test]
fn test_streaming_chunk_new() {
    let chunk = StreamingChunk::new("Hello");
    assert_eq!(chunk.content, "Hello");
    assert!(chunk.tool_calls.is_none());
    assert!(chunk.finish_reason.is_none());
}

/// 测试 StreamingChunk 的 is_final 方法
/// 有 finish_reason 时应返回 true
#[test]
fn test_streaming_chunk_is_final() {
    let chunk = StreamingChunk::new("Done").with_finish_reason(FinishReason::Stop);
    assert!(chunk.is_final());
}

/// 测试 StreamingChunk 的 is_final 方法（无 finish_reason）
/// 无 finish_reason 时应返回 false
#[test]
fn test_streaming_chunk_not_final() {
    let chunk = StreamingChunk::new("Content");
    assert!(!chunk.is_final());
}

/// 测试 StreamingChunk 的 has_content 方法
/// 有内容时应返回 true
#[test]
fn test_streaming_chunk_has_content() {
    let chunk = StreamingChunk::new("text");
    assert!(chunk.has_content());
}

/// 测试 StreamingChunk 的 has_content 方法（空内容）
/// 空内容时应返回 false
#[test]
fn test_streaming_chunk_empty_content() {
    let chunk = StreamingChunk::new("");
    assert!(!chunk.has_content());
}

/// 测试 StreamingChunk 的累积功能
/// accumulate 应合并内容和工具调用
#[test]
fn test_streaming_chunk_accumulate() {
    let mut chunk1 = StreamingChunk::new("Hello");
    let chunk2 = StreamingChunk::new(" World");

    chunk1.accumulate(&chunk2);
    assert_eq!(chunk1.content, "Hello World");
}

/// 测试 PartialToolCall 的创建
/// new() 应创建空的 partial tool call
#[test]
fn test_partial_tool_call_new() {
    let ptc = PartialToolCall::new(0);
    assert_eq!(ptc.index, 0);
    assert!(ptc.id.is_none());
    assert!(ptc.name.is_none());
    assert!(ptc.arguments.is_empty());
}

/// 测试 PartialToolCall 的 is_complete 方法
/// 有 id、name 和 arguments 时应返回 true
#[test]
fn test_partial_tool_call_is_complete() {
    let ptc = PartialToolCall::new(0)
        .with_id("call_123")
        .with_name("bash");
    let mut ptc = ptc;
    ptc.append_arguments("{\"cmd\": \"ls\"}");

    assert!(ptc.is_complete());
}

/// 测试 PartialToolCall 的 is_complete 方法（不完整）
/// 缺少字段时应返回 false
#[test]
fn test_partial_tool_call_not_complete() {
    let ptc = PartialToolCall::new(0).with_id("call_123");

    assert!(!ptc.is_complete());
}

/// 测试 PartialToolCall 的参数解析
/// try_parse_arguments 应正确解析 JSON
#[test]
fn test_partial_tool_call_parse_arguments() {
    let ptc = PartialToolCall::new(0).with_id("call_1").with_name("read");
    let mut ptc = ptc;
    ptc.append_arguments("{\"path\": \"/test.txt\"}");

    let args = ptc.try_parse_arguments();
    assert!(args.is_some());
    assert_eq!(args.unwrap()["path"], "/test.txt");
}

/// 测试 PartialToolCall 的参数解析失败
/// 无效 JSON 应返回 None
#[test]
fn test_partial_tool_call_parse_invalid() {
    let ptc = PartialToolCall::new(0).with_id("call_1").with_name("read");
    let mut ptc = ptc;
    ptc.append_arguments("invalid json");

    let args = ptc.try_parse_arguments();
    assert!(args.is_none());
}

/// 测试 StreamingState 的创建
/// new() 应创建空状态
#[test]
fn test_streaming_state_new() {
    let state = StreamingState::new();
    assert!(state.is_streaming());
}

/// 测试 StreamingState 的 finalize 方法
/// finalize 应返回累积的结果
#[test]
fn test_streaming_state_finalize() {
    let mut state = StreamingState::new();
    state.push(StreamingChunk::new("Hello"));
    state.push(StreamingChunk::new(" World").with_finish_reason(FinishReason::Stop));

    let result = state.finalize();
    assert_eq!(result.content, "Hello World");
    assert!(result.finish_reason.is_some());
}

/// 测试 StreamingState 的 is_streaming 方法
/// 未完成时应返回 true
#[test]
fn test_streaming_state_is_streaming() {
    let mut state = StreamingState::new();
    state.push(StreamingChunk::new("Partial"));

    assert!(state.is_streaming());
}

/// 测试 StreamingState 的 is_streaming 方法（已完成）
/// 有 finish_reason 时应返回 false
#[test]
fn test_streaming_state_not_streaming() {
    let mut state = StreamingState::new();
    state.push(StreamingChunk::new("Done").with_finish_reason(FinishReason::Stop));

    assert!(!state.is_streaming());
}
