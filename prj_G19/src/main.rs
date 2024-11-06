mod hardened;
mod fault_list_manager;
mod fault_env;
mod injector;
mod analizer;

use hardened::{Hardened, IncoherenceError};
use fault_list_manager::{FaultListEntry, static_analysis};
use std::{fs, io, panic};
use std::io::{Read, Write};
use syn::{File, Item};
use syn::visit::Visit;
use crate::fault_env::{Data, fault_injection_env};
use clap::Parser;
use std::process::Command;
use crate::fault_list_manager::DimData;
use crate::hardened::run_for_count_selection_sort;

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
    panic::set_hook(Box::new(|_panic_info| {        // SE NECESSARIO RIMUOVERE
        // Print a simple message when a panic occurs
        eprintln!("A panic occurred!");
    }));

    //Per il singolo caso di studio (Selection Sort)
    //TODO: dati letti da file??
    let mut vet = vec![10, 15, 27, -9, 19, 20, 16, 1, 3, -32];
    
    //Prova costruzione di matrici
    let mut mat1 = vec![vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8,]];

    let mut mat2 = vec![vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8],
                        vec![10, 20, 30, 15, 10, 10, 9, 8,]];
    let mut data2 = Data::Matrices(mat1, mat2);

    let mut args=Args::parse();

    //TODO: rimuovi qua! Solo per debug (questo deve essere scelto dall'utente)
    args.case_study=String::from("sel_sort");
    let what=args.case_study.as_str();

    /*per provare analisi statica matrici
    _=static_analysis::generate_analysis_file(
        String::from("src/fault_list_manager/file_fault_list/prova_mat.rs"),
        String::from("src/fault_list_manager/file_fault_list/prova_mat.json")
    );
    */

    match what {
        "sel_sort" => {
            //1. Analisi statica del codice (fornire nomi dei file INPUT/OUTPUT)
            static_analysis::generate_analysis_file(
                String::from("src/fault_list_manager/file_fault_list/selection_sort.rs"),
                String::from("src/fault_list_manager/file_fault_list/sel_sort_ris.json"));


            //2. Generazione della fault list (FL)
            fault_list_manager::create_fault_list(String::from
                                                      ("src/fault_list_manager/file_fault_list/sel_sort_ris.json"),
                                                  DimData::Vector(vet.len()),
                                                  String::from
                                                      ("src/fault_list_manager/file_fault_list/sel_sort_FL\
                                              .json"), run_for_count_selection_sort(&mut vet
                    .clone()));

            //Faccio partire l'ambiente di fault injection
            fault_injection_env(String::from("src/fault_list_manager/file_fault_list/sel_sort_FL\
                                              .json"),                //nome file in cui c'è la FL
                                String::from("abc"),                //nome programma target
                                String::from("abc"),                //nome file report
                                Data::Vector(vet));
        },
        _ => {
            println!("errore menu");
        }
    }
}