mod hardened;
mod fault_list_manager;
mod fault_env;
mod injector;
mod analizer;
mod pdf_generator;

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
use crate::hardened::*;

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
    //let mut data1= Data::Vector(vet);
    //let mut data2 = Data::Matrices(mat1, mat2);

    let mut args=Args::parse();

    //TODO: rimuovi qua! Solo per debug (questo deve essere scelto dall'utente)
    let cases = vec!["sel_sort", "bubble_sort", "matrix_multiplication"];
    //Questo al momento simula il menu (TODO)
    args.case_study=String::from(cases[0]);
    let what=args.case_study.as_str();


    /*per provare analisi statica matrici
    _=static_analysis::generate_analysis_file(
        String::from("src/fault_list_manager/file_fault_list/prova_mat.rs"),
        String::from("src/fault_list_manager/file_fault_list/prova_mat.json")
    );
    */

    //IMPLEMENTAZIONE MENU UTENTE
    /*
    // Descrizione iniziale
    println!("Realizzazione di un ambiente di Fault Injection per applicazione ridondata");

    // Impostiamo un percorso di default
    let default_path = "prj_G19/src/analizer";

    // Chiediamo all'utente di inserire il percorso o usare quello di default
    let user_input: String = Input::new()
        .with_prompt("inserisci output path per il report")
        .default(default_path.to_string())  // Imposta il percorso di default
        .interact_text()
        .unwrap();

    println!("Scegli un algoritmo da utilizzare: ");
    // Definire le opzioni del menu
    let options = vec![
        "Selection Sort",
        "Bubble Sort",
        "Matrix Multiplication"
    ];

    // Crea il menu di selezione
    let selection = Select::new()
        .with_prompt("Please select an operation")
        .default(0) // Selezione predefinita
        .items(&options)
        .interact()
        .unwrap();

    // Mostra l'opzione selezionata
    println!("Hai selezionato: {} e lo stai salvando in {}", options[selection], user_input);

    // Azione in base alla selezione
    match selection {
        0 => {
            //sel 
        }
        1 => {
            //bubble
        }
        2 => {
            //matr mol
        }
        _ => println!("Invalid selection."),
    }
    */

    match what {
        //Caso studio 1: Selection Sort
        "sel_sort" => {
            //1. Analisi statica del codice (fornire nomi dei file INPUT/OUTPUT)
            static_analysis::generate_analysis_file(
                String::from("src/fault_list_manager/file_fault_list/selection_sort.json"),
                String::from("src/fault_list_manager/file_fault_list/sel_sort_ris.json"));
            //2. Generazione della fault list (FL)
            fault_list_manager::create_fault_list(
                String::from("sel_sort"),
                String::from("src/fault_list_manager/file_fault_list/sel_sort_ris.json"),
                DimData::Vector(vet.len()),
                String::from ("src/fault_list_manager/file_fault_list/sel_sort_FL.json"),
                run_for_count_selection_sort(&mut vet.clone())
            );
            //3. Faccio partire l'ambiente di fault injection
            fault_injection_env(
                String::from("src/fault_list_manager/file_fault_list/sel_sort_FL.json"),
                String::from("sel_sort"),
                String::from("abc"),                //nome file report
                Data::Vector(vet));
        },

        //Caso studio 2: Bubble sort
        "bubble_sort" => {
            //1. Analisi statica del codice (fornire nomi dei file INPUT/OUTPUT)
            static_analysis::generate_analysis_file(
                String::from("src/fault_list_manager/file_fault_list/bubble_sort.rs"),
                String::from("src/fault_list_manager/file_fault_list/bubble_sort_ris.json"));
            //2. Generazione della fault list (FL)
            fault_list_manager::create_fault_list(
                String::from("bubble_sort"),
                String::from("src/fault_list_manager/file_fault_list/bubble_sort_ris.json"),
                DimData::Vector(vet.len()),
                String::from("src/fault_list_manager/file_fault_list/bubble_sort_FL.json"),
                run_for_count_bubble_sort(&mut vet.clone()));

            //Faccio partire l'ambiente di fault injection
            /*fault_injection_env(
                String::from("src/fault_list_manager/file_fault_list/sel_sort_FL.json"),
                String::from("bubble_sort"),
                String::from("abc"),
                Data::Vector(vet));
            */
        },
        //Caso studio 3: Matrix multiplication
        "matrix_multiplication" => {
            //1. Analisi statica del codice (fornire nomi dei file INPUT/OUTPUT)
            static_analysis::generate_analysis_file(
                String::from("src/fault_list_manager/file_fault_list/matrix_multiplication.rs"),
                String::from("src/fault_list_manager/file_fault_list/matrix_mul_ris.json"));

            //2. Generazione della fault list (FL)
            fault_list_manager::create_fault_list(
                String::from("matrix_multiplication"),
                String::from("src/fault_list_manager/file_fault_list/matrix_mul_ris.json"),
                DimData::Matrix((mat1.len(), mat1[0].len())),
                String::from("src/fault_list_manager/file_fault_list/matrix_mul_FL.json"),
                run_for_count_matrix_mul(&mat1.clone(),&mat2.clone())
            );

            //Faccio partire l'ambiente di fault injection
            /*fault_injection_env(
                String::from("src/fault_list_manager/file_fault_list/matrix_mul_FL.json"),
                String::from("matrix_multiplication"),
                String::from("abc"),
                Data::Matrices(mat1,mat2)
            );*/
        },

        _ => {
            println!("errore menu");
        }
    }
}