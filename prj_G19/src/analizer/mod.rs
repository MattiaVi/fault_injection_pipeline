use std::sync::mpsc::{Receiver, Sender};
use crate::fault_list_manager::static_analysis::Variable;
use crate::hardened::IncoherenceError;
use crate::injector::TestResult;

#[derive(Debug)]
struct Analizer{
    n_silent_fault: usize,
    n_assign_fault: usize,
    n_mul_fault: usize,
    n_generic_fault: usize,
    n_add_fault: usize,
}

impl Analizer{
    fn new() -> Analizer {
        Analizer{
            n_silent_fault: 0,
            n_assign_fault: 0,
            n_mul_fault: 0,
            n_generic_fault: 0,
            n_add_fault: 0,
        }
    }
}

pub fn analizer(rx_chan_inj_anl: Receiver<TestResult>) {
    let mut vec_result = Vec::new();
    let mut analizer = Analizer::new();

    while let Ok(test_result) = rx_chan_inj_anl.recv() {
        vec_result.push(test_result);
    }

    for test_result in &vec_result {
        let res = test_result.get_result();

        if res.is_ok() {
            analizer.n_silent_fault += 1;
        } else {
            match res.err().unwrap() {
                IncoherenceError::AssignFail => analizer.n_assign_fault += 1,
                IncoherenceError::AddFail => analizer.n_add_fault += 1,
                IncoherenceError::MulFail => analizer.n_mul_fault += 1,
                IncoherenceError::Generic => analizer.n_generic_fault += 1
            }
        }
    }
    print!("Analizer: {:?}", analizer);
}



