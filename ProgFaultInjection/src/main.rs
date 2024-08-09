mod hardened;
use crate::hardened::hardened::{Hardened,IncoherenceError};



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
}