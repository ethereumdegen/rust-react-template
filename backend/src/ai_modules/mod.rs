use crate::types::openai_completion::OpenAiCallableFunction;

pub mod system_command_module;

pub trait AiSystemModule {
    fn get_system_message(&self) -> Option<String>;
    fn get_functions(&self) -> Vec<OpenAiCallableFunction>;
}
