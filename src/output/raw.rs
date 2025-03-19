use termio::{Color, StyledText};
use crate::ansi::AnsiElement;
use crate::formatter::FormatAnsi;

pub struct RawFormatter {
    pub colorize: bool,
}

impl RawFormatter {
    pub fn new(colorize: bool) -> Self {
        Self { colorize }
    }
}

impl Default for RawFormatter {
    fn default() -> Self {
        Self { colorize: true }
    }
}

impl FormatAnsi for RawFormatter {
    fn format(&self, elements: &[AnsiElement]) -> String {
        let mut result = String::new();
        
        for element in elements {
            match element {
                AnsiElement::Text(text) => {
                    result.push_str(text);
                },
                _ => {
                    // Highlight sequences in the original text
                    if self.colorize {
                        match element.element_type() {
                            "CSI" => result.push_str(&element.escape_repr().color(Color::Blue).to_string()),
                            "Ctrl" => result.push_str(&element.escape_repr().color(Color::Yellow).to_string()),
                            _ => result.push_str(&element.escape_repr().color(Color::IntenseMagenta).to_string()),
                        }
                    } else {
                        result.push_str(&element.escape_repr());
                    }
                }
            }
        }
        
        result
    }
} 