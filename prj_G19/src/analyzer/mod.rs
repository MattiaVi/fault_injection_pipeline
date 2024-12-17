use std::fs;
use std::sync::mpsc::{Receiver};
use std::time::Instant;
use crate::fault_env::Data;
use crate::fault_list_manager::file_fault_list::{bubble_sort, matrix_multiplication, selection_sort};
use crate::hardened::{bubble_sort_hardened, matrix_multiplication_hardened, selection_sort_hardened,
                      Hardened, IncoherenceError};
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

pub fn analyzer(rx_chan_inj_anl: Receiver<TestResult>, file_path:String, data: Data<i32>, target:String) {
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
    
    let dim = get_data_for_dimension_table(0);
    let time = get_data_for_time_table(0, data);

    pdf_generator::print_pdf(file_path,analyzer);
}
fn get_data_for_dimension_table(i:u8) -> Result<Vec<f64>,String>{
    let mut dimensions:Vec<f64> = Vec::new();
    let file_path_nothardened = match i {
        0 => "src/fault_list_manager/file_fault_list/selection_sort/mod",
        1 => "src/fault_list_manager/file_fault_list/bubble_sort/mod",
        2 => "src/fault_list_manager/file_fault_list/matrix_multiplication/mod",
        _ => "",
    };
    let metadata_not_hard = fs::metadata(file_path_nothardened);

    let file_path_hardened = match i {
        0 => "src/hardened/selection_sort_hardened/mod.rs",
        1 => "src/hardened/bubble_sort_hardened/mod.rs",
        2 => "src/hardened/matrix_multiplication_hardened/mod.rs",
        _ => "",
    };
    let metadata_hard = fs::metadata(file_path_hardened);
    if metadata_not_hard.is_ok() && metadata_hard.is_ok() {
        let dim_not_hard = metadata_not_hard.unwrap().len() as f64;
        dimensions.push(dim_not_hard);
        let dim_hard = metadata_hard.unwrap().len() as f64;
        dimensions.push(dim_hard);
        let div_res =  f64::trunc((dim_hard/dim_not_hard)*100.0)/100.0;
        dimensions.push(div_res);
    }else{
        return Err(format!("il path del file: {} non è valido",file_path_nothardened));
    }
    Ok(dimensions)
}

fn get_data_for_time_table(i:u8, data:Data<i32>) -> Result<Vec<f64>,String>{
    let mut times:Vec<f64> = Vec::new();
    let mut data_hard= data.clone();
    println!("data: {:?}",data.clone().into_Vector());
    println!("data hard: {:?}",data.clone().into_Vector());
    let elapsed_time_not_hard= match i {
        0 => {
            let start_sel_sort = Instant::now();
            selection_sort::selection_sort(data.into_Vector());
            start_sel_sort.elapsed().as_nanos() as f64
        },
        1 => {
            let start_bb_sort = Instant::now();
            bubble_sort::bubble_sort(data.into_Vector());
            start_bb_sort.elapsed().as_nanos() as f64
        },
        2 => {
            let start_mat_multiplication =  Instant::now();
            let matrices=  data.into_Matrices();
            matrix_multiplication::matrix_multiplication(matrices.0,matrices.1);
            start_mat_multiplication.elapsed().as_nanos() as f64
        },
        _ => return Err("Indice non valido".to_string()),
    };
    times.push(elapsed_time_not_hard);

    println!("data hard: {:?}",data_hard.clone().into_Vector());
    let elapsed_time_hard= match i {
        0 => {
            let start_sel_sort = Instant::now();
            selection_sort_hardened::selection_sort(&mut Hardened::from_vec(data_hard.into_Vector())).unwrap();
            start_sel_sort.elapsed().as_nanos() as f64
        },
        1 => {
            let start_bb_sort = Instant::now();
            bubble_sort_hardened::bubble_sort(&mut Hardened::from_vec(data_hard.into_Vector())).unwrap();
            start_bb_sort.elapsed().as_nanos() as f64
        },
        2 => {
            let start_mat_multiplication =  Instant::now();
            let matrices=  data_hard.into_Matrices();
            matrix_multiplication_hardened::matrix_multiplication(&mut Hardened::from_mat(matrices.0),&mut Hardened::from_mat(matrices.1)).unwrap();
            start_mat_multiplication.elapsed().as_nanos() as f64
        },
        _ => return Err("Indice non valido".to_string()),
    };
    times.push(elapsed_time_hard);
    times.push(f64::trunc((elapsed_time_hard/elapsed_time_not_hard)*100.0)/100.0);
    Ok(times)
}
#[cfg(test)]
mod tests{
    use rand::Rng;
    use crate::analyzer::{get_data_for_dimension_table, get_data_for_time_table};
    use crate::fault_env::Data;
        #[test]
    fn try_get_execution_times(){

        let mut rng = rand::thread_rng();
        let vec: Vec<i32> = (0..3000).map(|_| rng.gen_range(0..20)).collect();
        println!("{:?}", vec);
        let tim = get_data_for_time_table(0,Data::Vector(vec));
        if tim.is_ok(){
            let times = tim.unwrap();
            println!("{:?}",times);
            assert!(times[0] >= 0.0 && times[1] >= 0.0);
        }else{
            println!("{}",tim.unwrap_err());
        }
    }
    fn try_get_files_dimensions(){
        let dim = get_data_for_dimension_table(0);
        if dim.is_ok(){
            let dimensions = dim.unwrap();
            println!("{:?}",dimensions);
            assert!(dimensions[0] >= 0.0 && dimensions[1] >= 0.0);
        }else{
            println!("{}",dim.unwrap_err());
        }
    }
}

