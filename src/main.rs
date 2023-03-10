use base64::{engine::general_purpose, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize)]
struct RecognitionConfig {
    encoding: String,
    language_code: String,
}

#[derive(Serialize)]
struct RecognitionAudio {
    content: String,
}

#[derive(Serialize)]
struct RecognizeRequest {
    config: RecognitionConfig,
    audio: RecognitionAudio,
}

#[derive(Deserialize)]
struct SpeechRecognitionAlternative {
    transcript: String,
}

#[derive(Deserialize)]
struct SpeechRecognitionResult {
    alternatives: Vec<SpeechRecognitionAlternative>,
}

#[derive(Deserialize)]
struct SpeechRecognitionResponse {
    results: Vec<SpeechRecognitionResult>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = "AIzaSyBOti4mM-6x9WDnZIjIeyEU21OpBXqWBgw";
    let language = "en-US";
    let audio_file_path = "sample.wav";

    // Read audio file content
    let mut audio_file = File::open(audio_file_path)?;
    let mut audio_content = Vec::new();
    audio_file.read_to_end(&mut audio_content)?;

    // Encode audio content to base64
    let encoded_audio_content = general_purpose::STANDARD.encode(&audio_content);

    // Build recognition request
    let recognition_config = RecognitionConfig {
        encoding: "LINEAR16".to_string(),
        language_code: language.to_string(),
    };

    let recognition_audio = RecognitionAudio {
        content: encoded_audio_content,
    };

    let recognize_request = RecognizeRequest {
        config: recognition_config,
        audio: recognition_audio,
    };

    // Send recognition request
    let client = Client::new();
    let url = format!(
        "https://speech.googleapis.com/v1/speech:recognize?key={}",
        key
    );
    let response = client.post(&url).json(&recognize_request).send().await?;

    // Parse recognition response
    let recognition_response: SpeechRecognitionResponse = response.json().await?;
    let first_result = recognition_response.results.first().unwrap();
    let first_alternative = first_result.alternatives.first().unwrap();

    println!("{}", first_alternative.transcript);

    Ok(())
}
