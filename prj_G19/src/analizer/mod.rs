use std::sync::mpsc::{Receiver, Sender};
use crate::injector::TestResult;

pub fn analizer(rx_chan_inj_anl: Receiver<TestResult>){
    assert_eq!("", "");
}