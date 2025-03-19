use crate::ansi::AnsiElement;

/// Trait for formatting ANSI elements
pub trait FormatAnsi {
    fn format(&self, elements: &[AnsiElement]) -> String;
}

/// Formats ANSI elements for single-line output (useful for debugging)
pub struct SingleLineFormatter;

impl FormatAnsi for SingleLineFormatter {
    fn format(&self, elements: &[AnsiElement]) -> String {
        let mut result = String::new();
        
        for element in elements {
            let elem_type = element.element_type();
            let escape = element.escape_repr();
            let description = element.description();
            
            result.push_str(&format!("[{} '{}': {}] ", elem_type, escape, description));
        }
        
        result
    }
} 