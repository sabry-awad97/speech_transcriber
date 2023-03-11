use crate::model;
use base64::{engine::general_purpose, Engine};
use reqwest::{Client, Error as ReqwestError};

// Speech-to-Text API client
pub struct SpeechClient {
    api_key: String,
    language: String,
    client: Client,
}

impl SpeechClient {
    pub fn new(language: String) -> SpeechClient {
        let api_key = "AIzaSyBOti4mM-6x9WDnZIjIeyEU21OpBXqWBgw".to_owned();
        SpeechClient {
            api_key,
            language,
            client: Client::new(),
        }
    }

    pub async fn recognize(
        &self,
        audio_content: &[u8],
        sample_rate: u32,
    ) -> Result<model::SpeechRecognitionResponse, ReqwestError> {
        let encoded_audio_content = general_purpose::STANDARD.encode(&audio_content);

        let recognition_config = model::RecognitionConfig {
            encoding: "LINEAR16".to_string(),
            sample_rate_hertz: sample_rate as i32,
            language_code: self.language.clone(),
            enable_word_time_offsets: true,
            enable_word_confidence: true,
            use_enhanced: true,
        };

        let recognition_audio = model::RecognitionAudio {
            content: encoded_audio_content,
        };

        let recognize_request = model::RecognizeRequest {
            config: recognition_config,
            audio: recognition_audio,
        };

        let url = format!(
            "https://speech.googleapis.com/v1/speech:recognize?key={}",
            self.api_key
        );

        let response = self
            .client
            .post(&url)
            .json(&recognize_request)
            .send()
            .await?;

        response.json::<model::SpeechRecognitionResponse>().await
    }
}
