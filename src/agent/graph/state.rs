//! Mcorcode Agent State
//!
//! 定义 LangGraph StateGraph 的状态结构，替代 Claude Code 的 mutableMessages。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::schema::{Message, MessageType, ToolCall};
use langchainrust::langgraph::StateSchema;

/// Mcorcode Agent 状态
///
/// 替代 Claude Code 的 mutableMessages 数组，作为 LangGraph 的状态。
/// 包含消息历史、工具调用队列、迭代计数等。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McorcodeState {
    /// 会话 ID
    pub session_id: String,

    /// 消息历史（替代 Claude Code mutableMessages）
    pub messages: Vec<StateMessage>,

    /// 待执行的工具调用队列
    pub tool_calls_queue: Vec<ToolCall>,

    /// 当前迭代次数
    pub iteration: usize,

    /// 最大迭代次数
    pub max_iterations: usize,

    /// 是否继续执行
    pub should_continue: bool,

    /// 最终输出
    pub final_output: Option<String>,

    /// Thinking budget（借鉴 Claude Code extended thinking）
    /// None = 关闭, Some(N) = 允许 N tokens 的思考
    pub thinking_budget: Option<usize>,

    pub needs_compact: bool,

    /// 错误信息
    pub error: Option<String>,

    /// 元数据
    pub metadata: HashMap<String, serde_json::Value>,
}

impl StateSchema for McorcodeState {}

impl McorcodeState {
    /// 创建新状态
    pub fn new(user_input: String) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            messages: vec![StateMessage::human(user_input)],
            tool_calls_queue: vec![],
            iteration: 0,
            max_iterations: 25,
            should_continue: true,
            final_output: None,
            thinking_budget: None,
            needs_compact: false,
            error: None,
            metadata: HashMap::new(),
        }
    }

    pub fn resume(session_id: String, messages: Vec<StateMessage>) -> Self {
        Self {
            session_id,
            messages,
            tool_calls_queue: vec![],
            iteration: 0,
            max_iterations: 25,
            should_continue: true,
            final_output: None,
            thinking_budget: None,
            needs_compact: false,
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// 添加系统消息
    pub fn add_system(&mut self, content: String) {
        self.messages.push(StateMessage::system(content));
    }

    /// 添加用户消息
    pub fn add_human(&mut self, content: String) {
        self.messages.push(StateMessage::human(content));
    }

    /// 添加 AI 消息
    pub fn add_ai(&mut self, content: String) {
        self.messages.push(StateMessage::ai(content));
    }

    /// 添加 AI 消息（带工具调用）
    pub fn add_ai_with_tools(&mut self, content: String, tool_calls: Vec<ToolCall>) {
        self.messages
            .push(StateMessage::ai_with_tools(content, tool_calls));
    }

    /// 添加工具结果
    pub fn add_tool_result(&mut self, tool_call_id: String, result: String) {
        self.messages.push(StateMessage::tool(tool_call_id, result));
    }

    /// 添加执行步骤
    pub fn add_step(&mut self, step: StateStep) {
        // 步骤存储在 metadata 中
        let steps = self
            .metadata
            .get("steps")
            .and_then(|v| v.as_array())
            .map(|a| a.clone())
            .unwrap_or_default();

        let step_json = serde_json::to_value(&step).unwrap();
        let mut steps = steps;
        steps.push(step_json);

        self.metadata
            .insert("steps".to_string(), serde_json::Value::Array(steps));
    }

    /// 检查是否有待执行的工具调用
    pub fn has_tool_calls(&self) -> bool {
        !self.tool_calls_queue.is_empty()
    }

    /// 检查是否达到最大迭代
    pub fn reached_max_iterations(&self) -> bool {
        self.iteration >= self.max_iterations
    }

    /// 检查是否有最终输出
    pub fn has_final_output(&self) -> bool {
        self.final_output.is_some()
    }

    /// 检查是否需要压缩
    pub fn should_compact(&self) -> bool {
        self.needs_compact
    }

    /// 获取消息总数
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// 获取转换为 LLM 格式的消息
    pub fn to_llm_messages(&self) -> Vec<langchainrust::schema::Message> {
        self.messages
            .iter()
            .map(|m| m.to_message().to_langchain())
            .collect()
    }

    /// 增加迭代计数
    pub fn increment_iteration(&mut self) {
        self.iteration += 1;
    }

    /// 设置错误
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.should_continue = false;
    }

    /// 完成执行
    pub fn finish(&mut self, output: String) {
        self.final_output = Some(output);
        self.should_continue = false;
    }
}

/// 状态中的消息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMessage {
    /// 消息类型
    pub role: MessageRole,

    /// 消息内容
    pub content: String,

    /// 工具调用（AI 消息）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,

    /// 工具调用 ID（工具消息）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,

    /// 时间戳
    pub timestamp: DateTime<Utc>,
}

impl StateMessage {
    pub fn system(content: String) -> Self {
        Self {
            role: MessageRole::System,
            content,
            tool_calls: None,
            tool_call_id: None,
            timestamp: Utc::now(),
        }
    }

    pub fn human(content: String) -> Self {
        Self {
            role: MessageRole::Human,
            content,
            tool_calls: None,
            tool_call_id: None,
            timestamp: Utc::now(),
        }
    }

    pub fn ai(content: String) -> Self {
        Self {
            role: MessageRole::AI,
            content,
            tool_calls: None,
            tool_call_id: None,
            timestamp: Utc::now(),
        }
    }

