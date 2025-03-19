#[derive(Debug, Clone)]
pub enum ControlCharacter {
    Null,           // \0
    Bell,           // \a
    Backspace,      // \b
    Tab,            // \t
    LineFeed,       // \n
    VerticalTab,    // \v
    FormFeed,       // \f
    CarriageReturn, // \r
    Escape,         // \e or \x1b
    Delete,         // \x7F
    Other(u8),
}

impl ControlCharacter {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(ControlCharacter::Null),
            0x07 => Some(ControlCharacter::Bell),
            0x08 => Some(ControlCharacter::Backspace),
            0x09 => Some(ControlCharacter::Tab),
            0x0A => Some(ControlCharacter::LineFeed),
            0x0B => Some(ControlCharacter::VerticalTab),
            0x0C => Some(ControlCharacter::FormFeed),
            0x0D => Some(ControlCharacter::CarriageReturn),
            0x1B => Some(ControlCharacter::Escape),
            0x7F => Some(ControlCharacter::Delete),
            0x01..=0x06 | 0x0E..=0x1A | 0x1C..=0x1F => Some(ControlCharacter::Other(byte)),
            _ => None,
        }
    }

    pub fn escape_repr(&self) -> String {
        match self {
            ControlCharacter::Null => "\\0".to_string(),
            ControlCharacter::Bell => "\\a".to_string(),
            ControlCharacter::Backspace => "\\b".to_string(),
            ControlCharacter::Tab => "\\t".to_string(),
            ControlCharacter::LineFeed => "\\n".to_string(),
            ControlCharacter::VerticalTab => "\\v".to_string(),
            ControlCharacter::FormFeed => "\\f".to_string(),
            ControlCharacter::CarriageReturn => "\\r".to_string(),
            ControlCharacter::Escape => "\\x1b".to_string(),
            ControlCharacter::Delete => "\\x7F".to_string(),
            ControlCharacter::Other(byte) => format!("\\x{:02X}", byte),
        }
    }

    pub fn description(&self) -> String {
        match self {
            ControlCharacter::Null => "Null character".to_string(),
            ControlCharacter::Bell => "Bell (alert)".to_string(),
            ControlCharacter::Backspace => "Backspace".to_string(),
            ControlCharacter::Tab => "Horizontal tab".to_string(),
            ControlCharacter::LineFeed => "Line feed (new line)".to_string(),
            ControlCharacter::VerticalTab => "Vertical tab".to_string(),
            ControlCharacter::FormFeed => "Form feed (new page)".to_string(),
            ControlCharacter::CarriageReturn => "Carriage return".to_string(),
            ControlCharacter::Escape => "Escape character".to_string(),
            ControlCharacter::Delete => "Delete character".to_string(),
            ControlCharacter::Other(byte) => format!("Control character: 0x{:02X}", byte),
        }
    }
} 