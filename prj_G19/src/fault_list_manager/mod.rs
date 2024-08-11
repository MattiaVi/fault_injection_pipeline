pub mod static_analysis;

///Generazione della fault list:
///     - generazione casuale di un certo numero di entry +

pub struct FaultListEntry{
    var: String,
    time: usize,
    fault_mask: u64,
}

impl FaultListEntry{
    fn get_var(&self)->&str{
        &self.var
    }
    fn get_time(&self)->usize{
        self.time
    }
    fn get_fault_mask(&self)->u64{
        self.fault_mask
    }
}

//Fault List Manager

#[cfg(test)]
mod tests{
    #[test]
    fn test_trivial(){
        assert_eq!(2,2);
    }
}