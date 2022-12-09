use crate::requester;
use crate::models::CompletionModels;
use crate::usage::Usage;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionChoice {
    text: String,
    index: i32,
    logprobs: Option<i32>,
    finish_reason: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Completion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage
}

/// Available parameters that can be sent with a completion request
pub struct Parameters<'a> {
    model: CompletionModels,
    query: Vec<(&'a str, Value)>
}

/// Function to create a completion request
///
/// Call it using [`build`] and add valid [`Parameters`] to the request to build a
/// completions request and close with `complete()`.
///
/// # Examples
///
/// ```rust
/// use oai_rs::{completions, models};
///
/// async {
///     let completions = completions::build(models::CompletionModels::TEXT_DAVINCI_003)
///         .prompt("Ice cream or cookies?")
///         .max_tokens(32)
///         .complete()
///         .await
///         .expect("Error Getting Response");
///
///         println!("{:?}", completions);
/// };
/// ```
pub fn build<'a>(model: CompletionModels) -> Parameters<'a> {
    Parameters {
        model,
        query: Vec::new()
    }
}

impl<'a> Parameters<'a> {

    /// The prompt to generate completions for, encoded as a string.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-prompt)
    pub fn prompt(mut self, input: &'a str) -> Self {
        self.query.push(("prompt", json!(input)));
        self
    }

    /// The prompts to generate completions for,
    /// array of strings, array of tokens, or array of token arrays.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-prompt)
    pub fn prompts(mut self, input: &'a Vec<&str>) -> Self {
        self.query.push(("prompt", json!(input)));
        self
    }

    /// The suffix that comes after a completion of inserted text.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-suffix)
    pub fn suffix(mut self, input: &'a str) -> Self {
        self.query.push(("suffix", json!(input)));
        self
    }

    /// What sampling temperature to use. Higher values means the
    /// model will take more risks. Try 0.9 for more creative
    /// applications, and 0 (argmax sampling) for ones with a
    /// well-defined answer.
    ///
    /// We generally recommend altering this or top_p but not both.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-temperature)
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
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-top_p)
    pub fn top_p(mut self, input: &'a f32) -> Self {
        self.query.push(("top_p", json!(input)));
        self
    }

    /// How many completions to generate for each prompt.
    ///
    /// # Safety
    ///
    /// Note: Because this parameter generates many completions, it can quickly
    /// consume your token quota. Use carefully and ensure that you have reasonable
    /// settings for max_tokens and stop.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-n)
    pub fn n(mut self, input: &'a u32) -> Self {
        self.query.push(("n", json!(input)));
        self
    }

    /// Include the log probabilities on the logprobs most likely tokens,
    /// as well the chosen tokens. For example, if logprobs is 5, the API
    /// will return a list of the 5 most likely tokens. The API will always
    /// return the logprob of the sampled token, so there may be up to
    /// logprobs+1 elements in the response.
    ///
    /// The maximum value for logprobs is 5.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-logprobs)
    pub fn logprobs(mut self, input: &'a u8) -> Self {
        self.query.push(("logprobs", json!(input)));
        self
    }

    /// Echo back the prompt in addition to the completion
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-echo)
    pub fn echo(mut self, input: &'a bool) -> Self {
        self.query.push(("echo", json!(input)));
        self
    }

    /// One sequence where the API will stop generating further
    /// tokens. The returned text will not contain the stop sequence.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-stop)
    pub fn stop(mut self, input: &'a str) -> Self {
        self.query.push(("stop", json!(input)));
        self
    }

    /// Up to 4 sequences where the API will stop generating further
    /// tokens. The returned text will not contain the stop sequence.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-stop)
    pub fn stops(mut self, input: &'a Vec<&str>) -> Self {
        self.query.push(("stop", json!(input)));
        self
    }

    /// A unique identifier representing your end-user, which can help
    /// OpenAI to monitor and detect abuse.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-user)
    pub fn user(mut self, input: &'a str) -> Self {
        self.query.push(("user", json!(input)));
        self
    }

    /// The maximum number of tokens to generate in the completion.
    ///
    /// The token count of your prompt plus max_tokens cannot exceed
    /// the model's context length. Most models have a context length
    /// of 2048 tokens (except for the newest models, which support 4096).
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-max_tokens)
    pub fn max_tokens(mut self, input: u16) -> Self {
        self.query.push(("max_tokens", json!(input)));
        self
    }

    /// Number between -2.0 and 2.0. Positive values penalize new tokens
    /// based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-presence_penalty)
    pub fn presence_penalty(mut self, input: &'a f32) -> Self {
        self.query.push(("presence_penalty", json!(input)));
        self
    }

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based
    /// on their existing frequency in the text so far, decreasing the model's
    /// likelihood to repeat the same line verbatim.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-frequency_penalty)
    pub fn frequency_penalty(mut self, input: &'a f32) -> Self {
        self.query.push(("frequency_penalty", json!(input)));
        self
    }

    /// Generates best_of completions server-side and returns the "best"
    /// (the one with the highest log probability per token). Results
    /// cannot be streamed.
    ///
    /// When used with `n`, best_of controls the number of candidate completions
    /// and `n` specifies how many to return – best_of must be greater than `n`.
    ///
    /// # Safety
    ///
    /// **Note:** Because this parameter generates many completions, it can
    /// quickly consume your token quota. Use carefully and ensure that you
    /// have reasonable settings for max_tokens and stop.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-best_of)
    pub fn best_of(mut self, input: &'a u32) -> Self {
        self.query.push(("best_of", json!(input)));
        self
    }

    // TODO logit_bias

    /// Complete the request and send
    pub async fn complete(self) -> Result<Completion, StatusCode> {

        let mut map = HashMap::new();
        map.insert("model", json!(self.model.as_string()));
        for (k, v) in self.query.into_iter() {
            map.insert(k, v);
        }

        let response: Result<Completion, StatusCode> = requester::completions(map).await;

        match response {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{completions, models};

    #[tokio::test]
    async fn completions_builder() {
        let completions = completions::build(models::CompletionModels::TEXT_DAVINCI_003)
            .prompt("What happened in 2020?")
            .user("oai-rs")
            .max_tokens(32)
            .complete()
            .await
            .expect("Error Getting Response");

        println!("{:?}", completions);

        assert_eq!(1, 1)
    }
}