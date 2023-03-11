use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::result::Result;

use hound;

/// An iterator that yields audio data from a WAV file in chunks of a specified size.
pub struct AudioStream {
    pub sample_rate: u32,
    pub duration: u32,
    pub num_channels: usize,
    pub buffer_size: usize,
    pub buffer: Vec<i16>,
    pub audio_decoder: hound::WavReader<BufReader<File>>,
}

impl AudioStream {
    /// Creates a new `AudioStream` for the given file path and buffer size.
    pub fn new(file_path: &str, buffer_size: usize) -> Result<Self, Box<dyn Error>> {
        let audio_file = File::open(file_path)?;
        let audio_reader = BufReader::new(audio_file);
        let audio_decoder = hound::WavReader::new(audio_reader)?;
        let sample_rate = audio_decoder.spec().sample_rate as u32;
        let num_channels = audio_decoder.spec().channels as usize;
        let buffer_size_bytes = buffer_size * num_channels * 2;

        Ok(Self {
            sample_rate,
            duration: audio_decoder.duration() / sample_rate,
            num_channels,
            buffer_size,
            buffer: vec![0; buffer_size_bytes],
            audio_decoder,
        })
    }

    pub fn audio_content(self) -> Vec<i16> {
        self.into_iter().collect::<Vec<Vec<i16>>>().concat()
    }
}

impl Iterator for AudioStream {
    type Item = Vec<i16>;

    fn next(&mut self) -> Option<Self::Item> {
        let num_samples = self.buffer_size * self.num_channels;
        let mut samples = self.audio_decoder.samples::<i16>();

        let mut i = 0;
        while let Some(sample) = samples.next() {
            self.buffer[i] = sample.unwrap();
            i += 1;
            if i == num_samples {
                break;
            }
        }

        if i == 0 {
            return None;
        }

        let num_samples_read = i;
        let num_samples_needed = num_samples - num_samples_read;

        if num_samples_needed > 0 {
            self.buffer[num_samples_read..num_samples].fill(0);
        }

        let audio_samples = self.buffer[..num_samples_read].to_vec();
        Some(audio_samples)
    }
}
