use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{panic, thread};
use crate::fault_env::Data;
use crate::fault_list_manager::FaultListEntry;
use crate::hardened::{Hardened, IncoherenceError};

//TODO
pub struct TestResult {
    field: i32
}

struct Variables {
    i: RwLock<Hardened<usize>>,
    j: RwLock<Hardened<usize>>,
    N: RwLock<Hardened<usize>>,
    min: RwLock<Hardened<usize>>,
    vec: RwLock<Vec<Hardened<i32>>>
}

impl Variables {
    fn new(vec: Vec<i32>) -> Self {
        Variables {
            i: RwLock::new(Hardened::from(0)),
            j: RwLock::new(Hardened::from(0)),
            min: RwLock::new(Hardened::from(0)),
            N: RwLock::new(Hardened::from(0)),
            vec: RwLock::new(Hardened::from_vec(vec))
        }
    }
}


fn runner_selection_sort(variables: Arc<Variables>, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> Result<(), IncoherenceError> {

    *variables.N.write().unwrap() = variables.vec.read().unwrap().len().into();
    tx_runner.send("i1");
    rx_runner.recv();

    *variables.j.write().unwrap() = Hardened::from(0);
    tx_runner.send("i2");
    rx_runner.recv();

    *variables.min.write().unwrap() = Hardened::from(10);
    tx_runner.send("i3");
    rx_runner.recv();

    *variables.i.write().unwrap() = Hardened::from(0);
    tx_runner.send("i4");
    rx_runner.recv();


    while *variables.i.read().unwrap() < (*variables.N.read().unwrap() - 1)? {
        tx_runner.send("i5");
        rx_runner.recv();

        variables.min.write().unwrap().assign(*variables.i.read().unwrap())?;
        tx_runner.send("i6");
        rx_runner.recv();

        variables.j.write().unwrap().assign((*variables.i.read().unwrap() + 1)?)?;
        tx_runner.send("i7");
        rx_runner.recv();


        while *variables.j.read().unwrap() < *variables.N.read().unwrap() {
            tx_runner.send("i8");
            rx_runner.recv();

            if variables.vec.read().unwrap()[*variables.j.read().unwrap()] < variables.vec.read().unwrap()[*variables.min.read().unwrap()] {
                tx_runner.send("i9");
                rx_runner.recv();

                variables.min.write().unwrap().assign(*variables.j.read().unwrap())?;
                tx_runner.send("i10");
                rx_runner.recv();
            }

            let tmp = (*variables.j.read().unwrap() + 1)?;  // necessario dato che non potrei fare j = j + 1, dato che dovrei acquisire un lock in lettura dopo averlo gia' acquisito sulla stessa variabile in scrittura
            variables.j.write().unwrap().assign(tmp)?;
            tx_runner.send("i11");
            rx_runner.recv();
        }

        variables.vec.write().unwrap().swap(variables.i.read().unwrap().inner()?, variables.min.read().unwrap().inner()?);
        tx_runner.send("i12");
        rx_runner.recv();

        let tmp = (*variables.i.read().unwrap() + 1)?;
        variables.i.write().unwrap().assign(tmp)?;
        tx_runner.send("i13");
        rx_runner.recv();
    }


    /*
    let mut N:Hardened<usize> = vet.len().into();
    let mut j= Hardened::from(0);
    let mut min = Hardened::from(0);
    //--------------SELECTION SORT-------------------------
    let mut i= Hardened::from(0);
    while i<(N-1)?{
        min.assign(i)?;                 //min=i
        j.assign((i+1)?)?;        //j=0
        //Ricerca del minimo
        while j<N{
            if vet[j]<vet[min]  {   min.assign(j)?; }
            j.assign((j+1)?)?;
        }
        //Scambio il minimo
        vet.swap(i.inner()?, min.inner()?);
        //Vado avanti
        i.assign((i+1)?)?;
    }
     */
    //------------------------------------------------------

    Ok(())
}


fn runner(variables: Arc<Variables>, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) {

    let result = panic::catch_unwind(|| {
        runner_selection_sort(variables, tx_runner, rx_runner)
    });

    match result {
        Ok(Ok(())) => (),
        Ok(Err(err)) => println!("Error found - {}", err),
        Err(_) => ()     //println!("runner_selection_sort panicked!")
    }

    //println!("Posso fare altre cose");
}



fn injector(variables: Arc<Variables>, fault_list_entry: FaultListEntry, tx_injector: Sender<&str>, rx_runner: Receiver<&str>) {

    let mut counter = 0usize;

    println!("error to inject: {:?}", fault_list_entry);

    // dato che fault_mask mi dice la posizione del bit da modificare, per ottenere la maschera devo calcolare 2^fault_mask
    let mut mask = 1 << (fault_list_entry.fault_mask);

    //println!("mask: {}", 1 << (fault_list_entry.fault_mask));       // ottengo la maschera

    while let Ok(msg) = rx_runner.recv() {
        counter += 1;

        if counter == fault_list_entry.time {
            match fault_list_entry.var.as_str() {
                "i" => {
                    let val = variables.i.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                    let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                    variables.i.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                },
                "j" => {
                    let val = variables.j.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                    let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                    variables.j.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                },
                "N" => {
                    let val = variables.N.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                    let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                    variables.N.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                },
                "min" => {
                    let val = variables.min.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                    let new_val = val ^ mask;                                             // nuovo valore da salvare (XOR per il bitflip)
                    variables.min.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                },
                _ => {
                    let index = fault_list_entry.var
                        .split(|c| c == '[' || c == ']')
                        .collect::<Vec<_>>()[1]
                        .parse::<usize>().unwrap(); // ottengo l'indice dell'elemento nel vttore in cui iniettare l'errore

                    let val = variables.vec.read().unwrap()[index].inner().unwrap().clone();
                    let new_val = val ^ (mask as i32);
                    let prova = variables.vec.write().unwrap()[index]["cp1"] = new_val;
                }
            };
        }

        tx_injector.send("ricevuto");
    }
}



pub fn injector_manager(rx_chan_fm_inj: Receiver<FaultListEntry>,
                tx_chan_inj_anl: Sender<TestResult>,
                target: String,
                vec: Vec<i32>){            //per il momento lasciamolo, poi si vedrà...

    let mut handles = vec![];


    while let Ok(fault_list_entry) = rx_chan_fm_inj.recv(){

        let variables = Variables::new(vec.clone());    // creo il set di variabili usate dai
        // thread
        let (tx_1, rx_1) = channel();
        let (tx_2, rx_2) = channel();


        let shared_variables = Arc::new(variables);

        let runner_variables = Arc::clone(&shared_variables);
        let injector_variables = Arc::clone(&shared_variables);

        handles.push(thread::spawn(move || runner(runner_variables, tx_1, rx_2)));     // lancio il thread che esegue l'algoritmo
        handles.push(thread::spawn(move || injector(injector_variables, fault_list_entry, tx_2, rx_1)));      // lancio il thread iniettore
    }



    for handle in handles {
        handle.join().unwrap();
        //TODO!
    }

    // invio dei risultati
}