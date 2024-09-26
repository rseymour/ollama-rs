use ollama_rs::{
    generation::{
        chat::ChatMessage,
        completion::{request::GenerationRequest, GenerationContext, GenerationResponseStream},
        functions::{
            tools::{DDGSearcher, Scraper},
            FunctionCallRequest, LlamaFunctionCall,
        },
    },
    Ollama,
};
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MODEL: &str = "llama3.1:latest";
    let ollama = Ollama::default();
    let scraper_tool = Arc::new(Scraper::new());
    let ddg_search_tool = Arc::new(DDGSearcher::new());
    let parser = Arc::new(LlamaFunctionCall {});

    let mut stdout = stdout();

    let mut context: Option<GenerationContext> = None;

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // ------------
        let result = ollama
            .send_function_call(
                FunctionCallRequest::new(
                    MODEL.to_string(),
                    vec![ddg_search_tool.clone()],
                    vec![ChatMessage::user(input.to_string())],
                ),
                parser.clone(),
            )
            .await
            .unwrap();
        //--------

        if let Some(res) = result.message {
            let return_value: Vec<Value> = serde_json::from_str(&res.content)?;
            for search_result in return_value.iter() {
                let resp = format!("{} at {}", search_result["title"], search_result["link"]);
                stdout.write_all(resp.as_bytes()).await?;
            }
            stdout.flush().await?;
        }
    }

    Ok(())
}
