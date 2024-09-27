use std::sync::mpsc::channel;
use crate::analizer::analizer;
use crate::fault_list_manager::fault_manager;
use crate::injector::injector_manager;

pub fn fault_injection_env(fault_list: String,      //nome file fault-list
                       target: String,          //nome programma target
                       report_name: String) {   //nome file report
    let (tx_chan_fm_inj, rx_chan_fm_inj) = channel();
    let (tx_chan_inj_anl, rx_chan_inj_anl) = channel();

    //Questi possono essere a loro volta wrapper che faranno delle cose
    fault_manager(tx_chan_fm_inj,fault_list);
    injector_manager(rx_chan_fm_inj, tx_chan_inj_anl, target);
    analizer(rx_chan_inj_anl);
}