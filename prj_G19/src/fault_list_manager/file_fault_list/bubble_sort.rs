fn bubble_sort(vet: Vec<i32>) {
    let N:usize = vet.len();
    let mut i=0;
    let mut j=0;

    for i in 0..N {
        // Se non ci sono scambi, l'array è ordinato (ottimizzazione)
        let mut swapped = false;
        for j in 0..(N - i - 1) {
            if vet[j] > vet[j + 1] {
                vet.swap(j, j + 1);
                swapped = true;
            }
        }
        // Se non è avvenuto nessuno scambio, interrompi il ciclo
        if !swapped {
            break;
        }
    }
}