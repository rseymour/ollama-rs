use crate::generation::chat::request::ChatMessageRequest;
use crate::generation::chat::ChatMessage;
use crate::generation::functions::tools::Toolbox;
use crate::generation::{options::GenerationOptions, parameters::FormatType};
use std::sync::Arc;

pub struct ToolboxCallRequest<'a> {
    pub chat: ChatMessageRequest,
    pub toolbox: &'a dyn Toolbox,
}

impl<'a> ToolboxCallRequest<'a> {
    pub fn new(model_name: String, toolbox: &'a dyn Toolbox, messages: Vec<ChatMessage>) -> Self {
        let chat = ChatMessageRequest::new(model_name, messages);
        Self { chat, toolbox }
    }

    /// Additional model parameters listed in the documentation for the Modelfile
    pub fn options(mut self, options: GenerationOptions) -> Self {
        self.chat.options = Some(options);
        self
    }

    /// The full prompt or prompt template (overrides what is defined in the Modelfile)
    pub fn template(mut self, template: String) -> Self {
        self.chat.template = Some(template);
        self
    }

    // The format to return a response in. Currently the only accepted value is `json`
    pub fn format(mut self, format: FormatType) -> Self {
        self.chat.format = Some(format);
        self
    }
}
