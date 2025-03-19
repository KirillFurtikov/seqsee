pub mod csi;
pub mod ctrl;

#[derive(Debug, Clone)]
pub enum AnsiElement {
    Text(String),
    Csi(csi::CSI),
    Ctrl(ctrl::ControlCharacter),
}

impl AnsiElement {
    pub fn element_type(&self) -> &'static str {
        match self {
            AnsiElement::Text(_) => "Text",
            AnsiElement::Csi(_) => "CSI",
            AnsiElement::Ctrl(_) => "Ctrl",
        }
    }
    
    pub fn escape_repr(&self) -> String {
        match self {
            AnsiElement::Text(text) => text.clone(),
            AnsiElement::Csi(csi) => csi.escape_repr(),
            AnsiElement::Ctrl(ctrl) => ctrl.escape_repr(),
        }
    }
    
    pub fn description(&self) -> String {
        match self {
            AnsiElement::Text(text) => format!("{}", text),
            AnsiElement::Csi(csi) => csi.description(),
            AnsiElement::Ctrl(ctrl) => ctrl.description(),
        }
    }
} 