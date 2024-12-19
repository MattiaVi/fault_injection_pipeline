use std::sync::mpsc::{Receiver, Sender};
use crate::hardened::{Hardened, IncoherenceError};
use crate::injector::{BubbleSortVariables, MatrixMultiplicationVariables, SelectionSortVariables};

pub fn runner_selection_sort(variables: &SelectionSortVariables, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> Result<(), IncoherenceError> {

    *variables.n.write().unwrap() = variables.vec.read().unwrap().len().into();
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

    while *variables.i.read().unwrap() < (*variables.n.read().unwrap() - 1)? {
        tx_runner.send("i5").unwrap();
        rx_runner.recv().unwrap();

        variables.min.write().unwrap().assign(*variables.i.read().unwrap())?;
        tx_runner.send("i6").unwrap();
        rx_runner.recv().unwrap();

        variables.j.write().unwrap().assign((*variables.i.read().unwrap() + 1)?)?;
        tx_runner.send("i7").unwrap();
        rx_runner.recv().unwrap();

        while *variables.j.read().unwrap() < *variables.n.read().unwrap() {
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
    let mut n:Hardened<usize> = vet.len().into_data();
    let mut j= Hardened::from(0);
    let mut min = Hardened::from(0);
    //--------------SELECTION SORT-------------------------
    let mut i= Hardened::from(0);
    while i<(n-1)?{
        min.assign(i)?;                 //min=i
        j.assign((i+1)?)?;        //j=0
        //Ricerca del minimo
        while j<n{
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

    *variables.n.write().unwrap() = Hardened::from(variables.vet.read().unwrap().len());
    tx_runner.send("i1").unwrap();
    rx_runner.recv().unwrap();

    *variables.i.write().unwrap() = Hardened::from(0);
    tx_runner.send("i2").unwrap();
    rx_runner.recv().unwrap();

    while *variables.i.read().unwrap() < *variables.n.read().unwrap() {
        tx_runner.send("i3").unwrap();
        rx_runner.recv().unwrap();

        *variables.swapped.write().unwrap() = Hardened::from(false);
        tx_runner.send("i4").unwrap();
        rx_runner.recv().unwrap();

        *variables.j.write().unwrap() = Hardened::from(0);
        tx_runner.send("i5").unwrap();
        rx_runner.recv().unwrap();

        while *variables.j.read().unwrap() < ((*variables.n.read().unwrap() - *variables.i.read().unwrap())? - 1)? {
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

pub fn runner_matrix_multiplication(variables: &MatrixMultiplicationVariables, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> Result<(), IncoherenceError> {

    *variables.size.write().unwrap() = Hardened::from(variables.a.read().unwrap().len());
    tx_runner.send("i1").unwrap();
    rx_runner.recv().unwrap();

    *variables.result.write().unwrap() = Hardened::from_mat(vec![vec![0; variables.size.read().unwrap().inner().unwrap()]; variables.size.read().unwrap().inner().unwrap()]);
    tx_runner.send("i2").unwrap();
    rx_runner.recv().unwrap();

    *variables.i.write().unwrap() = Hardened::from(0);
    tx_runner.send("i3").unwrap();
    rx_runner.recv().unwrap();

    *variables.j.write().unwrap() = Hardened::from(0);
    tx_runner.send("i4").unwrap();
    rx_runner.recv().unwrap();

    *variables.k.write().unwrap() = Hardened::from(0);
    tx_runner.send("i5").unwrap();
    rx_runner.recv().unwrap();

    while *variables.i.read().unwrap() < *variables.size.read().unwrap() {
        tx_runner.send("i6").unwrap();
        rx_runner.recv().unwrap();

        *variables.row.write().unwrap() = Hardened::from_vec(vec![0; variables.size.read().unwrap().inner().unwrap()]);
        tx_runner.send("i7").unwrap();
        rx_runner.recv().unwrap();

        variables.j.write().unwrap().assign(Hardened::from(0))?;
        tx_runner.send("i8").unwrap();
        rx_runner.recv().unwrap();

        while *variables.j.read().unwrap() < *variables.size.read().unwrap() {
            tx_runner.send("i9").unwrap();
            rx_runner.recv().unwrap();

            *variables.acc.write().unwrap() = Hardened::from(0);
            tx_runner.send("i10").unwrap();
            rx_runner.recv().unwrap();

            variables.k.write().unwrap().assign(Hardened::from(0))?;
            tx_runner.send("i11").unwrap();
            rx_runner.recv().unwrap();

            while *variables.k.read().unwrap() < *variables.size.read().unwrap() {
                tx_runner.send("i12").unwrap();
                rx_runner.recv().unwrap();

                let tmp = (*variables.acc.read().unwrap() + Hardened::from(
                    variables.a.read().unwrap()[variables.i.read().unwrap().inner()?][variables.k.read().unwrap().inner()?].inner()? *
                        variables.b.read().unwrap()[variables.k.read().unwrap().inner()?][variables.j.read().unwrap().inner()?].inner()?
                ))?;
                variables.acc.write().unwrap().assign(tmp)?;
                tx_runner.send("i13").unwrap();
                rx_runner.recv().unwrap();

                let tmp = (*variables.k.read().unwrap() + 1)?;
                variables.k.write().unwrap().assign(tmp)?;
                tx_runner.send("i14").unwrap();
                rx_runner.recv().unwrap();
            }

            variables.row.write().unwrap().push(*variables.acc.read().unwrap());
            tx_runner.send("i15").unwrap();
            rx_runner.recv().unwrap();

            let tmp = (*variables.j.read().unwrap() + 1)?;
            variables.j.write().unwrap().assign(tmp)?;
            tx_runner.send("i16").unwrap();
            rx_runner.recv().unwrap();
        }

        variables.result.write().unwrap().push(variables.row.read().unwrap().clone());
        tx_runner.send("i17").unwrap();
        rx_runner.recv().unwrap();

        let tmp = (*variables.i.read().unwrap() + 1)?;
        variables.i.write().unwrap().assign(tmp)?;
        tx_runner.send("i18").unwrap();
        rx_runner.recv().unwrap();
    }

    Ok(())


    /*
    let size = Hardened::from(a.len());
    let mut result: Vec<Vec<Hardened<i32>>> = Vec::new();

    let mut i = Hardened::from(0);
    let mut j = Hardened::from(0);
    let mut k = Hardened::from(0);

    while i < size {
        let mut row: Vec<Hardened<i32>> = Vec::new();
        j.assign(Hardened::from(0))?;

        while j < size {
            let mut acc = Hardened::from(0);
            k.assign(Hardened::from(0))?;

            while k < size {
                acc.assign((acc + Hardened::from(a[i.inner()?][k.inner()?].inner()?   *   b[k.inner()?][j.inner()?].inner()?) )? )?;
                k.assign((k + 1)?)?;
            }
            row.push(acc); // Aggiunge il valore calcolato alla riga
            j.assign((j + 1)?)?;
        }
        result.push(row); // Aggiunge la riga alla matrice risultante
        i.assign((i + 1)?)?;
    }
    Ok(result)
    */

}