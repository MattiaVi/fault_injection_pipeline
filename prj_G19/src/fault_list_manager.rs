//Analisi statica del codice
use syn::{visit::Visit, File, Item, FnArg, Pat, Stmt, Block, Local};

// Visitor per raccogliere variabili
pub struct VariableVisitor {
    pub variables: Vec<String>,
}

impl<'ast> Visit<'ast> for VariableVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        // Visita i parametri della funzione
        for arg in &node.sig.inputs {
            match arg {
                FnArg::Typed(pat_type) => {
                    if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        self.variables.push(pat_ident.ident.to_string());
                    }
                }
                _ => {}
            }
        }
        // Continua la visita per le variabili locali nel corpo della funzione
        self.visit_block(&node.block);
    }

    fn visit_block(&mut self, block: &'ast syn::Block) {
        for stmt in &block.stmts {
            match stmt {
                Stmt::Local(local) => {
                    // `local.init` è un Option<Expr>, quindi bisogna gestirlo come tale
                    if let Some(init) = &local.init {
                        // Il pattern di `local.pat` deve essere esaminato correttamente
                        if let Pat::Ident(pat_ident) = &*local.pat {
                            self.variables.push(pat_ident.ident.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
        // Visita anche eventuali blocchi annidati
        for stmt in &block.stmts {
            if let Stmt::Block(inner_block) = stmt {
                self.visit_block(inner_block);
            }
        }
    }
}


///Generazione della fault list:
///     - generazione casuale di un certo numero di entry +

pub struct FaultListEntry{
    var: String,
    time: usize,
    fault_mask: u64,
}

//Fault List Manager

#[cfg(test)]
mod tests{
    #[test]
    fn test_trivial(){
        assert_eq!(2,2);
    }
}