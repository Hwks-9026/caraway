use crate::frontend_types::Span;
use crate::ast::*;
use std::sync::Arc;
use std::collections::HashMap;

// We allow macros to expand to different AST nodes depending on where they are called.
pub trait CarawayMacro: Send + Sync { 
    fn name(&self) -> &'static str;

    // By default, macros return an error if used in the wrong context.
    // The implementations will only override the supported contexts.
    
    fn expand_primary(&self, _content: &str, _span: Span) -> Result<Primary, String> {
        Err(format!("Macro '@{}' cannot be used as an expression.", self.name()))
    }

    fn expand_lhs(&self, _content: &str, _span: Span) -> Result<AssignmentLhs, String> {
        Err(format!("Macro '@{}' cannot be used as an assignment target.", self.name()))
    }
    
    fn expand_import(&self, _content: &str, _span: Span) -> Result<ImportPath, String> {
        Err(format!("Macro '@{}' cannot be used in an import statement.", self.name()))
    }
}

#[derive(Clone)]
pub struct MacroRegistry {
    registry: Arc<HashMap<String, Arc<dyn CarawayMacro>>>,
}

impl MacroRegistry {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(HashMap::new()),
        }
    }

    // builder method for chaining macro additions
    pub fn with_macro(mut self, m: impl CarawayMacro + 'static) -> Self {
        let mut map = Arc::try_unwrap(self.registry).unwrap_or_else(|arc| (*arc).clone());
        map.insert(m.name().to_string(), Arc::new(m));
        self.registry = Arc::new(map);
        self
    }

    pub fn get(&self, name: &str) -> Option<&dyn CarawayMacro> {
        self.registry.get(name).map(|m| m.as_ref())
    }
}
