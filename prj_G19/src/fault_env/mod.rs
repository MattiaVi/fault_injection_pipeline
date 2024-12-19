use std::sync::mpsc::channel;
use std::time::Instant;
use crate::analyzer::analyzer;
use crate::fault_list_manager::fault_manager;
use crate::injector::injector_manager;

//Al fine di generalizzare passo dei dati anziché un vec specifico
#[derive(Clone)]
pub enum Data<T>{
    Vector(Vec<T>),
    Matrices(Vec<Vec<T>>, Vec<Vec<T>>)
}

impl<T> Data<T>{
    pub fn into_vector(self) ->Vec<T>{
        match self{
            Data::Vector(ris) =>{
                ris
            },
            _ => {
                panic!("Not a vector!");
            }
        }
    }

    pub fn into_matrices(self) ->(Vec<Vec<T>>, Vec<Vec<T>>){
        match self{
            Data::Matrices(a,b)=>{
                (a,b)
            },
            _=>{
                panic!("Not a matrices variant");
            }

        }
    }
}

pub fn fault_injection_env(fault_list: String,      // nome file fault-list
                           target: String,          // nome programma target
                           file_path: String,       // nome file report
                           data: Data<i32>,
                           timer:Instant,
                           esecuzione:i8)
{       // dati del problema

    let (tx_chan_fm_inj, rx_chan_fm_inj) = channel();
    let (tx_chan_inj_anl, rx_chan_inj_anl) = channel();
    fault_manager(tx_chan_fm_inj,fault_list);
    injector_manager(rx_chan_fm_inj, tx_chan_inj_anl, target.clone(), data.clone());
    let execution_time = timer.elapsed().as_millis()as f64;
    analyzer(rx_chan_inj_anl,file_path,data,target,esecuzione,execution_time);
}

/*
#[cfg(test)]
mod tests{
use crate::fault_env::Data;

#[test]
fn try_build_Matrices(){
    //Creo la struttura dati contenente le matrici
    let data = Data::Matrices(vec![1,2], vec![1,2]);

    //ricavo le matrici dal tipo enumerativo
    let (mat1,mat2) = data.into_matrices();

    //faccio qualcosa
    assert_eq!(mat1[1],2);
    assert_eq!(mat2[0],1);
}
}
*/