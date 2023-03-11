use colored::*;
use prettytable::{color, format, Attr, Cell, Row, Table};

use crate::model::{SpeechRecognitionAlternative, SpeechRecognitionResponse};

pub struct TranscriptionPrinter {
    table: Table,
}

impl TranscriptionPrinter {
    pub fn new() -> Self {
        let mut table = Table::new();
        table.set_titles(Self::create_title_row());

        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        Self { table }
    }

    pub fn print(&mut self, recognition_response: &SpeechRecognitionResponse) {
        for result in &recognition_response.results {
            for alternative in &result.alternatives {
                self.print_transcript(alternative);
                self.print_confidence_score(alternative);
                self.print_words(alternative);
                self.print_table();
            }
        }
    }

    fn create_title_row() -> Row {
        Row::new(vec![
            Cell::new("Word")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new("Confidence Score")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new("Start Time (s)")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new("End Time (s)")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BLUE)),
        ])
    }

    fn print_transcript(&self, alternative: &SpeechRecognitionAlternative) {
        println!("\n{}", "Transcript:".bold().blue());
        println!("{}", alternative.transcript);
    }

    fn print_confidence_score(&self, alternative: &SpeechRecognitionAlternative) {
        println!("\n{}", "Confidence Score:".bold().blue());
        println!("{:.2}", alternative.confidence);
    }

    fn print_words(&mut self, alternative: &SpeechRecognitionAlternative) {
        println!("\n{}", "Words:".bold().blue());
        for item in &alternative.words {
            let start_time = Self::format_time(&item.start_time);
            let end_time = Self::format_time(&item.end_time);
            self.table.add_row(Row::new(vec![
                Cell::new(&item.word),
                Cell::new(&Self::format_confidence(item.confidence)),
                Cell::new(&start_time),
                Cell::new(&end_time),
            ]));
        }
    }

    fn format_time(time_str: &str) -> String {
        let seconds = time_str.trim_end_matches('s').parse::<f64>().unwrap();
        format!("{:.2}", seconds)
    }

    fn format_confidence(confidence: f64) -> String {
        format!("{:.2}", confidence).replace(".", ",")
    }

    fn print_table(&self) {
        self.table.printstd();
    }
}
