#[derive(Debug, Clone)]
pub enum CSI {
    // Cursor movement
    CursorUp(u32),
    CursorDown(u32),
    CursorForward(u32),
    CursorBackward(u32),
    CursorPosition(u32, u32),
    CursorSavePosition,              // ESC 7 or ESC [ s
    CursorRestorePosition,           // ESC 8 or ESC [ u
    CursorToColumn(u32),             // ESC [ G
    CursorNextLine(u32),             // ESC [ E
    CursorPreviousLine(u32),         // ESC [ F
    
    // Erase functions
    EraseInDisplay(u8),
    EraseInLine(u8),
    
    // Graphics and attributes
    SetGraphicsMode(Vec<u8>),
    SetForegroundColor(u8, u8, u8),  // RGB
    SetBackgroundColor(u8, u8, u8),  // RGB
    SetForegroundColor256(u8),       // 8-bit/256 color
    SetBackgroundColor256(u8),       // 8-bit/256 color
    ResetAttributes,
    
    // Screen modes
    SetMode(Vec<u16>),               // ESC [ h
    ResetMode(Vec<u16>),             // ESC [ l
    
    // Terminal modes
    ApplicationKeypadMode,           // Was ESC =
    NumericKeypadMode,               // Was ESC >
    
    // Character sets
    SetG0SpecialChars,               // Was ESC ( 0
    SetG0NormalChars,                // Was ESC ( B
    
    // Scrolling
    ScrollUp(u32),                   // ESC [ S
    ScrollDown(u32),                 // ESC [ T
    
    // Window manipulation
    WindowManipulation(Vec<u16>),    // ESC [ t
    
    // Device status
    DeviceStatusReport,              // ESC [ 6 n
    CursorPositionReport,            // ESC [ ? 6 n
    
    // Other CSI commands
    Unknown(String),
}

impl CSI {
    pub fn escape_repr(&self) -> String {
        match self {
            CSI::CursorUp(n) => format!("\\x1b[{}A", n),
            CSI::CursorDown(n) => format!("\\x1b[{}B", n),
            CSI::CursorForward(n) => format!("\\x1b[{}C", n),
            CSI::CursorBackward(n) => format!("\\x1b[{}D", n),
            CSI::CursorPosition(row, col) => format!("\\x1b[{};{}H", row, col),
            CSI::CursorSavePosition => "\\x1b[s".to_string(),
            CSI::CursorRestorePosition => "\\x1b[u".to_string(),
            CSI::CursorToColumn(n) => format!("\\x1b[{}G", n),
            CSI::CursorNextLine(n) => format!("\\x1b[{}E", n),
            CSI::CursorPreviousLine(n) => format!("\\x1b[{}F", n),
            CSI::EraseInDisplay(n) => format!("\\x1b[{}J", n),
            CSI::EraseInLine(n) => format!("\\x1b[{}K", n),
            CSI::SetGraphicsMode(params) => {
                let params_str = params.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(";");
                format!("\\x1b[{}m", params_str)
            },
            CSI::SetForegroundColor(r, g, b) => format!("\\x1b[38;2;{};{};{}m", r, g, b),
            CSI::SetBackgroundColor(r, g, b) => format!("\\x1b[48;2;{};{};{}m", r, g, b),
            CSI::SetForegroundColor256(n) => format!("\\x1b[38;5;{}m", n),
            CSI::SetBackgroundColor256(n) => format!("\\x1b[48;5;{}m", n),
            CSI::ResetAttributes => "\\x1b[0m".to_string(),
            CSI::SetMode(params) => {
                let params_str = params.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(";");
                format!("\\x1b[{}h", params_str)
            },
            CSI::ResetMode(params) => {
                let params_str = params.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(";");
                format!("\\x1b[{}l", params_str)
            },
            CSI::ApplicationKeypadMode => "\\x1b=".to_string(),
            CSI::NumericKeypadMode => "\\x1b>".to_string(),
            CSI::SetG0SpecialChars => "\\x1b(0".to_string(),
            CSI::SetG0NormalChars => "\\x1b(B".to_string(),
            CSI::ScrollUp(n) => format!("\\x1b[{}S", n),
            CSI::ScrollDown(n) => format!("\\x1b[{}T", n),
            CSI::WindowManipulation(params) => {
                let params_str = params.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(";");
                format!("\\x1b[{}t", params_str)
            },
            CSI::DeviceStatusReport => "\\x1b[6n".to_string(),
            CSI::CursorPositionReport => "\\x1b[?6n".to_string(),
            CSI::Unknown(seq) => format!("\\x1b[{}", seq),
        }
    }
    
