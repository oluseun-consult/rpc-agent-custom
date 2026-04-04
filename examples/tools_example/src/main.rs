use std::{collections::HashMap, sync::LazyLock};

use rig::{completion::ToolDefinition, tool::Tool};
use rpc_agent::{Error, Providers, ToolWrapper};

static TICKET_PRICE: LazyLock<HashMap<&str, u32>> = LazyLock::new(|| {
    HashMap::from([
        ("london", 799),
        ("paris", 899),
        ("tokyo", 1400),
        ("berlin", 499),
    ])
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = rpc_agent::AgentServerBuilder::new(
        5500,
        Providers::Ollama,
        "You are a helpful assistant for an Airline called FlightAI.
        Give short, courteous answers, no more than 1 sentence.
        Always be accurate. If you don't know the answer, say so.",
        "gpt-oss:20b",
    );

    let server = builder.build_with_tool(ToolWrapper::new(TicketPriceTool))?;

    server.run().await?;

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct Input {
    destination_city: String,
}

#[derive(Clone)]
pub struct TicketPriceTool;

impl TicketPriceTool {
    fn get_ticket_price(airport: &str) -> Result<u32, Error> {
        let airport = airport.to_lowercase();
        TICKET_PRICE
            .get(airport.as_str())
            .copied()
            .ok_or(Error::ProviderError(
                "No ticket price found for this destination".into(),
            ))
    }
}

impl Tool for TicketPriceTool {
    const NAME: &'static str = "get_ticket_price";

    type Error = Error;
    type Args = Input;
    type Output = u32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_ticket_price".to_string(),
            description: "Get the price of a return ticket to the destination city.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "destination_city": {
                        "type": "string",
                        "description": "The destination city"
                    }
                },
                "required": ["destination_city"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("Tools called for {}", &args.destination_city);
        let result = Self::get_ticket_price(&args.destination_city)?;
        Ok(result)
    }
}
