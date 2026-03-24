use crate::frontend_types::Span;
use crate::macros::CarawayMacro;
use crate::ast::Primary;

pub struct HexColorMacro;

impl CarawayMacro for HexColorMacro {
    fn name(&self) -> &'static str {
        "hex"
    }

    fn expand_primary(&self, content: &str, span: Span) -> Result<Primary, String> {
        let content = content.trim();
        if content.len() != 6 {
            return Err("Hex color must be exactly 6 characters long.".to_string());
        }

        // Parse the hex string
        let r = u8::from_str_radix(&content[0..2], 16).map_err(|_| "Invalid hex for Red")?;
        let g = u8::from_str_radix(&content[2..4], 16).map_err(|_| "Invalid hex for Green")?;
        let b = u8::from_str_radix(&content[4..6], 16).map_err(|_| "Invalid hex for Blue")?;

        // Generate standard Caraway AST nodes!
        let rgb_list = vec![
            Primary::Number(r as f64, span),
            Primary::Number(g as f64, span),
            Primary::Number(b as f64, span),
        ];

        Ok(Primary::List(rgb_list, span))
    }
}
