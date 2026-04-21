use std::collections::HashMap;
use std::sync::Arc;

use langchainrust::core::language_models::BaseChatModel;
use langchainrust::core::tools::BaseTool;
use langchainrust::langgraph::{
    Checkpointer, CompiledGraph, FileCheckpointer, MemoryCheckpointer, StateGraph, END, START,
};
use langchainrust::language_models::openai::OpenAIError;

use crate::agent::graph::nodes::{CompactNode, LlmNode, ToolsNode};
use crate::agent::graph::router::{AgentRouter, CompactRouter, ToolsRouter};
use crate::agent::graph::state::McorcodeState;
use crate::permission::{PermissionChecker, PermissionMode};

pub struct AgentGraphBuilder {
    llm: Arc<dyn BaseChatModel<Error = OpenAIError>>,
    tools: HashMap<String, Arc<dyn BaseTool>>,
    permission_checker: PermissionChecker,
    max_iterations: usize,
    checkpointer: Option<String>,
    compact_threshold: usize,
}

impl AgentGraphBuilder {
    pub fn new(llm: Arc<dyn BaseChatModel<Error = OpenAIError>>) -> Self {
        Self {
            llm,
            tools: HashMap::new(),
            permission_checker: PermissionChecker::new(PermissionMode::Default),
            max_iterations: 25,
            checkpointer: None,
            compact_threshold: 50,
        }
    }

    pub fn with_tool(mut self, tool: Arc<dyn BaseTool>) -> Self {
        self.tools.insert(tool.name().to_string(), tool);
        self
    }

    pub fn with_tools(mut self, tools: Vec<Arc<dyn BaseTool>>) -> Self {
        for tool in tools {
            self.tools.insert(tool.name().to_string(), tool);
        }
        self
    }

    pub fn with_permission_mode(mut self, mode: PermissionMode) -> Self {
        self.permission_checker = PermissionChecker::new(mode);
        self
    }

    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    pub fn with_checkpointer(mut self, path: impl Into<String>) -> Self {
        self.checkpointer = Some(path.into());
        self
    }

    pub fn with_compact_threshold(mut self, threshold: usize) -> Self {
        self.compact_threshold = threshold;
        self
    }

    pub fn build(self) -> CompiledGraph<McorcodeState> {
        let mut graph = StateGraph::<McorcodeState>::new();

        let llm_node = LlmNode::new(self.llm.clone());
        graph.add_async_node("llm", llm_node);

        let tools_node = ToolsNode::new(self.tools.clone(), Arc::new(self.permission_checker));
        graph.add_async_node("tools", tools_node);

        let compact_node = CompactNode::new(self.compact_threshold);
        graph.add_async_node("compact", compact_node);

        graph.add_edge(START, "llm");

        graph.set_conditional_router("llm_router", AgentRouter);
        graph.add_conditional_edges(
            "llm",
            "llm_router",
            HashMap::from([
                ("tools".to_string(), "tools".to_string()),
                ("compact".to_string(), "compact".to_string()),
                ("end".to_string(), END.to_string()),
            ]),
            Some(END.to_string()),
        );

        graph.set_conditional_router("tools_router", ToolsRouter);
        graph.add_conditional_edges(
            "tools",
            "tools_router",
            HashMap::from([
                ("llm".to_string(), "llm".to_string()),
                ("end".to_string(), END.to_string()),
            ]),
            Some(END.to_string()),
        );

        graph.set_conditional_router("compact_router", CompactRouter);
        graph.add_conditional_edges(
            "compact",
            "compact_router",
            HashMap::from([("llm".to_string(), "llm".to_string())]),
            None,
        );

        let compiled = graph.compile().unwrap();

        let compiled = if let Some(path) = self.checkpointer {
            compiled.with_checkpointer(FileCheckpointer::new(path))
        } else {
            compiled.with_checkpointer(MemoryCheckpointer::new())
        };

        compiled.with_recursion_limit(self.max_iterations)
    }
}
