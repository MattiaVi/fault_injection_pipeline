fn matrix_multiplication(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let size = a.len();
    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < size {
        let mut row: Vec<i32> = Vec::new();
        j = 0;

        while j < size {
            let mut acc = 0;
            k = 0;

            while k < size {
                acc += a[i][k] * b[k][j];
                k += 1;
            }
            row.push(acc); // Aggiunge il valore calcolato alla riga
            j += 1;
        }
        result.push(row); // Aggiunge la riga alla matrice risultante
        i += 1;
    }
    result
}
