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
    //Per il singolo caso di studio (Selection Sort)

    //1. Analisi statica del codice (fornire nomi dei file INPUT/OUTPUT)
    static_analysis::generate_analysis_file(
        String::from("src/fault_list_manager/file_fault_list/selection_sort.rs"),
        String::from("src/fault_list_manager/file_fault_list/sel_sort_ris.json"));

    //2. Generazione della fault list (FL)

    fault_list_manager::create_fault_list(String::from
    ("src/fault_list_manager/file_fault_list/sel_sort_ris.json"),10,
                                          String::from
                                              ("src/fault_list_manager/file_fault_list/sel_sort_FL\
                                              .json"));

    //Faccio partire l'ambiente di fault injection
    fault_injection_env(    String::from("src/fault_list_manager/file_fault_list/sel_sort_FL\
                                              .json"),                //nome file in cui c'è la FL
                            String::from("abc"),                //nome programma target
                            String::from("abc"));               //nome file report
}