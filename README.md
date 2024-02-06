# AWS Lambda Function for Phoneme Conversion

This AWS Lambda function takes a word as input and returns its phoneme representation using the CMU Sphinx's phonetic dictionary. If the input word is found in the dictionary, the function returns its phonemes; otherwise, it indicates that the word is unknown.

## Prerequisites

- AWS CLI configured with appropriate permissions.
- [Rust](https://www.rust-lang.org/tools/install) installed on your system.
- [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda#installation) for building and deploying Lambda functions in Rust.

## Functionality

- **Input**: A JSON object with a single key `word`, where the value is the word you want to convert to phonemes.
- **Output**: A JSON object containing:
  - `req_id`: The request ID for the invocation.
  - `phonemes`: The phonetic representation of the input word if found in the dictionary.
  - `message`: Indicates "Data processed" if the word was found, or "unknown word" otherwise.

## Build & Deploy

1. Install [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda#installation)
2. Build the function with `cargo lambda build --release`
3. Deploy the function to AWS Lambda with `cargo lambda deploy --iam-role YOUR_ROLE`

## Build for ARM 64

Build the function with `cargo lambda build --release --arm64`