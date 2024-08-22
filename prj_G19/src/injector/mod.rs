use std::sync::mpsc::{Receiver, Sender};
use crate::fault_list_manager::FaultListEntry;

//TODO
pub struct TestResult{
    field: i32
}

pub fn injector(rx_chan_fm_inj: Receiver<FaultListEntry>,
                tx_chan_inj_anl: Sender<TestResult>,
                target: String){            //per il momento lasciamolo, poi si vedrà...
    assert_eq!("", "");
}