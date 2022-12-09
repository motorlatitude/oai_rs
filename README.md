[![OAI_RS](https://github.com/motorlatitude/oai_rs/main/.github/OAI_RS_Header.png?raw=true)]()

An async rust library that's a wrapper for the OpenAI API.

# 💫 Features
Handles some of the following endpoints of the API:

    - [x] Models
    - [x] Edits
    - [x] Images
    - [ ] Embeddings
    - [ ] Files
    - [ ] Fine-tunes
    - [ ] Moderations

# 📔 Examples

```rust
use oai_rs::{completions, models};

async {
     let completions = completions::build(models::CompletionModels::TEXT_DAVINCI_003)
         .prompt("Ice cream or cookies?")
         .max_tokens(32)
         .complete()
         .await;

        println!("{:?}", completions);
};
```
