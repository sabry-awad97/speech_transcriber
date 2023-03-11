use std::{error::Error, fs::File, io::BufReader};

pub struct AudioFileReader<'a> {
    file_path: &'a str,
}

impl<'a> AudioFileReader<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Self { file_path }
    }

    pub fn read_audio(&self) -> Result<(Vec<i16>, u32), Box<dyn Error>> {
        let audio_file = File::open(self.file_path)?;
        let audio_reader = BufReader::new(audio_file);
        let audio_decoder = hound::WavReader::new(audio_reader)?;
        let sample_rate = audio_decoder.spec().sample_rate;
        let audio_content: Vec<i16> = audio_decoder.into_samples().collect::<Result<_, _>>()?;
        Ok((audio_content, sample_rate))
    }
}