    pub fn ai_with_tools(content: String, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: MessageRole::AI,
            content,
            tool_calls: Some(tool_calls),
            tool_call_id: None,
            timestamp: Utc::now(),
        }
    }

    pub fn tool(tool_call_id: String, content: String) -> Self {
        Self {
            role: MessageRole::Tool,
            content,
            tool_calls: None,
            tool_call_id: Some(tool_call_id),
            timestamp: Utc::now(),
        }
    }

    /// 转换为 schema::Message
    pub fn to_message(&self) -> Message {
        match self.role {
            MessageRole::System => Message::system(&self.content),
            MessageRole::Human => Message::human(&self.content),
            MessageRole::AI => {
                if let Some(tool_calls) = &self.tool_calls {
                    Message::ai_with_tool_calls(&self.content, tool_calls.clone())
                } else {
                    Message::ai(&self.content)
                }
            }
            MessageRole::Tool => Message::tool(
                &self.tool_call_id.clone().unwrap_or_default(),
                &self.content,
            ),
        }
    }
}

/// 消息角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    Human,
    AI,
    Tool,
}

/// 执行步骤记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateStep {
    /// 步骤 ID
    pub id: String,

    /// 工具名称
    pub tool: String,

    /// 工具输入
    pub input: serde_json::Value,

    /// 执行结果
    pub observation: String,

    /// 执行时间
    pub duration_ms: u64,

    /// 是否成功
    pub success: bool,

    /// 时间戳
    pub timestamp: DateTime<Utc>,
}

impl StateStep {
    pub fn new(
        tool: String,
        input: serde_json::Value,
        observation: String,
        duration_ms: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tool,
            input,
            observation,
            duration_ms,
            success: true,
            timestamp: Utc::now(),
        }
    }

    pub fn failed(tool: String, input: serde_json::Value, error: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tool,
            input,
            observation: error,
            duration_ms: 0,
            success: false,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcorcode_state_new() {
        let state = McorcodeState::new("Hello".to_string());
        assert!(!state.session_id.is_empty());
        assert_eq!(state.messages.len(), 1);
        assert_eq!(state.iteration, 0);
        assert_eq!(state.max_iterations, 25);
        assert!(state.should_continue);
        assert!(state.tool_calls_queue.is_empty());
    }

    #[test]
    fn test_mcorcode_state_has_tool_calls() {
        let state = McorcodeState::new("Hello".to_string());
        assert!(!state.has_tool_calls());
    }

    #[test]
    fn test_mcorcode_state_reached_max_iterations() {
        let mut state = McorcodeState::new("Hello".to_string());
        assert!(!state.reached_max_iterations());
        state.iteration = 25;
        assert!(state.reached_max_iterations());
    }

    #[test]
    fn test_mcorcode_state_has_final_output() {
        let state = McorcodeState::new("Hello".to_string());
        assert!(!state.has_final_output());
    }

    #[test]
    fn test_mcorcode_state_add_system() {
        let mut state = McorcodeState::new("Hello".to_string());
        state.add_system("You are helpful".to_string());
        assert_eq!(state.messages.len(), 2);
    }

    #[test]
    fn test_mcorcode_state_add_human() {
        let mut state = McorcodeState::new("Hello".to_string());
        state.add_human("How are you?".to_string());
        assert_eq!(state.messages.len(), 2);
    }

    #[test]
    fn test_mcorcode_state_add_ai() {
        let mut state = McorcodeState::new("Hello".to_string());
        state.add_ai("I'm fine".to_string());
        assert_eq!(state.messages.len(), 2);
    }

    #[test]
    fn test_mcorcode_state_increment_iteration() {
        let mut state = McorcodeState::new("Hello".to_string());
        state.increment_iteration();
        assert_eq!(state.iteration, 1);
    }

    #[test]
    fn test_mcorcode_state_set_error() {
        let mut state = McorcodeState::new("Hello".to_string());
        state.set_error("Something went wrong".to_string());
        assert!(state.error.is_some());
        assert!(!state.should_continue);
    }

    #[test]
    fn test_mcorcode_state_finish() {
        let mut state = McorcodeState::new("Hello".to_string());
        state.finish("Done".to_string());
        assert_eq!(state.final_output, Some("Done".to_string()));
        assert!(!state.should_continue);
    }

    #[test]
    fn test_state_message_system() {
        let msg = StateMessage::system("Hello".to_string());
        assert_eq!(msg.role, MessageRole::System);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_state_message_human() {
        let msg = StateMessage::human("Hello".to_string());
        assert_eq!(msg.role, MessageRole::Human);
    }

    #[test]
    fn test_state_message_ai() {
        let msg = StateMessage::ai("Response".to_string());
        assert_eq!(msg.role, MessageRole::AI);
    }

    #[test]
    fn test_state_message_tool() {
        let msg = StateMessage::tool("call_123".to_string(), "Result".to_string());
        assert_eq!(msg.role, MessageRole::Tool);
        assert_eq!(msg.tool_call_id, Some("call_123".to_string()));
    }

    #[test]
    fn test_state_step_new() {
        let step = StateStep::new(
            "bash".to_string(),
            serde_json::json!({"cmd": "ls"}),
            "output".to_string(),
            100,
        );
        assert_eq!(step.tool, "bash");
        assert!(step.success);
        assert_eq!(step.duration_ms, 100);
    }

    #[test]
    fn test_state_step_failed() {
        let step = StateStep::failed(
            "bash".to_string(),
            serde_json::json!({"cmd": "ls"}),
            "error".to_string(),
        );
        assert!(!step.success);
        assert_eq!(step.observation, "error");
    }
}
