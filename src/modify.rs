use base64::{engine::general_purpose, Engine};
use bytemuck::cast_slice;
use hound::WavReader;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
struct RecognitionConfig {
    encoding: String,
    sample_rate_hertz: i32,
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

#[derive(Debug, Deserialize)]
struct SpeechRecognitionAlternative {
    transcript: String,
}

#[derive(Debug, Deserialize)]
struct SpeechRecognitionResult {
    alternatives: Vec<SpeechRecognitionAlternative>,
}

#[derive(Debug, Deserialize)]
struct SpeechRecognitionResponse {
    results: Option<Vec<SpeechRecognitionResult>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = "AIzaSyBOti4mM-6x9WDnZIjIeyEU21OpBXqWBgw";
    let language = "en-US";
    let audio_file_path = "sample.wav";
    let chunk_duration = 5; // seconds

    let mut audio_file = WavReader::open(audio_file_path)?;
    let sample_rate = audio_file.spec().sample_rate;
    let mut audio_content = Vec::new();
    for sample in audio_file.samples::<i16>() {
        audio_content.push(sample?);
    }
    let audio_slice = cast_slice(&audio_content);
    let recognition_config = RecognitionConfig {
        encoding: "LINEAR16".to_string(),
        sample_rate_hertz: sample_rate as i32,
        language_code: language.to_string(),
    };
    let client = Client::new();
    let url = format!(
        "https://speech.googleapis.com/v1/speech:recognize?key={}",
        key
    );
    let chunk_size = (sample_rate * chunk_duration).try_into()?;
    let chunks = audio_slice.chunks(chunk_size);
    for chunk in chunks {
        let encoded_chunk = general_purpose::STANDARD.encode(chunk);
        let recognition_audio = RecognitionAudio {
            content: encoded_chunk,
        };
        let recognize_request = RecognizeRequest {
            config: recognition_config.clone(),
            audio: recognition_audio,
        };
        let response = client.post(&url).json(&recognize_request).send().await?;
        let recognition_response: SpeechRecognitionResponse = response.json().await?;
        let results = recognition_response.results;
        if let Some(results) = results {
            let first_result = results.first().unwrap();
            let first_alternative = first_result.alternatives.first().unwrap();
            println!("{}", first_alternative.transcript);
        }
    }
    Ok(())
}
