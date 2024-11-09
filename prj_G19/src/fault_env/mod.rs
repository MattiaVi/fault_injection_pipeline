use std::sync::mpsc::channel;
use crate::analizer::analizer;
use crate::fault_list_manager::{DimData, fault_manager};
use crate::injector::injector_manager;

//Al fine di generalizzare passo dei dati anziché un vec specifico
#[derive(Clone)]
pub enum Data<T>{
    Vector(Vec<T>),
    Matrices(Vec<Vec<T>>, Vec<Vec<T>>)
}

impl<T> Data<T>{
    pub fn into_Vector(self)->Vec<T>{
        match self{
            Data::Vector(ris) =>{
                ris
            },
            _ => {
                panic!("Not a vector!");
            }
        }
    }

    pub fn into_Matrices(self)->(Vec<Vec<T>>, Vec<Vec<T>>){
        match self{
            Data::Matrices(a,b)=>{
                (a,b)
            },
            _=>{
                panic!("Not a matrices variant");
            }

        }
    }

    pub fn get_dim(self)->DimData{
        match self{
            Data::Vector(a)=>{
                DimData::Vector(a.len())
            },
            Data::Matrices(a,_)=>{
                DimData::Matrix((a.len(), a[0].len()))
            }
        }
    }
}

pub fn fault_injection_env(fault_list: String,      // nome file fault-list
                           target: String,          // nome programma target
                           report_name: String,     // nome file report
                           data: Data<i32>) {         // vettore in analisi

    let (tx_chan_fm_inj, rx_chan_fm_inj) = channel();
    let (tx_chan_inj_anl, rx_chan_inj_anl) = channel();

    //Questi possono essere a loro volta wrapper che faranno delle cose
    fault_manager(tx_chan_fm_inj,fault_list);
    injector_manager(rx_chan_fm_inj, tx_chan_inj_anl, target, data.into_Vector());
    analizer(rx_chan_inj_anl);
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
    let (mat1,mat2) = data.into_Matrices();

    //faccio qualcosa
    assert_eq!(mat1[1],2);
    assert_eq!(mat2[0],1);
}
}
*/