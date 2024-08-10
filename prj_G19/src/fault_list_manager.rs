pub struct FaultListEntry{
    var: String,
    time: usize,
    fault_mask: u64,
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_trivial(){
        assert_eq!(2,2);
    }
}