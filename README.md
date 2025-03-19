# 🎨 Seqsee

Human-readable ANSI sequences for Rust.

Seqsee is a specialized command-line tool designed to demystify ANSI escape sequences. It helps developers debug terminal-based applications by translating cryptic control codes into human-readable explanations. Whether you're troubleshooting cursor positioning issues, color rendering problems, or trying to understand how terminal applications manipulate the display, Seqsee provides clarity by revealing what's happening beneath the surface of your terminal.

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/KirillFurtikov/seqsee.git
cd seqsee

# Build the project
cargo build --release

# Install the binary
cargo install --path .
```

### From cargo
```bash
cargo install seqsee
```

## 💡 Usage

### 🔍 Describing escape sequences

Just use `printf` to send some sequences to `seqsee` for an explanation:

```bash
printf "\x1b[36mRust\x1b[1;4m is \x1b[41m awesome\x1b[0m\r\n\b" | seqsee
```

This will output a detailed breakdown of each ANSI sequence:

<img width="424" alt="image" src="https://github.com/user-attachments/assets/e45f8091-a3dc-4ba0-8779-cf91b5587afb" />


### 📄 Reading text files with escape sequences

You can parse files containing ANSI escape sequences:

```bash
seqsee -f test_ansi.txt
```

Seqsee will parse both the actual escape sequences and also the literal `\e` escape notations.

### 🔬 Examining command output

Many commands use ANSI sequences for colorized output. You can analyze them with seqsee:

```bash
ls --color=always | seqsee
git -c color.status=always status | seqsee
```

Note: Many programs disable colored output when piping. Use flags like `--color=always` to force them to include ANSI sequences.

### 🎯 Raw Mode

For a cleaner view of the text with highlighted sequences:

```bash
ls --color=always | seqsee --raw
```

## ✨ Supported ANSI Features

Seqsee supports parsing and explaining a wide range of ANSI escape sequences:

### 🖱️ Cursor Movement
- Cursor Up/Down/Forward/Backward
- Cursor Position (absolute and relative)
- Save/Restore cursor position
- Column positioning

### 🎨 Text Formatting
- Text styling (bold, italic, underline, etc.)
- 16-color mode (30-37, 40-47, 90-97, 100-107)
- 256-color mode (38;5;n and 48;5;n)
- RGB true color (38;2;r;g;b and 48;2;r;g;b)

### 🖥️ Screen Control
- Erase in display/line
- Scrolling
- Window manipulation

### ⚙️ Terminal Modes
- Application/Numeric keypad mode
- Character sets
- Various terminal modes (like mouse tracking)

## 📁 Project Structure

```
seqsee/
├── src/
│   ├── ansi/           # ANSI sequence definitions
│   │   ├── csi.rs      # CSI (Control Sequence Introducer) commands
│   │   ├── ctrl.rs     # Control characters
│   │   └── mod.rs      # Module definitions
│   ├── output/         # Output formatting
│   │   ├── raw.rs      # Raw output formatter
│   │   ├── table.rs    # Table output formatter
│   │   └── mod.rs      # Module definitions
│   ├── parser.rs       # ANSI sequence parser
│   ├── formatter.rs    # Formatter trait
│   └── main.rs         # CLI application
└── test_ansi.txt       # Example ANSI test file
```

## 🤝 Contributing

Contributions are welcome! Here are some ways you can contribute:

- Report bugs and feature requests
- Improve documentation
- Fix bugs and implement features
- Add support for additional ANSI sequences

## 📄 License

This project is licensed under the MIT License.
