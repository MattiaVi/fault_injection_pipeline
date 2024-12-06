mod hardened;
mod fault_list_manager;
mod fault_env;
mod injector;
mod analyzer;
mod pdf_generator;

use hardened::{Hardened, IncoherenceError};
use fault_list_manager::{FaultListEntry, static_analysis};
use std::io::{BufRead, Error, Read, Write};
use std::io;
use std::path::Path;
use std::fs::File;
use syn::visit::Visit;
use crate::fault_env::{Data, fault_injection_env};
use clap::Parser;
use crate::fault_list_manager::DimData;
use crate::hardened::*;
use dialoguer::{Select, Input};
use regex::Regex;


///Ambiente di Fault Injection per applicazione ridondata


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

pub struct InputData {
    pub vector: Vec<i32>,
    pub matrix_size: usize,
    pub matrix1: Vec<Vec<i32>>,
    pub matrix2: Vec<Vec<i32>>,
}
pub fn load_data_from_file(file_path: &str) -> Result<InputData, Error> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let mut lines = io::BufReader::new(file).lines();

    // Saltiamo il testo iniziale fino a quando non incontriamo una linea che inizia con un numero
    let mut current_line = String::new();
    while let Some(Ok(line)) = lines.next() {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() && trimmed_line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            current_line = trimmed_line.to_string();
            break;
        }
    }
    // Leggiamo la dimensione del vettore dalla riga corrente
    let vector_size: usize = current_line
        .parse::<usize>()
        .map_err(|_| Error::new(io::ErrorKind::InvalidData, "Formato invalido per la dimensione del vettore"))?;


    // Trova la riga del vettore, saltando righe vuote
    let mut vector_line = String::new();
    while let Some(Ok(line)) = lines.next() {
        if !line.trim().is_empty() {
            vector_line = line;
            break;
        }
    }
    
    let vector: Vec<i32> = vector_line
        .trim()
        .split(',') 
        .map(|n| n.trim()) 
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::new(io::ErrorKind::InvalidData, "Formato invalido nel vettore"))?;

    // Verifica che la dimensione del vettore corrisponda a quella dichiarata
    if vector.len() != vector_size {
        return Err(Error::new(io::ErrorKind::InvalidData, format!("La dimensione del vettore ({}) non corrisponde ai dati forniti ({})", vector_size,vector.len())));
    }

    // Trova la dimensione della matrice, saltando righe vuote
    let mut matrix_size_line = String::new();
    while let Some(Ok(line)) = lines.next() {
        if !line.trim().is_empty() {
            matrix_size_line = line;
            break;
        }
    }

    // Leggi la dimensione delle matrici
    let matrix_size: usize = matrix_size_line
        .trim()
        .parse::<usize>()
        .map_err(|_| Error::new(io::ErrorKind::InvalidData, "Formato invalido per la dimensione della matrice"))?;


    // Leggi la prima matrice
    let mut matrix1 = Vec::new();
    for _ in 0..matrix_size {
        let row_line = loop {
            if let Some(Ok(line)) = lines.next() {
                if !line.trim().is_empty() {
                    break line;
                }
            }
        };

        let row: Vec<i32> = row_line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Formato invalido nelle righe della matrice 1"))?;

        if row.len() != matrix_size {
            return Err(Error::new(io::ErrorKind::InvalidData, "La dimensione della matrice 1 non corrisponde ai dati forniti"));
        }
        matrix1.push(row);
    }

    // Leggi la seconda matrice
    let mut matrix2 = Vec::new();
    for _ in 0..matrix_size {
        let row_line = loop {
            if let Some(Ok(line)) = lines.next() {
                if !line.trim().is_empty() {
                    break line;
                }
            } else {
                return Err(Error::new(
                    io::ErrorKind::InvalidData,
                    "Righe della matrice 2 mancanti",
                ));
            }
        };

        let row: Vec<i32> = row_line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Formato invalido nelle righe della matrice 2"))?;

        if row.len() != matrix_size {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                "La dimensione della matrice 2 non corrisponde ai dati forniti",
            ));
        }
        matrix2.push(row);
    }
    
    Ok(InputData {
        vector,
        matrix_size,
        matrix1,
        matrix2,
    })
}

