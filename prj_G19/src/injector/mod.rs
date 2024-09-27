use std::sync::mpsc::{Receiver, Sender};
use crate::fault_list_manager::FaultListEntry;

//TODO
pub struct TestResult{
    field: i32
}

pub fn injector_manager(rx_chan_fm_inj: Receiver<FaultListEntry>,
                tx_chan_inj_anl: Sender<TestResult>,
                target: String){            //per il momento lasciamolo, poi si vedrà...

    while let Ok(a)=rx_chan_fm_inj.recv(){
            println!("Received {:?}",a);
    }
}