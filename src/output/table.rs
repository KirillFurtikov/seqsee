use tabled::settings::{Style, Alignment};
use tabled::{Table, Tabled};
use termio::{Color, Decoration, StyledText};
use crate::ansi::AnsiElement;
use crate::formatter::FormatAnsi;

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "Type")]
    element_type: String,
    
    #[tabled(rename = "Esc")]
    escape: String,
    
    #[tabled(rename = "Desc")]
    description: String,
}

pub struct TableFormatter {
    pub colorize: bool,
}

impl TableFormatter {
    pub fn new(colorize: bool) -> Self {
        Self { colorize }
    }
}

impl Default for TableFormatter {
    fn default() -> Self {
        Self { colorize: true }
    }
}

impl FormatAnsi for TableFormatter {
    fn format(&self, elements: &[AnsiElement]) -> String {
        if elements.is_empty() {
            return String::new();
        }

        let rows: Vec<Row> = elements.iter().map(|element| {
            let element_type = if self.colorize {
                match element.element_type() {
                    "Text" => element.element_type().color(Color::IntenseBlack).to_string(),
                    "CSI" => element.element_type().color(Color::Blue).decoration(Decoration::Bold).to_string(),
                    "Ctrl" => element.element_type().color(Color::Yellow).decoration(Decoration::Bold).to_string(),
                    _ => element.element_type().to_string(),
                }
            } else {
                element.element_type().to_string()
            };
            
            let escape = if self.colorize {
                element.escape_repr().color(Color::IntenseMagenta).to_string()
            } else {
                element.escape_repr()
            };
            
            Row {
                element_type,
                escape,
                description: element.description(),
            }
        }).collect();
        
        let mut table = Table::new(rows);
        table.with(Style::empty())
             .with(Alignment::left());

        table.to_string()
    }
} 