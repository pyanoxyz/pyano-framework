// This code is adapted or copied from another location.
// Original source: [Abraxas-365/langchain-rustsrc/tools/tool.rs].
// Ensure that the usage complies with the original license terms, if applicable.

use std::error::Error;
use std::string::String;

use async_trait::async_trait;
use serde_json::{ json, Value };

#[async_trait]
pub trait Tool: Send + Sync {
    /// Returns the name of the tool.
    fn name(&self) -> String;

    /// Provides a description of what the tool does and when to use it.
    fn description(&self) -> String;
    /// This are the parametters for OpenAi-like function call.
    /// You should return a jsnon like this one
    /// ```json
    /// {
    //     "type": "function",
    //     "function": {
    //         "name": "retrieve_documents",
    //         "description": "Retrieve documents from the vector store based on the query.",
    //         "parameters": {
    //             "type": "object",
    //             "properties": {
    //                 "query": {
    //                     "type": "string"
    //                 }
    //             },
    //             "required": [
    //                 "query"
    //             ]
    //         }
    //     }
    // },
    ///
    /// If there s no implementation the defaul will be the self.description()
    ///```
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
                "properties": {
                "input": {
                    "type": "string",
                    "description":self.description()
                }
            },
            "required": ["input"]
        })
    }

    /// Processes an input string and executes the tool's functionality, returning a `Result`.
    ///
    /// This function utilizes `parse_input` to parse the input and then calls `run`.
    /// Its used by the Agent
    async fn call(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let input = self.parse_input(input).await;
        let json_result = self.run(input).await?;
        // Convert the JSON result to a string
        serde_json::to_string(&json_result).map_err(|e| e.into())
    }

    async fn json_call(&self, input: &str) -> Result<Value, Box<dyn Error>> {
        let input = self.parse_input(input).await;
        self.run(input).await
    }

    /// Executes the core functionality of the tool.
    ///
    /// Example implementation:
    /// ```rust,ignore
    /// async fn run(&self, input: Value) -> Result<String, Box<dyn Error>> {
    ///     let input_str = input.as_str().ok_or("Input should be a string")?;
    ///     self.simple_search(input_str).await
    /// }
    /// ```
    async fn run(&self, input: Value) -> Result<Value, Box<dyn Error>>;

    /// Parses the input string, which could be a JSON value or a raw string, depending on the LLM model.
    ///
    /// Implement this function to extract the parameters needed for your tool. If a simple
    /// string is sufficient, the default implementation can be used.
    async fn parse_input(&self, input: &str) -> Value {
        match serde_json::from_str::<Value>(input) {
            Ok(input) => input, // Return the parsed JSON as a `Value`
            Err(_) => json!({"query": input}), // Wrap the raw string into a JSON object
        }
    }
}
