use auto_toolbox::{add_to_toolbox, toolbox};
use ollama_rs::{
    generation::{
        chat::ChatMessage,
        functions::{toolbox_request::ToolboxCallRequest, LlamaFunctionCall, Toolbox},
    },
    Ollama,
};
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{stdout, AsyncWriteExt};

struct MyToolBox;

#[toolbox]
impl MyToolBox {
    #[add_to_toolbox("tightens a lid")] // this adds the following function to the toolbox with the description "tightens a lid"
    /// `rotations` - number of rotations
    pub fn lid_tightener(rotations: f32) -> Result<String, std::io::Error> {
        println!(
            "running some cool rotation code with rotations: {}",
            rotations
        );
        Ok(format!("this many rotations: {}", rotations))
    }
}

impl Toolbox for MyToolBox {
    fn get_impl_json(&self) -> Value {
        MyToolBox::get_impl_json()
    }

    fn call_value_fn(&self, tool_name: &str, tool_args: Value) -> Value {
        MyToolBox::call_value_fn(tool_name, tool_args)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MODEL: &str = "llama3.2:3b";
    let ollama = Ollama::default();
    let parser = Arc::new(LlamaFunctionCall {});
    let my_toolbox = MyToolBox;
    let mut stdout = stdout();

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
            .send_toolbox_call(
                &mut ToolboxCallRequest::new(
                    MODEL.to_string(),
                    &my_toolbox,
                    vec![ChatMessage::user(input.to_string())],
                ),
                parser.clone(), // this is messed up because it calls into parse() and I renamed it to parse_toolbox()
            )
            .await
            .unwrap();
        //--------

        if let Some(res) = result.message {
            let return_value: Value = serde_json::from_str(&res.content)?;
            let resp = format!("{}", return_value);
            stdout.write_all(resp.as_bytes()).await?;
            stdout.flush().await?;
        }
    }

    Ok(())
}
