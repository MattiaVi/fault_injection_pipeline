mod hardened;
mod fault_list_manager;

use hardened::{Hardened, IncoherenceError};
use fault_list_manager::{FaultListEntry, static_analysis};
use std::fs;
use syn::{File, Item};
use syn::visit::Visit;

/// <h3>Caso di studio 1: Selection Sort</h3>
fn selection_sort(vet: &mut Vec<Hardened<i32>>)->Result<(), IncoherenceError>{
    let mut N:Hardened<usize> = vet.len().into();
    let mut j= Hardened::from(0);
    let mut min = Hardened::from(0);
    //--------------SELECTION SORT-------------------------
    let mut i= Hardened::from(0);
    while i<(N-1)?{
        min.assign(i)?;                 //min=i
        j.assign((i+1)?)?;        //j=0
        //Ricerca del minimo
        while j<N{
            if vet[j]<vet[min]  {   min.assign(j)?; }
            j.assign((j+1)?)?;
        }
        //Scambio il minimo
        vet.swap(i.inner()?, min.inner()?);
        //Vado avanti
        i.assign((i+1)?)?;
    }
    //------------------------------------------------------
    Ok(())
}

fn main(){
    /*
    let mut myvec =
        Hardened::from_vec(vec![34, 12, 54, 1, 10, 21, 19, 2, 3, 24, 9]);

    println!("Vettore prima: {:?}", myvec);
    match selection_sort(&mut myvec){
        Ok(a)=>{
            println!("Vettore dopo: {:?}", myvec);
        }
        Err(e)=>{
            println!("Errore: {}", e);
        }
    }

    let a = Hardened::from(3);
    println!("Uso Debug: {:?}",a);
     */

    let file_path = "src/fault_list_manager/\
                                    file_fault_list\
                                    /selection_sort.rs";
    let code = fs::read_to_string(file_path)
        .expect("Failed to read file");

    let file: File = syn::parse_str(&code).expect("Failed to parse code");

    for item in file.items {
        if let syn::Item::Fn(func) = item {
            static_analysis::analyze_function(&func);
        }
    }
}