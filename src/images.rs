use crate::requester;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageURL {
    pub url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    pub created: u64,
    pub data: Vec<ImageURL>
}

pub struct GenerateParameters<'a> {
    prompt: String,
    query: Vec<(&'a str, Value)>
}

pub struct VariationParameters<'a> {
    image: String,
    query: Vec<(&'a str, Value)>
}

pub struct EditParameters<'a> {
    prompt: String,
    image: String,
    query: Vec<(&'a str, Value)>
}

/// Available parameters that can be sent with an image request
pub struct Parameters {}

/// Function to create a edit request
///
/// Call it using [`build`] and add valid [`Parameters`] to the request to build an
/// images request.
///
/// For images there are 3 types of requests, `generate`, `edits`, `variations`. From there requests
/// can be built the usual way, see Examples below.
///
/// # Examples
///
/// ```rust
/// use oai_rs::images;
///
/// async {
///     let images = images::build()
///         .generate(String::from("Modern SVG stroke gradient CPU in the shape of a brain icon"))
///         .n(&3)
///         .size("256x256")
///         .done()
///         .await;
///
///     println!("{:?}", images);
/// };
/// ```
///
pub fn build() -> Parameters {
    Parameters {}
}

/// Parameter to set the request type for the images endpoint either, `generate`, `edit` or `variations`.
impl Parameters {
    /// Genertes image(s) given a prompt.
    pub fn generate<'a>(self, prompt: String) -> GenerateParameters<'a> {
        GenerateParameters {
            prompt,
            query: Vec::new()
        }
    }

    /// Creates an edited or extended image given an original image and a prompt.
    pub fn edits<'a>(self, image: String, prompt: String) -> EditParameters<'a> {
        EditParameters {
            prompt,
            image,
            query: Vec::new()
        }
    }

    /// Creates a variation of a given image.
    pub fn variation<'a>(self, image: String) -> VariationParameters<'a> {
        VariationParameters {
            image,
            query: Vec::new()
        }
    }
}

impl<'a> GenerateParameters<'a> {
    /// How many images to generate. Must be number between 1 and 10
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create#images/create-n)
    pub fn n(mut self, input: &'a u8) -> Self {
        self.query.push(("n", json!(input)));
        self
    }

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create#images/create-size)
    pub fn size(mut self, input: &'a str) -> Self {
        self.query.push(("size", json!(input)));
        self
    }

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create#images/create-response_format)
    pub fn response_format(mut self, input: &'a str) -> Self {
        self.query.push(("size", json!(input)));
        self
    }

    /// A unique identifier representing your end-user, which can help
    /// OpenAI to monitor and detect abuse.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create#images/create-user)
    pub fn user(mut self, input: &'a str) -> Self {
        self.query.push(("user", json!(input)));
        self
    }

    /// Complete the request and send
    pub async fn done(self) -> Result<Images, StatusCode> {

        let mut map = HashMap::new();
        map.insert("prompt", json!(self.prompt));
        for (k, v) in self.query.into_iter() {
            map.insert(k, v);
        }

        let response: Result<Images, StatusCode> = requester::images(requester::ImageRequestType::Generations, map).await;

        match response {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }
}


impl<'a> EditParameters<'a> {

    ///An additional image whose fully transparent areas
    ///(e.g. where alpha is zero) indicate where image should
    ///be edited. Must be a valid PNG file, less than 4MB,
    ///and have the same dimensions as image.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-edit#images/create-edit-mask)
    pub fn mask(mut self, input: &'a str) -> Self {
        self.query.push(("mask", json!(input)));
        self
    }

    /// How many images to generate. Must be number between 1 and 10
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-edit#images/create-edit-n)
    pub fn n(mut self, input: &'a u8) -> Self {
        self.query.push(("n", json!(input)));
        self
    }

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-edit#images/create-edit-size)
    pub fn size(mut self, input: &'a str) -> Self {
        self.query.push(("size", json!(input)));
        self
    }

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-edit#images/create-edit-response_format)
    pub fn response_format(mut self, input: &'a str) -> Self {
        self.query.push(("size", json!(input)));
        self
    }

    /// A unique identifier representing your end-user, which can help
    /// OpenAI to monitor and detect abuse.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-edit#images/create-edit-user)
    pub fn user(mut self, input: &'a str) -> Self {
        self.query.push(("user", json!(input)));
        self
    }

    /// Complete the request and send
    pub async fn done(self) -> Result<Images, StatusCode> {

        let mut map = HashMap::new();
        map.insert("prompt", json!(self.prompt));
        map.insert("image", json!(self.image));
        for (k, v) in self.query.into_iter() {
            map.insert(k, v);
        }

        let response: Result<Images, StatusCode> = requester::images(requester::ImageRequestType::Edits, map).await;

        match response {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }
}

impl<'a> VariationParameters<'a> {
    /// How many images to generate. Must be number between 1 and 10
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-variation#images/create-variation-n)
    pub fn n(mut self, input: &'a u8) -> Self {
        self.query.push(("n", json!(input)));
        self
    }

    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-variation#images/create-variation-size)
    pub fn size(mut self, input: &'a str) -> Self {
        self.query.push(("size", json!(input)));
        self
    }

    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-variation#images/create-variation-response_format)
    pub fn response_format(mut self, input: &'a str) -> Self {
        self.query.push(("size", json!(input)));
        self
    }

    /// A unique identifier representing your end-user, which can help
    /// OpenAI to monitor and detect abuse.
    ///
    /// [OpenAI Reference](https://beta.openai.com/docs/api-reference/images/create-variation#images/create-variation-user)
    pub fn user(mut self, input: &'a str) -> Self {
        self.query.push(("user", json!(input)));
        self
    }

    /// Complete the request and send
    pub async fn done(self) -> Result<Images, StatusCode> {

        let mut map = HashMap::new();
        map.insert("image", json!(self.image));
        for (k, v) in self.query.into_iter() {
            map.insert(k, v);
        }

        let response: Result<Images, StatusCode> = requester::images(requester::ImageRequestType::Variations, map).await;

        match response {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }
}