fn main() {
    //API KEY per prendere vettori per algoritmi di ordinamento

    kaggle::Authentication::with_credentials("federicopretini", "5b7355de00b8dc63f52f18be16918e00");

    /*
    panic::set_hook(Box::new(|_panic_info| {        // SE NECESSARIO RIMUOVERE
        // Print a simple message when a panic occurs
        eprintln!("A panic occurred!");
    }));
    */

    //IMPLEMENTAZIONE MENU UTENTE---------------------------

    // Descrizione iniziale
    println!("Realizzazione di un ambiente di Fault Injection per applicazione ridondata");

    // Impostiamo un percorso di default per salvare il pdf generato
    let mut file_path: String = "results/".to_string();
    let input_path: String = "src/data/input.txt".to_string();

    let mut nome_file: String = Input::new()
        .with_prompt("Inserisci il nome del file per il report senza estensione")
        .default("demo".to_string())  // Imposta il percorso di default
        .interact_text()
        .unwrap();

    let regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();

    while !regex.is_match(&nome_file) {
        println!("Nome file invalido, per favore ritenta");
        nome_file = Input::new()
            .with_prompt("Inserisci il nome del file per il report SENZA ESTENSIONE")
            .default("demo".to_string())  // Imposta il percorso di default
            .interact_text()
            .unwrap();
    }
    file_path.push_str(&nome_file);

    // Sorgente dei dati
    let data_sources = vec!["Data file", "Dataset"];
    let data_source_selection = Select::new()
        .with_prompt("Seleziona la sorgente dei dati")
        .items(&data_sources)
        .interact()
        .unwrap();

    // Caricamento dati in base alla sorgente scelta
    let input_data = match data_source_selection {
        0 => match load_data_from_file(&input_path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Errore: {}", e);
                std::process::exit(1);
            }
        },
        //TODO: implemetare API
        1 => match load_data_from_file(&input_path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Errore: {}", e);
                std::process::exit(1);
            }
        },
        _ => unreachable!(),
    };

    //println!("Dati caricati: {:?}", data);
    let mut num_faults: i32 = 2000;

    // Scelta tra singolo algoritmo o tutti
    let operation_modes = vec!["Esegui un singolo algoritmo", "Esegui un'analisi comparativa tra tutti gli algoritmi"];
    let mode_selection = Select::new()
        .with_prompt("Seleziona il tipo di analisi")
        .items(&operation_modes)
        .default(0)
        .interact()
        .unwrap();

    match mode_selection {

        // Caso del singolo algoritmo
        0 => {
            file_path.push_str(".pdf");
            //scegli algoritmo--------------------------------------------------------------------------
            let options = vec![
                "Selection Sort",
                "Bubble Sort",
                "Matrix Multiplication"
            ];

            // Menu di selezione
            let algo_selection = Select::new()
                .with_prompt("Scegli un algoritmo da utilizzare")
                .default(0) // Selezione predefinita
                .items(&options)
                .interact()
                .unwrap();

            // Mostriamo l'opzione selezionata
            println!("Hai selezionato: {} e lo stai salvando in {}", options[algo_selection], nome_file);

            //--------------------------------------------------------------------------

            //scelta tra H/non H, variazione tra #fault list
            let options = vec![
                "Not Hardened vs Hardened",
                "Variazione cardinalità fault list entries",
            ];

            let single_algo_anlysis_selection = Select::new()
                .with_prompt("Scegli una modalità di single analysis")
                .default(0) // Selezione predefinita
                .items(&options)
                .interact()
                .unwrap();

            // Mostriamo l'opzione selezionata
            println!("Hai selezionato: {}", options[single_algo_anlysis_selection]);


            match single_algo_anlysis_selection {
                0 => {
                    num_faults = Input::new()
                        .with_prompt("Inserisci il numero di fault entries desiderate")
                        .default(2000)
                        .interact_text()
                        .unwrap();

                    match algo_selection {
                        0 => {
                            // Caso studio 1: Selection Sort
                            //let mut vettore = Data::Vector(input_data.vector.clone()); //let mut vettore= vet.clone();
                            run_case_study(
                                num_faults,
                                "sel_sort",
                                &file_path,
                                Data::Vector(input_data.vector.clone()),
                                DimData::Vector(input_data.vector.len()),
                                "src/fault_list_manager/file_fault_list/selection_sort/selection_sort.json",
                                "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_ris.json",
                                "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_FL.json",
                                |vettore| run_for_count_selection_sort(vettore),
                            );
                        }
                        1 => {
                            // Caso studio 2: Bubble Sort
                            let mut vettore = Data::Vector(input_data.vector.clone());
                            run_case_study(
                                num_faults,
                                "bubble_sort",
                                &file_path,
                                Data::Vector(input_data.vector.clone()),
                                DimData::Vector(input_data.vector.len()),
                                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort.rs",
                                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_ris.json",
                                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_FL.json",
                                |vettore| run_for_count_bubble_sort(vettore),
                            );
                        }
                        2 => {
                            // Caso studio 3: Matrix Multiplication
                            let mut matrici = Data::Matrices(input_data.matrix1.clone(), input_data.matrix2.clone());
                            run_case_study(
                                num_faults,
                                "matrix_multiplication",
                                &file_path,
                                Data::Matrices(input_data.matrix1.clone(), input_data.matrix2.clone()),
                                DimData::Matrix((input_data.matrix1.len(), input_data.matrix_size)),
                                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_multiplication.rs",
                                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_ris.json",
                                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_FL.json",
                                |matrici| run_for_count_matrix_mul(matrici),
                            );
                        }
                        _ => println!("Invalid selection."),
                    }
                }
                1 => {
                    let cardinalities: Vec<i32> = vec![1000, 2000, 3000];
                    let mut vettore = Data::Vector(input_data.vector.clone());
                    match algo_selection {
                        0 => {
                            // Caso studio 1: Selection Sort
                            for cardinality in cardinalities{
                                run_case_study(
                                    cardinality,
                                    "sel_sort",
                                    &file_path,
                                    Data::Vector(input_data.vector.clone()),
                                    DimData::Vector(input_data.vector.len()),
                                    "src/fault_list_manager/file_fault_list/selection_sort/selection_sort.json",
                                    "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_ris.json",
                                    "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_FL.json",
                                    |vettore| run_for_count_selection_sort(vettore),
                                );
                            }
                        }
                        1 => {
                            // Caso studio 2: Bubble Sort
                            //let mut vettore = Data::Vector(input_data.vector.clone());
                            for cardinality in cardinalities{
                                run_case_study(
                                    cardinality,
                                    "bubble_sort",
                                    &file_path,
                                    Data::Vector(input_data.vector.clone()),
                                    DimData::Vector(input_data.vector.len()),
                                    "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort.rs",
                                    "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_ris.json",
                                    "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_FL.json",
                                    |vettore| run_for_count_bubble_sort(vettore),
                                );
                            }
                        }
                        2 => {
                            // Caso studio 3: Matrix Multiplication
                            let mut matrici = Data::Matrices(input_data.matrix1.clone(), input_data.matrix2.clone());
                            for cardinality in cardinalities {
                                run_case_study(
                                    cardinality,
                                    "matrix_multiplication",
                                    &file_path,
                                    Data::Matrices(input_data.matrix1.clone(), input_data.matrix2.clone()),
                                    DimData::Matrix((input_data.matrix1.len(), input_data.matrix_size)),
                                    "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_multiplication.rs",
                                    "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_ris.json",
                                    "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_FL.json",
                                    |matrici| run_for_count_matrix_mul(matrici),
                                );
                            }
                        }
                        _ => println!("Invalid selection."),
                    }
                }
                _ => println!("Invalid selection."),
            }
        }

        1 => {
            // Esegui tutti gli algoritmi
            num_faults = Input::new()
                .with_prompt("Inserisci il numero di fault entries desiderate")
                .default(2000)
                .interact_text()
                .unwrap();
            file_path.push_str("_all.pdf");
            println!("{:?}", file_path);

            // Caso studio 1: Selection Sort
            let mut vettore = Data::Vector(input_data.vector.clone());
            run_case_study(
                num_faults,
                "sel_sort",
                &file_path,
                Data::Vector(input_data.vector.clone()),
                DimData::Vector(input_data.vector.len()),
                "src/fault_list_manager/file_fault_list/selection_sort/selection_sort.json",
                "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_ris.json",
                "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_FL.json",
                |vettore| run_for_count_selection_sort(vettore),
            );

            // Caso studio 2: Bubble Sort
            let mut vettore = Data::Vector(input_data.vector.clone());
            run_case_study(
                num_faults,
                "bubble_sort",
                &file_path,
                Data::Vector(input_data.vector.clone()),
                DimData::Vector(input_data.vector.len()),
                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort.rs",
                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_ris.json",
                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_FL.json",
                |vettore| run_for_count_bubble_sort(vettore),
            );

            // Caso studio 3: Matrix Multiplication
            let mut matrici = Data::Matrices(input_data.matrix1.clone(), input_data.matrix2.clone());
            run_case_study(
                num_faults,
                "matrix_multiplication",
                &file_path,
                Data::Matrices(input_data.matrix1.clone(), input_data.matrix2.clone()),
                DimData::Matrix((input_data.matrix1.len(), input_data.matrix_size)),
                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_multiplication.rs",
                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_ris.json",
                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_FL.json",
                |matrici| run_for_count_matrix_mul(matrici),
            );
        }
        _ => unreachable!(),
    }
    println!("Operazione completata. Report salvato in: {}", file_path);


    fn run_case_study(num_faults: i32,
                      case_name: &str,
                      file_path: &str,
                      input_data: Data<i32>,
                      dim_data: DimData,
                      analysis_input_file: &str,
                      analysis_output_file: &str,
                      fault_list_file: &str,
                      fault_list_run: impl FnOnce(Data<i32>) -> usize) {
        // 1. Analisi statica del codice

        // TODO: cercare di gestire l'errore magari con un expect
        static_analysis::generate_analysis_file(
            analysis_input_file.to_string(),
            analysis_output_file.to_string(),
        );

        // 2. Generazione della fault list (FL)
        fault_list_manager::create_fault_list(
            num_faults,
            case_name.to_string(),
            analysis_output_file.to_string(),
            dim_data,
            fault_list_file.to_string(),
            fault_list_run(input_data.clone()),
        );

        // 3. Faccio partire l'ambiente di fault injection
        fault_injection_env(
            fault_list_file.to_string(),
            case_name.to_string(),
            file_path.to_string(),
            input_data.clone(),
        );
    }
}
