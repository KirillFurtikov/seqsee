use crate::ansi::{AnsiElement, csi, ctrl};
use std::io::{self, Read};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Invalid sequence: {0}")]
    InvalidSequence(String),
}

pub struct AnsiParser;

impl AnsiParser {
    /// Parse ANSI sequences from input
    pub fn parse<R: Read>(mut input: R) -> Result<Vec<AnsiElement>, ParserError> {
        let mut elements = Vec::new();
        let mut buf = Vec::new();
        
        // Read all input into buffer
        input.read_to_end(&mut buf)?;
        
        // Process buffer into elements - first handle literal "\e" sequences
        let expanded_buf = Self::expand_literal_escapes(&buf);
        
        // Process buffer into elements
        let mut i = 0;
        let mut text_buf = String::new();
        
        while i < expanded_buf.len() {
            if expanded_buf[i] == 0x1B {  // ESC character
                // First, add accumulated text if any
                if !text_buf.is_empty() {
                    elements.push(AnsiElement::Text(std::mem::take(&mut text_buf)));
                }
                
                // Process escape sequence
                if i + 1 >= expanded_buf.len() {
                    // Just an ESC at the end
                    elements.push(AnsiElement::Ctrl(ctrl::ControlCharacter::Escape));
                    i += 1;
                    continue;
                }
                
                match expanded_buf[i + 1] {
                    b'[' => { // CSI sequence
                        let (elem, consumed) = Self::parse_csi(&expanded_buf[i..]);
                        elements.push(elem);
                        i += consumed;
                    }
                    
                    // Simple escape sequences now handled as CSI
                    b'7' => {
                        elements.push(AnsiElement::Csi(csi::CSI::CursorSavePosition));
                        i += 2;
                    }
                    b'8' => {
                        elements.push(AnsiElement::Csi(csi::CSI::CursorRestorePosition));
                        i += 2;
                    }
                    b'=' => {
                        elements.push(AnsiElement::Csi(csi::CSI::ApplicationKeypadMode));
                        i += 2;
                    }
                    b'>' => {
                        elements.push(AnsiElement::Csi(csi::CSI::NumericKeypadMode));
                        i += 2;
                    }
                    
                    // G0 character set
                    b'(' => {
                        if i + 2 < expanded_buf.len() {
                            match expanded_buf[i + 2] {
                                b'0' => {
                                    elements.push(AnsiElement::Csi(csi::CSI::SetG0SpecialChars));
                                    i += 3;
                                }
                                b'B' => {
                                    elements.push(AnsiElement::Csi(csi::CSI::SetG0NormalChars));
                                    i += 3;
                                }
                                _ => {
                                    // Unknown G0 sequence
                                    let seq = format!("({}", expanded_buf[i + 2] as char);
                                    elements.push(AnsiElement::Csi(csi::CSI::Unknown(seq)));
                                    i += 3;
                                }
                            }
                        } else {
                            // Incomplete sequence
                            let seq = if i + 2 == expanded_buf.len() {
                                format!("({})", expanded_buf[i + 2] as char)
                            } else {
                                "(".to_string()
                            };
                            elements.push(AnsiElement::Csi(csi::CSI::Unknown(seq)));
                            i = expanded_buf.len(); // End processing
                        }
                    }
                    
                    // Unrecognized escape sequence - treat as unknown CSI
                    _ => {
                        if expanded_buf[i + 1] < 128 {
                            let seq = format!("{}", expanded_buf[i + 1] as char);
                            elements.push(AnsiElement::Csi(csi::CSI::Unknown(seq)));
                        } else {
                            let seq = format!("0x{:02X}", expanded_buf[i + 1]);
                            elements.push(AnsiElement::Csi(csi::CSI::Unknown(seq)));
                        }
                        i += 2;
                    }
                }
            } else if let Some(ctrl_char) = ctrl::ControlCharacter::from_byte(expanded_buf[i]) {
                // Control character
                if !text_buf.is_empty() {
                    elements.push(AnsiElement::Text(std::mem::take(&mut text_buf)));
                }
                elements.push(AnsiElement::Ctrl(ctrl_char));
                i += 1;
            } else {
                // Regular text character
                text_buf.push(expanded_buf[i] as char);
                i += 1;
            }
        }
        
        // Add any remaining text
        if !text_buf.is_empty() {
            elements.push(AnsiElement::Text(text_buf));
        }
        
        Ok(elements)
    }
    
