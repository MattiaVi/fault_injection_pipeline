use core::mem::size_of;
use std::sync::mpsc::Sender;
use std::fs;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, format};
use std::fs::OpenOptions;
use std::io::Write;
use serde_json;
use crate::static_analysis::{ResultAnalysis,Variable};
use rand::prelude::*;


pub mod static_analysis;

///Generazione della fault list:
///     - generazione casuale di un certo numero di entry +
///
/// path_raw_info
pub fn create_fault_list(path_raw_info: String, N: usize, file_path_dest: String)
    ->Vec<FaultListEntry>{
    //RETRIEVING INFORMAZIONI GREZZE
    //Prendere il contenuto del file come stringa
    let raw_info = fs::read_to_string(path_raw_info).unwrap();
    //Unmarshaling (Stringa JSON --> Struttura Dati)
    let info:ResultAnalysis =serde_json::from_str(&raw_info).unwrap();

    //-----------------------Per Debug--------------------------
    println!("Numero istruzioni: {}", info.num_inst);

    let vars:Vec<Variable> = info.vars;
    let num_vars=vars.len();
    //----------------------------------------------------------

    //--------------------------GENERAZIONE DELLA FAULT LIST-----------------------------
    let NUM_FAULTS=200;     //TODO: stabilire quanti!
    let mut fault_list:Vec<FaultListEntry> = Vec::new();
    //Ingrediente fondamentale: Generazione di numeri casuali
    //Fonte utile:
    //https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-values

    let mut rnd=rand::thread_rng();

    for i in 0..NUM_FAULTS{
        let what_var=rnd.gen_range(0..num_vars);
        //Caso 'vettore'
        if vars[what_var].ty==String::from("Vec < i32 >"){
            //Quale variabile del vettore voglio iniettare?
            let what_el = rnd.gen_range(0..N);
            let it=FaultListEntry{
                var: format!("{}[{}]", vars[what_var].name, what_el),
                time: rnd.gen_range(vars[what_var].start..info.num_inst),
                //TODO: va bene memorizzare solo il numero o dobbiamo metterci tutta la maschera?
                fault_mask: rnd.gen_range(0..size_of::<i32>() as u64),
            };
            fault_list.push(it);
        }
        //Caso 'non vettore'
        else {
            let it = FaultListEntry {
                var: vars[what_var].name.clone(),
                time: rnd.gen_range(vars[what_var].start..info.num_inst),
                fault_mask: rnd.gen_range(0..vars[what_var].size
                    .parse::<usize>()
                    .unwrap() as u64*8),
            };
            fault_list.push(it);
        }
    }

    //DEBUG
    //fault_list.iter().for_each(|el|println!("{:?}", el));

    //SERIALIZZAZIONE (MARSHALLING) della fault list

    //TODO: risolvere problema di errore su file
    let mut fl= OpenOptions::new()
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open(file_path_dest)
        .unwrap();
    let ris_JSON = serde_json::to_string_pretty(&fault_list).unwrap();
    fl.write_all(ris_JSON.as_bytes()).unwrap();

    return fault_list;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaultListEntry{
    var: String,
    time: usize,
    fault_mask: u64,
}

impl FaultListEntry{
    fn get_var(&self)->&str{
        &self.var
    }
    fn get_time(&self)->usize{
        self.time
    }
    fn get_fault_mask(&self)->u64{
        self.fault_mask
    }
}

//Fault List Manager (Stage pipeline)

//Stage della pipeline: Fault List Manager
pub fn fault_manager(tx_chan_fm_inj: Sender<FaultListEntry>, fault_list:String){
    //Deserializzare (unmarshalling)) della fault list
    let flist_string = fs::read_to_string(fault_list).unwrap();
    let flist:Vec<FaultListEntry>=serde_json::from_str(&flist_string.trim()).unwrap();
    flist.into_iter().for_each(|el|tx_chan_fm_inj.send(el).unwrap());
    drop(tx_chan_fm_inj);
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_trivial(){
        assert_eq!(2,2);
    }
}

/***    PUNTI SALIENTI REALIZZAZIONE PARTE CARLO
    1. ANALISI STATICA DEL CODICE (fn generate_analysis_file()) --> questa parte prende in input
     i file creati da Mattia (Prima parte), produce in output un file json con le informazioni su
      numero di istruzioni della funzione e per ogni variabile, nome, tipo e dimensione
    2. GENERAZIONE DELLA FAULT LIST (fn create_fault_list()) --> prende in input il file JSON
    discusso al punto precedente e ne produce un altro contenente una lista di entry {nome
    variabile, tempo iniezione, fault_mask} (Questo file è frutto della serializzazione di una
    collezione di elementi di tipo 'Fault List Entry'
    3. REALIZZAZIONE PRIMO STAGE PIPELINE, in questa fase si deserializza il file JSON risultato
    del punto precedente e, elemento per elemento, lo si manda nel canale allo stage successivo
    (Iniettore <-> Alex)
 */


