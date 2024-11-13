use std::sync::mpsc::{Receiver};
use crate::hardened::IncoherenceError;
use crate::injector::TestResult;
use crate::pdf_generator;
#[derive(Debug)]
pub struct Analyzer{
    pub(crate) n_silent_fault: usize,
    pub(crate) n_assign_fault: usize,
    pub(crate) n_mul_fault: usize,
    pub(crate) n_generic_fault: usize,
    pub(crate) n_add_fault: usize,
    pub(crate) total_fault: usize,
}

impl Analyzer{
    fn new() -> Analyzer {
        Analyzer {
            n_silent_fault: 0,
            n_assign_fault: 0,
            n_mul_fault: 0,
            n_generic_fault: 0,
            n_add_fault: 0,
            total_fault: 0,
        }
    }
}

pub fn analyzer(rx_chan_inj_anl: Receiver<TestResult>, file_path:String) {
    let mut vec_result = Vec::new();
    let mut analyzer = Analyzer::new();

    while let Ok(test_result) = rx_chan_inj_anl.recv() {
        vec_result.push(test_result);
    }

    for test_result in &vec_result {
        let res = test_result.get_result();

        if res.is_ok() {
            analyzer.n_silent_fault += 1;
        } else {
            match res.err().unwrap() {
                IncoherenceError::AssignFail => analyzer.n_assign_fault += 1,
                IncoherenceError::AddFail => analyzer.n_add_fault += 1,
                IncoherenceError::MulFail => analyzer.n_mul_fault += 1,
                IncoherenceError::Generic => analyzer.n_generic_fault += 1
            }
        }
    }
    analyzer.total_fault =  analyzer.n_silent_fault +
                            analyzer.n_assign_fault +
                            analyzer.n_add_fault +
                            analyzer.n_mul_fault +
                            analyzer.n_generic_fault;

    print!("Analyzer: {:?}", analyzer);
    pdf_generator::print_pdf(file_path,analyzer);
}