    // Helper function to expand literal escape sequences
    fn expand_literal_escapes(buf: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(buf.len());
        let mut i = 0;
        
        while i < buf.len() {
            // Look for \e escape sequence
            if i + 1 < buf.len() && buf[i] == b'\\' && (buf[i + 1] == b'e' || buf[i + 1] == b'E') {
                // Replace \e with ESC (0x1B)
                result.push(0x1B);
                i += 2;
            } else {
                // Pass through other characters
                result.push(buf[i]);
                i += 1;
            }
        }
        
        result
    }
    
    // Parse a CSI sequence, return the element and number of bytes consumed
    fn parse_csi(buf: &[u8]) -> (AnsiElement, usize) {
        let mut i = 2; // Skip ESC [
        let mut params_u8 = Vec::new();
        let mut params_u16 = Vec::new();
        let mut current_param = String::new();
        let mut is_private = false;
        
        // Check for private sequences (like ESC [ ? ...)
        if i < buf.len() && buf[i] == b'?' {
            is_private = true;
            i += 1;
        }
        
        // Parse parameters
        while i < buf.len() && ((buf[i] as char).is_ascii_digit() || buf[i] == b';') {
            if buf[i] == b';' {
                if !current_param.is_empty() {
                    let param_value = current_param.parse::<u16>().unwrap_or(0);
                    params_u16.push(param_value);
                    if param_value <= 255 {
                        params_u8.push(param_value as u8);
                    } else {
                        params_u8.push(0); // Fallback for out-of-range values
                    }
                } else {
                    params_u16.push(0);
                    params_u8.push(0); // Empty parameter defaults to 0
                }
                current_param.clear();
            } else {
                current_param.push(buf[i] as char);
            }
            i += 1;
        }
        
        // Add the last parameter if any
        if !current_param.is_empty() {
            let param_value = current_param.parse::<u16>().unwrap_or(0);
            params_u16.push(param_value);
            if param_value <= 255 {
                params_u8.push(param_value as u8);
            } else {
                params_u8.push(0); // Fallback for out-of-range values
            }
        }
        
        // Process by command byte
        if i < buf.len() {
            let cmd_byte = buf[i] as char;
            let element = match cmd_byte {
                'A' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::CursorUp(n))
                }
                'B' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::CursorDown(n))
                }
                'C' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::CursorForward(n))
                }
                'D' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::CursorBackward(n))
                }
                'E' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::CursorNextLine(n))
                }
                'F' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::CursorPreviousLine(n))
                }
                'G' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::CursorToColumn(n))
                }
                'H' | 'f' => {
                    let row = params_u16.first().copied().unwrap_or(1) as u32;
                    let col = params_u16.get(1).copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::CursorPosition(row, col))
                }
                'J' => {
                    let n = params_u8.first().copied().unwrap_or(0);
                    AnsiElement::Csi(csi::CSI::EraseInDisplay(n))
                }
                'K' => {
                    let n = params_u8.first().copied().unwrap_or(0);
                    AnsiElement::Csi(csi::CSI::EraseInLine(n))
                }
                'S' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::ScrollUp(n))
                }
                'T' => {
                    let n = params_u16.first().copied().unwrap_or(1) as u32;
                    AnsiElement::Csi(csi::CSI::ScrollDown(n))
                }
                's' => AnsiElement::Csi(csi::CSI::CursorSavePosition),
                'u' => AnsiElement::Csi(csi::CSI::CursorRestorePosition),
                'h' => AnsiElement::Csi(csi::CSI::SetMode(params_u16)),
                'l' => AnsiElement::Csi(csi::CSI::ResetMode(params_u16)),
                't' => AnsiElement::Csi(csi::CSI::WindowManipulation(params_u16)),
                'n' => {
                    if is_private && !params_u8.is_empty() && params_u8[0] == 6 {
                        AnsiElement::Csi(csi::CSI::CursorPositionReport)
                    } else if !params_u8.is_empty() && params_u8[0] == 6 {
                        AnsiElement::Csi(csi::CSI::DeviceStatusReport)
                    } else {
                        // Unknown report request
                        let mut seq = String::new();
                        if is_private {
                            seq.push('?');
                        }
                        for p in &params_u16 {
                            seq.push_str(&p.to_string());
                            seq.push(';');
                        }
                        if !seq.is_empty() && seq.ends_with(';') {
                            seq.pop(); // Remove trailing semicolon
                        }
                        seq.push('n');
                        AnsiElement::Csi(csi::CSI::Unknown(seq))
                    }
                }
                'm' => {
                    if params_u8.is_empty() {
                        AnsiElement::Csi(csi::CSI::ResetAttributes)
                    } else if params_u8.len() >= 3 && params_u8[0] == 38 && params_u8[1] == 2 {
                        // 24-bit RGB color (38;2;r;g;b)
                        let r = params_u8.get(2).copied().unwrap_or(0);
                        let g = params_u8.get(3).copied().unwrap_or(0);
                        let b = params_u8.get(4).copied().unwrap_or(0);
                        AnsiElement::Csi(csi::CSI::SetForegroundColor(r, g, b))
                    } else if params_u8.len() >= 3 && params_u8[0] == 48 && params_u8[1] == 2 {
                        // 24-bit RGB color (48;2;r;g;b)
                        let r = params_u8.get(2).copied().unwrap_or(0);
                        let g = params_u8.get(3).copied().unwrap_or(0);
                        let b = params_u8.get(4).copied().unwrap_or(0);
                        AnsiElement::Csi(csi::CSI::SetBackgroundColor(r, g, b))
                    } else if params_u8.len() >= 3 && params_u8[0] == 38 && params_u8[1] == 5 {
                        // 8-bit/256 color (38;5;n)
                        let color_index = params_u8.get(2).copied().unwrap_or(0);
                        // Convert to approximate RGB for display
                        // This is a simplification - in a real terminal, there would be a color table
                        AnsiElement::Csi(csi::CSI::SetForegroundColor256(color_index))
                    } else if params_u8.len() >= 3 && params_u8[0] == 48 && params_u8[1] == 5 {
                        // 8-bit/256 color (48;5;n)
                        let color_index = params_u8.get(2).copied().unwrap_or(0);
                        AnsiElement::Csi(csi::CSI::SetBackgroundColor256(color_index))
                    } else {
                        AnsiElement::Csi(csi::CSI::SetGraphicsMode(params_u8))
                    }
                }
                _ => {
                    // Unknown CSI sequence
                    let mut seq = String::new();
                    if is_private {
                        seq.push('?');
                    }
                    for p in &params_u16 {
                        seq.push_str(&p.to_string());
                        seq.push(';');
                    }
                    if !seq.is_empty() && seq.ends_with(';') {
                        seq.pop(); // Remove trailing semicolon
                    }
                    seq.push(cmd_byte);
                    AnsiElement::Csi(csi::CSI::Unknown(seq))
                }
            };
            
            (element, i + 1) // Include the command byte
        } else {
            // Incomplete CSI sequence
            let mut seq = String::new();
            if is_private {
                seq.push('?');
            }
            for p in &params_u16 {
                seq.push_str(&p.to_string());
                seq.push(';');
            }
            if !seq.is_empty() && seq.ends_with(';') {
                seq.pop(); // Remove trailing semicolon
            }
            
            (AnsiElement::Csi(csi::CSI::Unknown(seq)), i)
        }
    }
} 