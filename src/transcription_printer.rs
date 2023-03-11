use crate::model::{SpeechRecognitionAlternative, SpeechRecognitionResponse};
use colored::*;
use prettytable::{color, format, Attr, Cell, Row, Table};
use std::io::Write;

pub enum OutputFormat {
    Table,
    Csv,
    Json,
}

pub struct TranscriptionPrinter {
    table: Table,
    output_format: OutputFormat,
}

impl TranscriptionPrinter {
    pub fn new(output_format: OutputFormat) -> Self {
        let mut table = Table::new();
        table.set_titles(Self::create_title_row());

        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        Self {
            table,
            output_format,
        }
    }

    pub fn print(
        &mut self,
        recognition_response: &SpeechRecognitionResponse,
        writer: Option<&mut dyn Write>,
    ) {
        match self.output_format {
            OutputFormat::Table => self.print_table(recognition_response),
            OutputFormat::Csv => {
                match writer {
                    Some(mut w) => self.print_csv(recognition_response, &mut w),
                    None => self.print_csv(recognition_response, &mut std::io::stdout()),
                };
            }
            OutputFormat::Json => {
                match writer {
                    Some(mut w) => self.print_json(recognition_response, &mut w),
                    None => self.print_json(recognition_response, &mut std::io::stdout()),
                };
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

    fn print_table(&mut self, recognition_response: &SpeechRecognitionResponse) {
        for result in &recognition_response.results {
            for alternative in &result.alternatives {
                self.print_transcript(alternative);
                self.print_confidence_score(alternative);
                println!("\n{}", "Words:".bold().blue());
                for item in &alternative.words {
                    let start_time = Self::format_time(&item.start_time);
                    let end_time = Self::format_time(&item.end_time);
                    self.table.add_row(Row::new(vec![
                        Cell::new(&item.word),
                        Cell::new(&format!("{:.2}", item.confidence)),
                        Cell::new(&start_time),
                        Cell::new(&end_time),
                    ]));
                }
                self.table.printstd();
            }
        }
    }

    fn print_csv<W: Write>(
        &self,
        recognition_response: &SpeechRecognitionResponse,
        writer: &mut W,
    ) {
        writeln!(writer, "Word,Confidence Score,Start Time (s),End Time (s)").unwrap();
        for result in &recognition_response.results {
            for alternative in &result.alternatives {
                for item in &alternative.words {
                    let start_time = Self::format_time(&item.start_time);
                    let end_time = Self::format_time(&item.end_time);
                    writeln!(
                        writer,
                        "{},{:.2},{},{}",
                        item.word, item.confidence, start_time, end_time
                    )
                    .unwrap();
                }
            }
        }
    }

    fn print_json<W: Write>(
        &mut self,
        recognition_response: &SpeechRecognitionResponse,
        writer: &mut W,
    ) {
        let json_output = serde_json::to_string_pretty(&recognition_response).unwrap();
        writeln!(writer, "{}", json_output).unwrap();
    }

    fn print_transcript(&self, alternative: &SpeechRecognitionAlternative) {
        println!("\n{}", "Transcript:".bold().blue());
        println!("{}", alternative.transcript);
    }

    fn print_confidence_score(&self, alternative: &SpeechRecognitionAlternative) {
        println!("\n{}", "Confidence Score:".bold().blue());
        println!("{:.2}", alternative.confidence);
    }

    fn format_time(time_str: &str) -> String {
        let seconds = time_str.trim_end_matches('s').parse::<f64>().unwrap();
        format!("{:.2}", seconds)
    }
}
