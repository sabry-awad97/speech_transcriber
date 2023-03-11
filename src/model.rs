use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RecognitionConfig {
    pub encoding: String,
    pub sample_rate_hertz: i32,
    pub language_code: String,
    pub enable_word_time_offsets: bool,
    pub enable_word_confidence: bool,
    pub use_enhanced: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    pub confidence: f64,
    pub end_time: String,
    pub start_time: String,
    pub word: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeechRecognitionAlternative {
    pub confidence: f64,
    pub transcript: String,
    pub words: Vec<Word>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechRecognitionResult {
    pub alternatives: Vec<SpeechRecognitionAlternative>,
    pub _language_code: String,
    pub _result_end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechRecognitionResponse {
    pub _request_id: String,
    pub results: Vec<SpeechRecognitionResult>,
    pub _total_billed_time: String,
}

#[derive(Serialize)]
pub struct RecognitionAudio {
    pub content: String,
}

#[derive(Serialize)]
pub struct RecognizeRequest {
    pub config: RecognitionConfig,
    pub audio: RecognitionAudio,
}
