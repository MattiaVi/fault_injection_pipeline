fn selection_sort(vet: Vec<i32>, N: i32){
    let mut i=0;
    let mut j=0;
    let Pos=0;

    while i<N-1{
        Pos=i;
        j=i+1;
        //Ricerca del minimo
        while j<N{
            if Vec[j] < Vec[Pos]{   Pos=j;  }
            j = j+1;
        }
        if Pos!=i{      vet.swap(Pos,i);    }
        i=i+1;
    }
}