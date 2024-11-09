/*
fn matrix_multiplication(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let size = 5; // Dimensione fissa 5x5
    let mut result = vec![vec![0; size]; size];

    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}
*/

fn matrix_multiplication(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let size = 5; // Dimensione fissa 5x5
    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut i=0;
    let mut k=0;
    let mut j=0;

    for i in 0..size {
        let mut row: Vec<i32> = Vec::new(); // Crea una nuova riga
        for j in 0..size {
            let mut acc = 0;
            for k in 0..size {
                acc += a[i][k] * b[k][j];
            }
            row.push(acc); // Aggiunge il valore calcolato alla riga
        }
        result.push(row); // Aggiunge la riga alla matrice risultante
    }
    result
}