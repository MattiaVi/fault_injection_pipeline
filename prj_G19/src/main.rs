mod hardened;
mod fault_list_manager;
mod fault_env;
mod injector;
mod analizer;

use hardened::{Hardened, IncoherenceError};
use fault_list_manager::{FaultListEntry, static_analysis};
use std::{fs, io};
use std::io::{Read, Write};
use syn::{File, Item};
use syn::visit::Visit;
use crate::fault_env::fault_injection_env;
use clap::Parser;
use std::process::Command;

//TODO: completare con quello che serve per la realizzazione del menu da linea di comando
///Ambiente di Fault Injection per applicazione ridondata
#[derive(Parser,Debug)]
#[command(version,long_about=None)]
struct Args{
    #[arg(short, long, default_value ="Ciao")]
    case_study: String,
}

//Per uso futuro...
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main(){
    //Per il singolo caso di studio (Selection Sort)
    let vet = vec![10,15,27,-9,19,20,16, 1, 3, 40];

    /*
    let args=Args::parse();
    println!("{}", args.case_study);*/

    //1. Analisi statica del codice (fornire nomi dei file INPUT/OUTPUT)
    static_analysis::generate_analysis_file(
        String::from("src/fault_list_manager/file_fault_list/selection_sort.rs"),
        String::from("src/fault_list_manager/file_fault_list/sel_sort_ris.json"));

    //2. Generazione della fault list (FL)
    fault_list_manager::create_fault_list(String::from
    ("src/fault_list_manager/file_fault_list/sel_sort_ris.json"),vet.len(),
                                          String::from
                                              ("src/fault_list_manager/file_fault_list/sel_sort_FL\
                                              .json"));

    //Faccio partire l'ambiente di fault injection
    fault_injection_env(    String::from("src/fault_list_manager/file_fault_list/sel_sort_FL\
                                              .json"),                //nome file in cui c'è la FL
                            String::from("abc"),                //nome programma target
                            String::from("abc"));               //nome file report
}