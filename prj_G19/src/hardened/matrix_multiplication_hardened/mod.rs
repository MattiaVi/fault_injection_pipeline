mod tests;
use crate::hardened::*;
pub fn matrix_multiplication(a: &Vec<Vec<Hardened<i32>>>, b: &Vec<Vec<Hardened<i32>>>) -> Result<Vec<Vec<Hardened<i32>>>, IncoherenceError> {

    let size = Hardened::from(a.len());
    let mut result: Vec<Vec<Hardened<i32>>> = Hardened::from_mat(Vec::new());

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
}