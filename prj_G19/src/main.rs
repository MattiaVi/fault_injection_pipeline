mod hardened;
mod fault_list_manager;
mod fault_env;
mod injector;
mod analizer;

use hardened::{Hardened, IncoherenceError};
use fault_list_manager::{FaultListEntry, static_analysis};
use std::fs;
use syn::{File, Item};
use syn::visit::Visit;
use crate::fault_env::fault_injection_env;

fn main(){
    static_analysis::generate_analysis_file(
        String::from("src/fault_list_manager/file_fault_list/selection_sort.rs"),
        String::from("src/fault_list_manager/file_fault_list/sel_sort_ris.txt"));

    fault_injection_env(String::from("abc"), String::from("abc"), String::from("abc"));
}