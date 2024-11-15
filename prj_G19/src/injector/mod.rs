mod algorithms;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{panic, thread, vec};
use dialoguer::Input;
use crate::fault_list_manager::FaultListEntry;
use crate::hardened::{Hardened, IncoherenceError};
use algorithms::{runner_selection_sort};
use crate::injector::algorithms::runner_bubble_sort;


#[derive(Debug)]
pub struct TestResult {
    fault_list_entry: FaultListEntry,
    result: Result<(), IncoherenceError>
}
impl TestResult {
    pub fn get_result(&self) -> Result<(), IncoherenceError> {
        self.result.clone()
    }
}
enum AlgorithmVariables {
    SelectionSort(SelectionSortVariables),
    BubbleSort(BubbleSortVariables),
    MatrixMultiplication(MatrixMultiplicationVariables),
}

struct SelectionSortVariables {
    i: RwLock<Hardened<usize>>,
    j: RwLock<Hardened<usize>>,
    N: RwLock<Hardened<usize>>,
    min: RwLock<Hardened<usize>>,
    vec: RwLock<Vec<Hardened<i32>>>,
}

struct BubbleSortVariables {
    i: RwLock<Hardened<usize>>,
    j: RwLock<Hardened<usize>>,
    N: RwLock<Hardened<usize>>,
    swapped: RwLock<Hardened<bool>>,
    vet: RwLock<Vec<Hardened<i32>>>,
}

struct MatrixMultiplicationVariables {
    size: RwLock<Hardened<usize>>,
    i: RwLock<Hardened<usize>>,
    j: RwLock<Hardened<usize>>,
    k: RwLock<Hardened<usize>>,
    row: RwLock<Vec<Hardened<i32>>>,
    acc: RwLock<Hardened<i32>>,
    a: RwLock<Vec<Vec<Hardened<i32>>>>,
    b: RwLock<Vec<Vec<Hardened<i32>>>>,
    result: RwLock<Vec<Vec<Hardened<i32>>>>
}

// Common initialization trait
trait VariableSet {
    type Input;
    fn new(input: Self::Input) -> Self;
}

impl VariableSet for SelectionSortVariables {
    type Input = Vec<i32>;
    fn new(vec: Vec<i32>) -> Self {
        SelectionSortVariables {
            i: RwLock::new(Hardened::from(0)),
            j: RwLock::new(Hardened::from(0)),
            min: RwLock::new(Hardened::from(0)),
            N: RwLock::new(Hardened::from(0)),
            vec: RwLock::new(Hardened::from_vec(vec))
        }
    }
}

impl VariableSet for BubbleSortVariables {
    type Input = Vec<i32>;
    fn new(vet: Vec<i32>) -> Self {
        BubbleSortVariables {
            i: RwLock::new(Hardened::from(0)),
            j: RwLock::new(Hardened::from(0)),
            swapped: RwLock::new(Hardened::from(false)),
            N: RwLock::new(Hardened::from(0)),
            vet: RwLock::new(Hardened::from_vec(vet))
        }
    }
}

impl VariableSet for MatrixMultiplicationVariables {
    type Input = (Vec<Vec<i32>>, Vec<Vec<i32>>);
    fn new((a, b): (Vec<Vec<i32>>, Vec<Vec<i32>>)) -> Self {
        MatrixMultiplicationVariables {
            size: RwLock::new(Hardened::from(0)),
            i: RwLock::new(Hardened::from(0)),
            j: RwLock::new(Hardened::from(0)),
            k: RwLock::new(Hardened::from(0)),
            row: RwLock::new(Hardened::from_vec(Vec::new())),
            acc: RwLock::new(Hardened::from(0)),
            a: RwLock::new(Hardened::from_mat(Vec::new())),
            b: RwLock::new(Hardened::from_mat(Vec::new())),
            result: RwLock::new(Hardened::from_mat(Vec::new()))
        }
    }
}


impl AlgorithmVariables {
    fn from_target(target: &str, vec: Vec<i32>) -> Arc<AlgorithmVariables> {
        match target {
            "sel_sort" => Arc::new(AlgorithmVariables::SelectionSort(SelectionSortVariables::new(vec))),
            "bubble_sort" => Arc::new(AlgorithmVariables::BubbleSort(BubbleSortVariables::new(vec))),
            _ => panic!("Unknown target algorithm"),
        }
    }
}

