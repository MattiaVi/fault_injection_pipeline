use std::sync::mpsc::{Receiver, Sender};
use crate::hardened::{Hardened, IncoherenceError};
use crate::injector::{BubbleSortVariables, SelectionSortVariables};

pub fn runner_selection_sort(variables: &SelectionSortVariables, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> Result<(), IncoherenceError> {

    *variables.N.write().unwrap() = variables.vec.read().unwrap().len().into();
    tx_runner.send("i1").unwrap();
    rx_runner.recv().unwrap();

    *variables.j.write().unwrap() = Hardened::from(0);
    tx_runner.send("i2").unwrap();
    rx_runner.recv().unwrap();

    *variables.min.write().unwrap() = Hardened::from(10);
    tx_runner.send("i3").unwrap();
    rx_runner.recv().unwrap();

    *variables.i.write().unwrap() = Hardened::from(0);
    tx_runner.send("i4").unwrap();
    rx_runner.recv().unwrap();

    while *variables.i.read().unwrap() < (*variables.N.read().unwrap() - 1)? {
        tx_runner.send("i5").unwrap();
        rx_runner.recv().unwrap();

        variables.min.write().unwrap().assign(*variables.i.read().unwrap())?;
        tx_runner.send("i6").unwrap();
        rx_runner.recv().unwrap();

        variables.j.write().unwrap().assign((*variables.i.read().unwrap() + 1)?)?;
        tx_runner.send("i7").unwrap();
        rx_runner.recv().unwrap();

        while *variables.j.read().unwrap() < *variables.N.read().unwrap() {
            tx_runner.send("i8").unwrap();
            rx_runner.recv().unwrap();

            if variables.vec.read().unwrap()[*variables.j.read().unwrap()] < variables.vec.read().unwrap()[*variables.min.read().unwrap()] {
                tx_runner.send("i9").unwrap();
                rx_runner.recv().unwrap();

                variables.min.write().unwrap().assign(*variables.j.read().unwrap())?;
                tx_runner.send("i10").unwrap();
                rx_runner.recv().unwrap();
            }

            let tmp = (*variables.j.read().unwrap() + 1)?;  // necessario dato che non potrei fare j = j + 1, dato che dovrei acquisire un lock in lettura dopo averlo gia' acquisito sulla stessa variabile in scrittura
            variables.j.write().unwrap().assign(tmp)?;
            tx_runner.send("i11").unwrap();
            rx_runner.recv().unwrap();
        }

        variables.vec.write().unwrap().swap(variables.i.read().unwrap().inner()?, variables.min.read().unwrap().inner()?);
        tx_runner.send("i12").unwrap();
        rx_runner.recv().unwrap();

        let tmp = (*variables.i.read().unwrap() + 1)?;
        variables.i.write().unwrap().assign(tmp)?;
        tx_runner.send("i13").unwrap();
        rx_runner.recv().unwrap();
    }


    /*
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
     */
    //------------------------------------------------------

    Ok(())
}


pub fn runner_bubble_sort(variables: &BubbleSortVariables, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> Result<(), IncoherenceError> {

    *variables.N.write().unwrap() = Hardened::from(variables.vet.read().unwrap().len());
    tx_runner.send("i1").unwrap();
    rx_runner.recv().unwrap();

    *variables.i.write().unwrap() = Hardened::from(0);
    tx_runner.send("i2").unwrap();
    rx_runner.recv().unwrap();

    while *variables.i.read().unwrap() < *variables.N.read().unwrap() {
        tx_runner.send("i3").unwrap();
        rx_runner.recv().unwrap();

        *variables.swapped.write().unwrap() = Hardened::from(false);
        tx_runner.send("i4").unwrap();
        rx_runner.recv().unwrap();

        *variables.j.write().unwrap() = Hardened::from(0);
        tx_runner.send("i5").unwrap();
        rx_runner.recv().unwrap();

        while *variables.j.read().unwrap() < ((*variables.N.read().unwrap() - *variables.i.read().unwrap())? - 1)? {
            tx_runner.send("i6").unwrap();
            rx_runner.recv().unwrap();

            if variables.vet.read().unwrap()[*variables.j.read().unwrap()].inner()? > variables.vet.read().unwrap()[(*variables.j.read().unwrap() + 1)?].inner()? {
                tx_runner.send("i7").unwrap();
                rx_runner.recv().unwrap();

                variables.vet.write().unwrap().swap(variables.j.read().unwrap().inner()?, (*variables.j.read().unwrap() + 1)?.inner()?);
                tx_runner.send("i8").unwrap();
                rx_runner.recv().unwrap();

                *variables.swapped.write().unwrap() = Hardened::from(true);
                tx_runner.send("i9").unwrap();
                rx_runner.recv().unwrap();

            }
            let tmp = (*variables.j.read().unwrap() + 1)?;
            variables.j.write().unwrap().assign(tmp)?;
            tx_runner.send("i10").unwrap();
            rx_runner.recv().unwrap();

        }

        if !variables.swapped.read().unwrap().inner()? {
            tx_runner.send("i11").unwrap();
            rx_runner.recv().unwrap();
            break;
        }

        let tmp = (*variables.i.read().unwrap() + 1)?;
        variables.i.write().unwrap().assign(tmp)?;
        tx_runner.send("i12").unwrap();
        rx_runner.recv().unwrap();
    }

    Ok(())

    /*
    let n = Hardened::from(vet.len());
    let mut i = Hardened::from(0);

    while i < n {
        let mut swapped = Hardened::from(false);
        let mut j = Hardened::from(0);

        while j < ((n - i)? - 1)? {
            if vet[j].inner()? > vet[(j + 1)?].inner()? {
                vet.swap(j.inner()?, (j + 1)?.inner()?);
                swapped = Hardened::from(true);
            }
            j.assign((j + 1)?)?;
        }
        if !swapped.inner()? {
            break;
        }
        i.assign((i + 1)?)?;
    }
    Ok(())
    */
}