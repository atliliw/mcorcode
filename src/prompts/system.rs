use crate::prompts::PromptTemplate;

pub fn system_prompt() -> PromptTemplate {
    PromptTemplate::from_template(
        "You are a helpful AI assistant with access to tools for file operations, \
         code editing, and shell commands. Use these tools to help the user accomplish \
         their tasks. Always explain what you're doing before taking action.",
    )
}

pub fn agent_prompt() -> PromptTemplate {
    PromptTemplate::from_template(
        "You are an AI agent that helps users with coding tasks.\n\
         Available tools:\n\
         - read_file: Read file contents\n\
         - write_file: Write content to a file\n\
         - edit_file: Edit file with string replacement\n\
         - bash: Execute shell commands\n\
         - grep: Search for patterns in files\n\
         - glob: Find files matching patterns\n\n\
         Current working directory: {workdir}\n\n\
         Task: {input}",
    )
}

pub fn react_prompt() -> PromptTemplate {
    PromptTemplate::from_template(
        "Answer the following questions as best you can.\n\n\
         You have access to the following tools:\n{tools}\n\n\
         Use the following format:\n\n\
         Thought: I should consider what to do\n\
         Action: the action to take, should be one of [{tool_names}]\n\
         Action Input: the input to the action\n\
         Observation: the result of the action\n\
         ... (this Thought/Action/Action Input/Observation can repeat N times)\n\
         Thought: I now know the final answer\n\
         Final Answer: the final answer to the original input question\n\n\
         Begin!\n\n\
         Question: {input}",
    )
}
