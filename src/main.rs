use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

struct ASRPhonemeGenerator {
    word2phone: HashMap<String, Vec<String>>,
}

impl ASRPhonemeGenerator {
    fn new() -> io::Result<Self> {
        let mut generator = ASRPhonemeGenerator {
            word2phone: HashMap::new(),
        };

        let file = File::open("cmudict_SPHINX_40")?;
        let reader = BufReader::new(file);
        let re = Regex::new(r#"" "|\t"#).unwrap();

        for line in reader.lines() {
            let line = line?;
            let content: Vec<&str> = re.split(&line).collect();
            let word = content[0].to_lowercase();
            let phone = content[1..].join(" ");

            generator.word2phone.entry(word).or_insert_with(Vec::new).push(phone);
        }

        Ok(generator)
    }
}

async fn convert_word_to_phonemes(word: &str) -> Result<(String, bool), lambda_runtime::Error> {
    let generator = ASRPhonemeGenerator::new().map_err(lambda_runtime::Error::from)?;

    match generator.word2phone.get(word) {
        Some(phones) => Ok((phones[0].clone(), true)),  // Word found, return phonemes and true
        None => Ok((word.to_string(), false)),  // Word not found, return the word itself and false
    }
}

#[derive(Deserialize)]
struct Request {
    word: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    phonemes: String,
    message: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let word = event.payload.word;  // Extract the word from the request

    let (phonemes, word_found) = convert_word_to_phonemes(&word).await?;  // Get phonemes and word found flag

    // Set the message based on whether the word was found or not
    let message = if word_found {
        "data processed".to_string()
    } else {
        "unknown word".to_string()
    };

    let resp = Response {
        req_id: event.context.request_id,
        phonemes,
        message,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // Disable printing the name of the module in every log line.
        .with_target(false)
        // Disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
