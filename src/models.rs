use crate::requester;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ModelPermissions {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: Option<String>,
    pub owned_by: Option<String>,
    pub permission: Option<Vec<ModelPermissions>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootModel {
    pub data: Vec<Model>
}

pub enum CompletionModels {
    /// Uses the text-davinci-003 model.
    ///
    /// Most capable GPT-3 model. Can do any task the other models can do,
    /// often with higher quality, longer output and better instruction-following.
    ///
    /// **Strengths:** Complex intent, cause and effect, summarization for audience.
    #[allow(non_camel_case_types)]
    TEXT_DAVINCI_003,
    /// Uses the text-davinci-002 model.
    ///
    /// Second generation model. Can do any task earlier models can do,
    /// often with less context.
    ///
    /// **Strengths:** Complex intent, cause and effect, summarization for audience.
    #[allow(non_camel_case_types)]
    TEXT_DAVINCI_002,
    /// Uses the text-davinci-001 model.
    ///
    /// Older version of the most advanced model. Can do any task earlier models can do,
    /// often with less context.
    ///
    /// **Strengths:** Complex intent, cause and effect, summarization for audience.
    #[allow(non_camel_case_types)]
    TEXT_DAVINCI_001,
    /// Uses the text-curie-001 model.
    ///
    /// Very capable but faster and lower cost than the text-davinci-003 model
    ///
    /// **Strengths:** Language translation, complex classification, text sentiment, summarization.
    #[allow(non_camel_case_types)]
    TEXT_CURIE_001,
    /// Uses the text-babbage-001 model.
    ///
    /// Very faster and lower cost.
    ///
    /// **Strengths:** Moderate classification, semantic search classification
    #[allow(non_camel_case_types)]
    TEXT_BABBAGE_001,
    /// Uses the text-ada-001 model.
    ///
    /// Capable of simple tasks. This is the fastest GPT3 model and lowest cost.
    ///
    /// **Strengths:** Parsing text, simple classification, address correction, keywords.
    #[allow(non_camel_case_types)]
    TEXT_ADA_001,
    /// Use a model through it's identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::models;
    ///
    /// let completionModel = models::CompletionModel::from_str("text-davinci-003");
    /// ```
    #[allow(non_camel_case_types)]
    from_str(&'static str)
}

impl CompletionModels {
    pub fn as_string(&self) -> String {
        match &*self {
            CompletionModels::TEXT_DAVINCI_003 => String::from("text-davinci-003"),
            CompletionModels::TEXT_DAVINCI_002 => String::from("text-davinci-002"),
            CompletionModels::TEXT_DAVINCI_001 => String::from("text-davinci-001"),
            CompletionModels::TEXT_CURIE_001 => String::from("text-curie-001"),
            CompletionModels::TEXT_BABBAGE_001 => String::from("text-babbage-001"),
            CompletionModels::TEXT_ADA_001 => String::from("text-ada-001"),
            CompletionModels::from_str(t) => String::from(*t)
        }
    }
}

pub enum EditModels {
    /// Uses the text-davinci-edit-001 model.
    ///
    /// Specialised model of the most capable GPT-3 model. The model will attempt
    /// to modify the supplied text in accordance to some instructions.
    ///
    /// **Strengths:** Complex intent, cause and effect, summarization for audience.
    #[allow(non_camel_case_types)]
    TEXT_DAVINCI_EDIT_001,
    /// Use a model through it's identifier
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::models;
    ///
    /// let editModel = models::EditModels::from_str("text-davinci-edit-001");
    /// ```
    #[allow(non_camel_case_types)]
    from_str(&'static str)
}

impl EditModels {
    pub fn as_string(&self) -> String {
        match &*self {
            EditModels::TEXT_DAVINCI_EDIT_001 => String::from("text-davinci-edit-001"),
            EditModels::from_str(t) => String::from(*t)
        }
    }
}

/// Request a list of all currently available models from the API
pub async fn list() -> Result<Vec<Model>, StatusCode> {
    let response: Result<RootModel, StatusCode> = requester::models(None).await;

    match response {
        Ok(t) => Ok(t.data),
        Err(e) => Err(e),
    }
}

/// Return information for a specific model by its identifier
pub async fn get(model_name: String) -> Result<Model, StatusCode> {
    let response: Result<Model, StatusCode> = requester::models(Some(model_name)).await;

    match response {
        Ok(t) => Ok(t),
        Err(e) => Err(e),
    }
}