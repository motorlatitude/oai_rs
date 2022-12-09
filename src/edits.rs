use crate::requester;
use crate::models::EditModels;
use crate::usage::Usage;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct EditChoice {
    text: String,
    index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edit {
    pub object: String,
    pub created: u64,
    pub choices: Vec<EditChoice>,
    pub usage: Usage
}

/// Available parameters that can be sent with an edit request
pub struct Parameters<'a> {
    model: EditModels,
    instruction: String,
    query: Vec<(&'a str, Value)>
}

/// Function to create a edit request
///
/// Call it using [`build`] and add valid [`Parameters`] to the request to build a
/// edits request and close with `edit()`.
///
/// # Examples
///
/// ```rust
/// use oai_rs::{edits, models};
///
/// async {
///     let edits = edits::build(models::EditModels::TEXT_DAVINCI_EDIT_001, String::from("Fix the spelling and grammar mistakes"))
///         .input("Im bad at splling, hopefuly AI can fox this.")
///         .edit()
///         .await
///         .expect("Error Getting Response");
///
///         println!("{:?}", edits);
/// };
/// ```
pub fn build<'a>(model: EditModels, instruction: String) -> Parameters<'a> {
    Parameters {
        model,
        instruction,
        query: Vec::new()
    }
}

impl<'a> Parameters<'a> {

    /// The text to generate edits for, encoded as a string.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/edits/create#edits/create-input)
    pub fn input(mut self, input: &'a str) -> Self {
        self.query.push(("input", json!(input)));
        self
    }

    /// How many edits to generate for the input and instruction.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/edits/create#edits/create-n)
    pub fn n(mut self, input: &'a u32) -> Self {
        self.query.push(("n", json!(input)));
        self
    }

    /// What sampling temperature to use. Higher values means the
    /// model will take more risks. Try 0.9 for more creative
    /// applications, and 0 (argmax sampling) for ones with a
    /// well-defined answer.
    ///
    /// We generally recommend altering this or top_p but not both.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/edits/create#edits/create-temperature)
    pub fn temperature(mut self, input: &'a f32) -> Self {
        self.query.push(("temperature", json!(input)));
        self
    }

    /// An alternative to sampling with `temperature`, called
    /// nucleus sampling, where the model considers the results
    /// of the tokens with `top_p` probability mass. So 0.1
    /// means only the tokens comprising the top 10% probability
    /// mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/edits/create#edits/create-top_p)
    pub fn top_p(mut self, input: &'a f32) -> Self {
        self.query.push(("top_p", json!(input)));
        self
    }

    /// Complete the request and send
    pub async fn edit(self) -> Result<Edit, StatusCode> {

        let mut map = HashMap::new();
        map.insert("model", json!(self.model.as_string()));
        map.insert("instruction", json!(self.instruction));
        for (k, v) in self.query.into_iter() {
            map.insert(k, v);
        }

        let response: Result<Edit, StatusCode> = requester::edits(map).await;

        match response {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }
}