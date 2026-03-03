use serde::{Deserialize, Serialize, de::value::Error};
use rig::{completion::ToolDefinition, tool::Tool};
use serde_json::json;

#[derive(Deserialize)]
pub struct OperationArgs {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Multiplier;

impl Tool for Multiplier {
    const NAME: &'static str = "multiply";
    type Error = Error;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(
            &self,
            _prompt: String,
    ) -> ToolDefinition {
        ToolDefinition {
            name: "multiply".to_string(),
            description: "Multiply x and y together".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first number to multiply"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second number to multiply"
                    }
                },
                "required": ["x", "y"],
            }),
        }
    }

    async fn call(
            &self,
            args: Self::Args,
        ) -> Result<Self::Output, Self::Error> {
        println!("[tool-call] Multiplying {} and {}", args.x, args.y);
        let result: i32 = args.x * args.y;
        Ok(result)
    }
}