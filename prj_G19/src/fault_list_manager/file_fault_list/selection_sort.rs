fn selection_sort(vet: Vec<i32>){
    let mut N = vet.len();
    let mut j=0;
    let mut min=0;

    //-----------------------SELECTION SORT-------------------------
    let mut i=0;
    while i<N-1{
        min=i;
        j=i+1;
        //Ricerca del minimo
        while j<N{
            if Vec[j] < Vec[min]{   Pos=j;  }
            j = j+1;
        }
        //Scambio il minimo
        vet.swap(min,i);
        //Vado avanti
        i=i+1;
    }
}