fn runner(variables: Arc<AlgorithmVariables>, fault_list_entry: FaultListEntry, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> TestResult {

    let result = panic::catch_unwind(|| {
        match &*variables {
            AlgorithmVariables::SelectionSort(var) => {
                runner_selection_sort(var, tx_runner, rx_runner)
            }
            AlgorithmVariables::BubbleSort(var) => {
                runner_bubble_sort(var, tx_runner, rx_runner)
            }
            AlgorithmVariables::MatrixMultiplication(_) => {
                Ok(())
            }
        }

    });



    match result {
        Ok(Ok(())) => TestResult {result: Ok(()), fault_list_entry},
        Ok(Err(err)) => {
            println!("Error found - {}", err);
            TestResult {result: Err(err), fault_list_entry}
        },
        Err(_) => TestResult { result: Err(IncoherenceError::Generic), fault_list_entry }     //println!("runner_selection_sort panicked!")
    }
}



fn injector(variables: Arc<AlgorithmVariables>, fault_list_entry: FaultListEntry, tx_injector: Sender<&str>, rx_runner: Receiver<&str>) {

    let mut counter = 0usize;

    println!("error to inject: {:?}", fault_list_entry);

    // dato che fault_mask mi dice la posizione del bit da modificare, per ottenere la maschera devo calcolare 2^fault_mask
    let mut mask = 1 << (fault_list_entry.flipped_bit);

    //println!("mask: {}", 1 << (fault_list_entry.fault_mask));       // ottengo la maschera

    while let Ok(msg) = rx_runner.recv() {
        counter += 1;

        if counter == fault_list_entry.time {
            match &*variables {
                AlgorithmVariables::SelectionSort(var) => {
                    match fault_list_entry.var.as_str() {
                        "i" => {
                            let val = var.i.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.i.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "j" => {
                            let val = var.j.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.j.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "N" => {
                            let val = var.N.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.N.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "min" => {
                            let val = var.min.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                             // nuovo valore da salvare (XOR per il bitflip)
                            var.min.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        _ => {
                            let index = fault_list_entry.var
                                .split(|c| c == '[' || c == ']')
                                .collect::<Vec<_>>()[1]
                                .parse::<usize>().unwrap(); // ottengo l'indice dell'elemento nel vttore in cui iniettare l'errore

                            let val = var.vec.read().unwrap()[index].inner().unwrap().clone();
                            let new_val = val ^ (mask as i32);
                            var.vec.write().unwrap()[index]["cp1"] = new_val;
                        }
                    };
                }
                AlgorithmVariables::BubbleSort(var) => {
                    match fault_list_entry.var.as_str() {
                        "i" => {
                            let val = var.i.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.i.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "j" => {
                            let val = var.j.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.j.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "N" => {
                            let val = var.N.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.N.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "swapped" => {
                            let val = var.swapped.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = !val;                                             // nuovo valore da salvare (XOR per il bitflip)
                            var.swapped.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        _ => {
                            let index = fault_list_entry.var
                                .split(|c| c == '[' || c == ']')
                                .collect::<Vec<_>>()[1]
                                .parse::<usize>().unwrap(); // ottengo l'indice dell'elemento nel vttore in cui iniettare l'errore

                            let val = var.vet.read().unwrap()[index].inner().unwrap().clone();
                            let new_val = val ^ (mask as i32);
                            var.vet.write().unwrap()[index]["cp1"] = new_val;
                        }
                    };
                }
                AlgorithmVariables::MatrixMultiplication(_) => {}
            }
        }



        tx_injector.send("ricevuto").unwrap();
    }
}



pub fn injector_manager(rx_chan_fm_inj: Receiver<FaultListEntry>,
                        tx_chan_inj_anl: Sender<TestResult>,
                        target: String,
                        vec: Vec<i32>){            //per il momento lasciamolo, poi si vedrà...

    panic::set_hook(Box::new(|_panic_info| {        // SE NECESSARIO RIMUOVERE
        // Print a simple message when a panic occurs
        eprintln!("A panic occurred!");
    }));

    let mut handles_runner = vec![];
    let mut handles_injector = vec![];

    while let Ok(fault_list_entry) = rx_chan_fm_inj.recv(){

        let var = AlgorithmVariables::from_target(target.as_str(), vec.clone());

        // thread
        let (tx_1, rx_1) = channel();
        let (tx_2, rx_2) = channel();

        let shared_variables = var;

        let runner_variables = Arc::clone(&shared_variables);
        let injector_variables = Arc::clone(&shared_variables);

        let fault_list_entry_runner = fault_list_entry.clone();

        handles_runner.push(thread::spawn(move || runner(runner_variables, fault_list_entry_runner, tx_1, rx_2)));     // lancio il thread che esegue l'algoritmo
        handles_injector.push(thread::spawn(move || injector(injector_variables, fault_list_entry, tx_2, rx_1)));      // lancio il thread iniettore
        break;
    }



    for handle in handles_runner {
        let result = handle.join().unwrap();

        tx_chan_inj_anl.send(result).unwrap();
    }


    for handle in handles_injector {
        handle.join().unwrap();
    }


    drop(tx_chan_inj_anl);
}