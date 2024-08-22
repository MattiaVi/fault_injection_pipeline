mod hardened;
mod fault_list_manager;

use hardened::{Hardened, IncoherenceError};
use fault_list_manager::{FaultListEntry, static_analysis};
use std::fs;
use syn::{File, Item};
use syn::visit::Visit;


fn main(){
    let file_path = "src/fault_list_manager/\
                                    file_fault_list\
                                    /selection_sort.rs";
    let code = fs::read_to_string(file_path)
        .expect("Failed to read file");

    let file: File = syn::parse_str(&code).expect("Failed to parse code");

    for item in file.items {
        if let syn::Item::Fn(func) = item {
            static_analysis::analyze_function(&func);
        }
    }
}