use std::sync::mpsc::{Receiver};
use crate::hardened::IncoherenceError;
use crate::injector::TestResult;
use crate::pdf_generator;
#[derive(Debug,Clone)]
pub struct Analyzer{
    pub(crate) n_silent_fail: usize,
    pub(crate) n_assign_fail: usize,
    pub(crate) n_mul_fail: usize,
    pub(crate) n_generic_fail: usize,
    pub(crate) n_add_fail: usize,
    pub(crate) n_indexmut_fail: usize,
    pub(crate) n_index_fail: usize,
    pub(crate) n_ord_fail: usize,
    pub(crate) n_partialord_fail: usize,
    pub(crate) n_partialeq_fail: usize,
    pub(crate) total_fault: usize,
}

impl Analyzer{
    fn new() -> Analyzer {
        Analyzer {
            n_silent_fail: 0,
            n_assign_fail: 0,
            n_mul_fail: 0,
            n_generic_fail: 0,
            n_add_fail: 0,
            n_indexmut_fail: 0,
            n_index_fail: 0,
            n_ord_fail: 0,
            n_partialord_fail: 0,
            n_partialeq_fail: 0,
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
            analyzer.n_silent_fail += 1;
        } else {
            match res.err().unwrap() {
                IncoherenceError::AssignFail => analyzer.n_assign_fail += 1,
                IncoherenceError::AddFail => analyzer.n_add_fail += 1,
                IncoherenceError::MulFail => analyzer.n_mul_fail += 1,
                IncoherenceError::Generic => analyzer.n_generic_fail += 1,
                IncoherenceError::IndexMutFail => analyzer.n_indexmut_fail += 1,
                IncoherenceError::IndexFail => analyzer.n_index_fail += 1,
                IncoherenceError::OrdFail => analyzer.n_ord_fail += 1,
                IncoherenceError::PartialOrdFail => analyzer.n_partialord_fail += 1,
                IncoherenceError::PartialEqFail => analyzer.n_partialeq_fail += 1
            }
        }
    }
    analyzer.total_fault =  analyzer.n_silent_fail + analyzer.n_assign_fail + analyzer.n_add_fail +
                            analyzer.n_mul_fail + analyzer.n_generic_fail + analyzer.n_indexmut_fail +
                            analyzer.n_index_fail + analyzer.n_ord_fail + analyzer.n_partialord_fail +
                            analyzer.n_partialeq_fail;

    print!("Analyzer: {:?}", analyzer);
    pdf_generator::print_pdf(file_path,analyzer);
}


