use std::error::Error;

use bytemuck::cast_slice;
use model::SpeechRecognitionResponse;
use speech_client::SpeechClient;
use transcription_printer::{OutputFormat, TranscriptionPrinter};

use crate::audio_file_reader::AudioStream;

mod audio_file_reader;
mod model;
mod speech_client;
mod transcription_printer;

struct SpeechTranscriber {
    speech_client: SpeechClient,
}

impl SpeechTranscriber {
    pub fn new(language: &str) -> Self {
        let speech_client = SpeechClient::new(language.to_owned());
        Self { speech_client }
    }

    pub async fn transcribe(
        self,
        audio_content: &[i16],
        sample_rate: &u32,
    ) -> Result<SpeechRecognitionResponse, Box<dyn Error>> {
        let audio_slice = cast_slice(&audio_content);
        let recognition_response = self
            .speech_client
            .recognize(audio_slice, *sample_rate)
            .await?;

        Ok(recognition_response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language_code = "en-US";
    let audio_file_path = "sample.wav";
    let buffer_duaration = 2; // 2 seconds
    let audio_stream = AudioStream::new(audio_file_path, buffer_duaration)?;
    let sample_rate = audio_stream.sample_rate;
    let audio_content = audio_stream.audio_content();
    let speech_transcriber = SpeechTranscriber::new(language_code);
    let recognition_response = speech_transcriber
        .transcribe(&audio_content, &sample_rate)
        .await?;

    let mut printer = TranscriptionPrinter::new(OutputFormat::Table);
    printer.print(&recognition_response, None);

    Ok(())
}
