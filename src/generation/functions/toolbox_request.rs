use crate::generation::chat::request::ChatMessageRequest;
use crate::generation::chat::ChatMessage;
use crate::generation::functions::tools::Toolbox;
use crate::generation::{options::GenerationOptions, parameters::FormatType};
use std::sync::Arc;

pub struct ToolboxCallRequest<'a> {
    pub chat: ChatMessageRequest,
    pub toolbox: &'a dyn Toolbox,
}