    pub fn description(&self) -> String {
        match self {
            CSI::CursorUp(n) => format!("Move cursor up {} lines", n),
            CSI::CursorDown(n) => format!("Move cursor down {} lines", n),
            CSI::CursorForward(n) => format!("Move cursor forward {} columns", n),
            CSI::CursorBackward(n) => format!("Move cursor backward {} columns", n),
            CSI::CursorPosition(row, col) => format!("Move cursor to position ({}, {})", row, col),
            CSI::CursorSavePosition => "Save cursor position".to_string(),
            CSI::CursorRestorePosition => "Restore cursor position".to_string(),
            CSI::CursorToColumn(n) => format!("Move cursor to column {}", n),
            CSI::CursorNextLine(n) => format!("Move cursor to beginning of line {} lines down", n),
            CSI::CursorPreviousLine(n) => format!("Move cursor to beginning of line {} lines up", n),
            CSI::EraseInDisplay(n) => {
                match n {
                    0 => "Erase from cursor to end of screen".to_string(),
                    1 => "Erase from cursor to beginning of screen".to_string(),
                    2 => "Erase entire screen".to_string(),
                    3 => "Erase saved lines".to_string(),
                    _ => format!("Unknown erase display mode: {}", n),
                }
            },
            CSI::EraseInLine(n) => {
                match n {
                    0 => "Erase from cursor to end of line".to_string(),
                    1 => "Erase from cursor to beginning of line".to_string(),
                    2 => "Erase entire line".to_string(),
                    _ => format!("Unknown erase line mode: {}", n),
                }
            },
            CSI::SetGraphicsMode(params) => {
                if params.is_empty() {
                    return "Reset all attributes".to_string();
                }
                
                let descriptions: Vec<String> = params.iter().map(|&param| {
                    match param {
                        0 => "Reset all".to_string(),
                        1 => "Bold".to_string(),
                        2 => "Faint".to_string(),
                        3 => "Italic".to_string(),
                        4 => "Underline".to_string(),
                        5 => "Slow blink".to_string(),
                        6 => "Rapid blink".to_string(),
                        7 => "Reverse".to_string(),
                        8 => "Conceal".to_string(),
                        9 => "Crossed-out".to_string(),
                        21 => "Double underline".to_string(),
                        22 => "Normal intensity".to_string(),
                        23 => "Not italic".to_string(),
                        24 => "Not underlined".to_string(),
                        25 => "Not blinking".to_string(),
                        27 => "Not reversed".to_string(),
                        28 => "Reveal".to_string(),
                        29 => "Not crossed out".to_string(),
                        30..=37 => format!("Foreground color: {}", param - 30),
                        38 => "Set foreground color".to_string(), // Would need more params to decode fully
                        39 => "Reset foreground color".to_string(),
                        40..=47 => format!("Background color: {}", param - 40),
                        48 => "Set background color".to_string(), // Would need more params to decode fully
                        49 => "Reset background color".to_string(),
                        90..=97 => format!("Bright foreground color: {}", param - 90),
                        100..=107 => format!("Bright background color: {}", param - 100),
                        _ => format!("Unknown parameter: {}", param),
                    }
                }).collect();
                
                format!("Set mode: {}", descriptions.join(", "))
            },
            CSI::SetForegroundColor(r, g, b) => format!("Set foreground color to RGB({},{},{})", r, g, b),
            CSI::SetBackgroundColor(r, g, b) => format!("Set background color to RGB({},{},{})", r, g, b),
            CSI::SetForegroundColor256(n) => format!("Set foreground color to 256-color: {}", n),
            CSI::SetBackgroundColor256(n) => format!("Set background color to 256-color: {}", n),
            CSI::ResetAttributes => "Reset all attributes".to_string(),
            CSI::SetMode(params) => {
                let descriptions: Vec<String> = params.iter().map(|&param| {
                    match param {
                        1 => "Application cursor keys".to_string(),
                        2 => "Designate US G0 character set".to_string(),
                        3 => "132 column mode".to_string(),
                        4 => "Smooth scroll".to_string(),
                        6 => "Origin mode".to_string(),
                        7 => "Auto-wrap mode".to_string(),
                        12 => "Start blinking cursor".to_string(),
                        20 => "Automatic newline".to_string(),
                        25 => "Show cursor".to_string(),
                        47 => "Use alternate screen buffer".to_string(),
                        1000 => "Send mouse X/Y on button press and release".to_string(),
                        1001 => "Use hilite mouse tracking".to_string(),
                        1002 => "Use cell motion mouse tracking".to_string(),
                        1003 => "Use all motion mouse tracking".to_string(),
                        1004 => "Send focus events to tty".to_string(),
                        1005 => "Enable UTF-8 mouse mode".to_string(),
                        1006 => "Enable SGR mouse mode".to_string(),
                        1007 => "Enable alternate scroll mode".to_string(),
                        1049 => "Save cursor as in DECSC and use alternate screen buffer".to_string(),
                        _ => format!("Unknown mode: {}", param),
                    }
                }).collect();
                
                format!("Set mode: {}", descriptions.join(", "))
            },
            CSI::ResetMode(params) => {
                let descriptions: Vec<String> = params.iter().map(|&param| {
                    match param {
                        1 => "Normal cursor keys".to_string(),
                        3 => "80 column mode".to_string(),
                        4 => "Jump scroll".to_string(),
                        6 => "Normal cursor mode".to_string(),
                        7 => "No auto-wrap mode".to_string(),
                        12 => "Stop blinking cursor".to_string(),
                        20 => "No automatic newline".to_string(),
                        25 => "Hide cursor".to_string(),
                        47 => "Use normal screen buffer".to_string(),
                        1000..=1003 => "Turn off mouse tracking".to_string(),
                        1004 => "Don't send focus events to tty".to_string(),
                        1005 => "Disable UTF-8 mouse mode".to_string(),
                        1006 => "Disable SGR mouse mode".to_string(),
                        1007 => "Disable alternate scroll mode".to_string(),
                        1049 => "Use normal screen buffer and restore cursor as in DECRC".to_string(),
                        _ => format!("Unknown mode: {}", param),
                    }
                }).collect();
                
                format!("Reset mode: {}", descriptions.join(", "))
            },
            CSI::ApplicationKeypadMode => "Application keypad mode".to_string(),
            CSI::NumericKeypadMode => "Numeric keypad mode".to_string(),
            CSI::SetG0SpecialChars => "Set G0 special chars mode".to_string(),
            CSI::SetG0NormalChars => "Set G0 normal chars mode".to_string(),
            CSI::ScrollUp(n) => format!("Scroll up {} lines", n),
            CSI::ScrollDown(n) => format!("Scroll down {} lines", n),
            CSI::WindowManipulation(params) => {
                if params.is_empty() {
                    return "Unknown window manipulation".to_string();
                }
                
                let operation = params[0];
                match operation {
                    1 => "De-iconify window".to_string(),
                    2 => "Iconify window".to_string(),
                    3 => {
                        if params.len() >= 3 {
                            format!("Move window to ({}, {})", params[1], params[2])
                        } else {
                            "Move window (incomplete parameters)".to_string()
                        }
                    },
                    4 => {
                        if params.len() >= 3 {
                            format!("Resize window to {} rows, {} columns", params[1], params[2])
                        } else {
                            "Resize window (incomplete parameters)".to_string()
                        }
                    },
                    5 => "Raise window to front".to_string(),
                    6 => "Lower window to bottom".to_string(),
                    7 => "Refresh window".to_string(),
                    8 => {
                        if params.len() >= 3 {
                            format!("Resize window to {} rows, {} columns", params[1], params[2])
                        } else {
                            "Resize window (incomplete parameters)".to_string()
                        }
                    },
                    9 => {
                        if params.len() >= 2 {
                            match params[1] {
                                0 => "Restore maximized window".to_string(),
                                1 => "Maximize window".to_string(),
                                2 => "Maximize window vertically".to_string(),
                                3 => "Maximize window horizontally".to_string(),
                                _ => format!("Unknown window manipulation: {}", params[1]),
                            }
                        } else {
                            "Maximize window (incomplete parameters)".to_string()
                        }
                    },
                    10 => {
                        if params.len() >= 2 {
                            match params[1] {
                                0 => "Undo full-screen mode".to_string(),
                                1 => "Change to full-screen mode".to_string(),
                                2 => "Toggle full-screen mode".to_string(),
                                _ => format!("Unknown full-screen operation: {}", params[1]),
                            }
                        } else {
                            "Full-screen manipulation (incomplete parameters)".to_string()
                        }
                    },
                    11 => "Report window state".to_string(),
                    13 => "Report window position".to_string(),
                    14 => "Report window size in pixels".to_string(),
                    18 => "Report window size in characters".to_string(),
                    19 => "Report screen size in characters".to_string(),
                    20 => "Report icon label".to_string(),
                    21 => "Report window title".to_string(),
                    22 => {
                        if params.len() >= 2 {
                            match params[1] {
                                0 => "Save icon and window title".to_string(),
                                1 => "Save icon title".to_string(),
                                2 => "Save window title".to_string(),
                                _ => format!("Unknown title saving operation: {}", params[1]),
                            }
                        } else {
                            "Save window/icon title (incomplete parameters)".to_string()
                        }
                    },
                    23 => {
                        if params.len() >= 2 {
                            match params[1] {
                                0 => "Restore icon and window title".to_string(),
                                1 => "Restore icon title".to_string(),
                                2 => "Restore window title".to_string(),
                                _ => format!("Unknown title restoring operation: {}", params[1]),
                            }
                        } else {
                            "Restore window/icon title (incomplete parameters)".to_string()
                        }
                    },
                    _ => format!("Unknown window manipulation code: {}", operation),
                }
            },
            CSI::DeviceStatusReport => "Request cursor position".to_string(),
            CSI::CursorPositionReport => "Request extended cursor position".to_string(),
            CSI::Unknown(seq) => format!("Unknown CSI sequence: {}", seq),
        }
    }
} 