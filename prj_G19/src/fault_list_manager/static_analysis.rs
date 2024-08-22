use syn::{File, ItemFn, Block, Stmt, Pat, Type, Expr, FnArg};
use quote::ToTokens;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use crate::fault_list_manager::static_analysis;

//Analizza la funzione
pub fn analyze_function(func: &ItemFn, file_path_dest: String) {
    //println!("Function: {}", func.sig.ident);

    //TODO: Rimuovere lo expect  per gestire l'errore
    let mut fp= OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path_dest)
        .unwrap();

    let body = &func.block;
    let mut instruction_count = 0;
    let mut variable_types = HashMap::new();

    // Count the number of instructions and extract variable types
    instruction_count += count_statements(&body, &mut variable_types);

    // Extract variables
    let mut variables = Vec::new();
    extract_variables(&func, &variable_types, &mut variables);

    //todo: Queste informazioni vanno salvate su un file al posto di essere stampate
    //Formato del file:
    // <N>
    // <name1> <type1> <size1>
    // ...
    //<nameN> <typeN> >sizeN>

    //dove N è il numero di istruzioni

    //todo: rimuovi expect()
    fp.write_all(format!("{}\n", instruction_count).as_bytes()).expect("errore");
    //println!("Variables:");
    for var in variables {
        //println!("Name: {}, Type: {}, Size: {}", var.name, var.ty, var.size);
        fp.write_all( format!("{} {} {}\n", var.name, var.ty, var.size).as_bytes()).expect("errore");
    }
}

fn count_statements(block: &Block, variable_types: &mut HashMap<String, String>) -> usize {
    let mut count = 0;
    let mut local_variables = HashMap::new();

    for stmt in &block.stmts {
        match stmt {
            Stmt::Local(local) => {
                count += 1;
                // Estrazione del nome e del tipo della variabile
                if let Pat::Type(pat_type) = &local.pat {
                    if let Pat::Ident(pat_ident) = &*pat_type.pat {
                        let var_name = pat_ident.ident.to_string();
                        let var_type = extract_type(&*pat_type.ty);
                        local_variables.insert(var_name.clone(), var_type.clone());
                    }
                } else if let Pat::Ident(pat_ident) = &local.pat {
                    let var_name = pat_ident.ident.to_string();
                    let var_type = if let Some(init) = &local.init {
                        infer_type_from_expr(&init.expr)
                    } else {
                        "unknown".to_string()
                    };
                    local_variables.insert(var_name.clone(), var_type.clone());
                }
            }
            Stmt::Expr(expr,_) => {
                count += 1;                         //Il while/if/for/lo conto come istruzione!
                //cicli while
                if let Expr::While(while_expr) = expr {
                    count += count_statements(&while_expr.body, &mut local_variables);
                }
                //if/elseif
                else if let Expr::If(if_expr) = expr{
                    count += count_statements(&if_expr.then_branch, &mut local_variables);
                    if let Some((_, else_branch)) = &if_expr.else_branch {
                        count += count_statements_in_expr(else_branch, &mut local_variables);
                    }
                }
                //for
                else if let Expr::ForLoop(for_expr) = expr{
                    count += count_statements(&for_expr.body, &mut local_variables);
                }
            }
            _ => {}
        }
    }

    // Aggiornamento delle variabili globali con quelle locali
    // tramite concatenazione delle due collezioni
    variable_types.extend(local_variables);
    count
}

fn count_statements_in_expr(expr: &Expr, variable_types: &mut HashMap<String, String>) -> usize {
    match expr {
        Expr::Block(block_expr) => count_statements(&block_expr.block, variable_types),
        Expr::If(if_expr) => {
            let mut count = count_statements(&if_expr.then_branch, variable_types);
            if let Some((_, else_branch)) = &if_expr.else_branch {
                count += count_statements_in_expr(else_branch, variable_types);
            }
            count
        }
        _ => 0,
    }
}

fn extract_type(ty: &Type) -> String {
    // Conversione del tipo a stringa
    ty.to_token_stream().to_string()
}

//Funzione ricorsiva che inferisce il tipo da una certa espressione
fn infer_type_from_expr(expr: &Expr) -> String {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Int(_) => "i32".to_string(),
            syn::Lit::Float(_) => "f64".to_string(),
            syn::Lit::Str(_) => "String".to_string(),
            syn::Lit::Bool(_) => "bool".to_string(),
            _ => "unknown".to_string(),
        },
        Expr::Assign(binary) => {
            let left_type = infer_type_from_expr(&binary.left);
            let right_type = infer_type_from_expr(&binary.right);
            if left_type == right_type {

                left_type
            } else {
                "unknown".to_string()
            }
        },
        Expr::Unary(unary) => infer_type_from_expr(&unary.expr),
        _ => "unknown".to_string(),
    }
}

//Effettua il binding tipo<-->dimensione
fn type_size(type_str: &str) -> String {
    match type_str {
        "i8" | "u8" => "1",
        "i16" | "u16" => "2",
        "i32" | "u32" => "4",
        "i64" | "u64" => "8",
        "isize" | "usize" => "4", // Assumendo architettura a 32-bit; usare "8" per 64-bit
        "f32" => "4",
        "f64" => "8",
        "bool"=>"1",
        "Vec < i32 >" => "4*len",
        _ => "unknown",
    }
        .to_string()
}

//Tipo che conserva le informazioni di una certa variabile
struct Variable {
    name: String,
    ty: String,
    size: String,
}

fn extract_variables(func: &ItemFn, variable_types: &HashMap<String, String>, variables: &mut Vec<Variable>) {

    // Estrazione dei parametri della funzione
    for param in &func.sig.inputs {
        if let FnArg::Typed(pat_type) = param {
            let ty = if let Type::Path(type_path) = &*pat_type.ty {
                type_path.to_token_stream().to_string()
            } else {
                "unknown".to_string()
            };

            let name = if let Pat::Ident(pat_ident) = &*pat_type.pat {
                pat_ident.ident.to_string()
            } else {
                "parameter".to_string()
            };

            variables.push(Variable {
                name,
                ty: ty.clone(),
                size: type_size(&ty),
            });
        }
    }

    // Estrazione delle variabili locali
    for (name, ty) in variable_types {
        variables.push(Variable {
            name: name.clone(),
            ty: ty.clone(),
            size: type_size(&ty),
        });
    }
}

//todo: Eliminare expect in modo da far ritornare un Result alla funzione
pub fn generate_analysis_file(file_path_src: String, file_path_dest: String){
    let code = fs::read_to_string(file_path_src)
        .expect("Failed to read file");
    let file: File = syn::parse_str(&code).expect("Failed to parse code");

    for item in file.items {
        if let syn::Item::Fn(func) = item {
            analyze_function(&func, file_path_dest.clone());
        }
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_prova(){
        assert_eq!(2,2);
    }
}