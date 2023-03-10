use std::{fs::File, io::BufReader};

use base64::{engine::general_purpose, Engine};
use bytemuck::cast_slice;
use prettytable::{format, Cell, Row, Table};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RecognitionConfig {
    encoding: String,
    sample_rate_hertz: i32,
    language_code: String,
    enable_word_time_offsets: bool,
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
#[serde(rename_all = "camelCase")]
pub struct Word {
    end_time: String,
    start_time: String,
    word: String,
}

#[derive(Deserialize)]
struct SpeechRecognitionAlternative {
    confidence: f64,
    transcript: String,
    words: Vec<Word>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpeechRecognitionResult {
    alternatives: Vec<SpeechRecognitionAlternative>,
    language_code: String,
    result_end_time: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpeechRecognitionResponse {
    request_id: String,
    results: Vec<SpeechRecognitionResult>,
    pub total_billed_time: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = "AIzaSyBOti4mM-6x9WDnZIjIeyEU21OpBXqWBgw";
    let language = "en-US";
    let audio_file_path = "sample.wav";

    // Read audio file content
    let audio_file = File::open(audio_file_path)?;
    let audio_reader = BufReader::new(audio_file);
    let audio_decoder = hound::WavReader::new(audio_reader)?;
    let sample_rate = audio_decoder.spec().sample_rate;
    let audio_content: Vec<i16> = audio_decoder.into_samples().collect::<Result<_, _>>()?;

    let audio_slice = cast_slice(&audio_content);

    // Encode audio content to base64
    let encoded_audio_content = general_purpose::STANDARD.encode(&audio_slice);

    // Build recognition request
    let recognition_config = RecognitionConfig {
        encoding: "LINEAR16".to_string(),
        sample_rate_hertz: sample_rate as i32,
        language_code: language.to_string(),
        enable_word_time_offsets: true,
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

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Word"),
        Cell::new("Start Time (s)"),
        Cell::new("End Time (s)"),
    ]));

    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    // Extract the transcription from the response and print it to the console
    for result in recognition_response.results {
        for alternative in result.alternatives {
            println!("Transcript: {}", alternative.transcript);
            for item in alternative.words {
                let seconds = item.start_time.trim_end_matches('s').parse::<f64>()?;
                let start_time = format!("{:.3}", seconds);
                let seconds = item.end_time.trim_end_matches('s').parse::<f64>()?;
                let end_time = format!("{:.3}", seconds);
                table.add_row(Row::new(vec![
                    Cell::new(&item.word),
                    Cell::new(&start_time),
                    Cell::new(&end_time),
                ]));
            }
        }
    }
    table.printstd();

    Ok(())
}
