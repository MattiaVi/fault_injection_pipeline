use std::cmp::Ordering;
use std::fmt::{Display, Debug, Formatter};
use std::ops::{Add, Index, IndexMut, Sub};
use thiserror::Error;

//-------------------------------------------------------------
#[derive(Clone, Copy)]
/// <h2>Tipo ```Hardened<T>``` </h2> <br>
/// <p>Questo nuovo tipo 'Hardened' ha al suo interno DUE COPIE
/// del valore della variabile di tipo T.
/// Questo per asserire alla realizzazione della 'Regola 1':
/// ogni variabile x deve essere duplicata facendone due copie x1 e x2. </p>
pub struct Hardened<T>{
    cp1: T,
    cp2: T,
}

impl<T> Hardened<T>
where T: Debug+PartialEq+Eq+Copy+Clone{
    ///Controllo di coerenza: si controlla che le due copie del valore della
    /// variabile siano uguali. E' la funzione utilizzata affinché venga rispettata
    /// la 'Regola 3' secondo cui ogni lettura deve essere preceduta dal controllo delle
    /// due copie, nel caso in cui questo fallisse, è stato trovato un fault!
    fn incoherent(&self)->bool{
        self.cp1 != self.cp2
    }

    /// L'operazione di assegnazione non può essere ridefinita (cioè non posso ridefinire '='
    /// in a=b) perché dovrei modificare la semantica del movimento caratteristica di Rust.
    /// L'operazione del tipo a=b, con a, b di tipo ```Hardened<T>``` deve essere fatta nel seguente
    /// modo: ```a.assign(b)```
    pub fn assign(&mut self, other: Hardened<T>)->Result<(), IncoherenceError>{
        if other.incoherent(){
            return Err(IncoherenceError::AssignFail)
        }
        //Regola 2: Ogni scrittura deve essere eseguita su entrambe le copie
        self.cp1 = other.cp1;
        self.cp2 = other.cp2;
        Ok(())
    }

    ///Crea un vettore ```Vec<Hardened<T>>``` da un Vec<T>
    pub fn from_vec(vet: Vec<T>)->Vec<Hardened<T>>{
        vet.iter().map(|&x| Hardened::from(x)).collect()
    }

    ///Estrae (dopo aver controllato la coerenza del dato) il dato
    /// di tipo T incapsulato al suo interno.
    pub fn inner(&self)->Result<T, IncoherenceError>{
        if self.incoherent(){
            return Err(IncoherenceError::Generic)
        }
        Ok(self.cp1)
    }
}

///Crea una variabile di tipo ```Hardened<T>``` da una di tipo T,
/// si assume che tale variabile sia copiabile.
impl<T> From<T> for Hardened<T> where T:Copy{
    fn from(value: T) -> Self {
        Self{cp1: value, cp2: value}
    }
}

//---------------------OPERAZIONI ARITMETICHE-------------------------
//Tutte queste operazioni in caso di fallimento ritornano un Errore
//di tipo IncoherenceError, implementato usando il crate thiserror.
// a = b+c
impl<T> Add for Hardened<T>
where T: Add<Output=T>+PartialEq+Eq+Debug+Copy+Clone{
    type Output = Result<Hardened<T>, IncoherenceError>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.incoherent() || rhs.incoherent(){
            return Err(IncoherenceError::AddFail)
        }
        Ok(Self{
            cp1: self.cp1 + rhs.cp1,
            cp2: self.cp2 + rhs.cp2,
        })
    }
}

impl Add<usize> for Hardened<usize>{
    type Output = Result<Hardened<usize>, IncoherenceError>;
    fn add(self, rhs: usize) -> Self::Output {
        if self.incoherent() {
            return Err(IncoherenceError::AddFail);
        }
        Ok(Self{
            cp1: self.cp1 + rhs,
            cp2: self.cp2 + rhs,
        })
    }
}

impl<T> Sub for Hardened<T>
where T:Sub<Output=T>+PartialEq+Eq+Debug+Copy+Clone{
    type Output=Result<Hardened<T>,IncoherenceError>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.incoherent() || rhs.incoherent(){
            return Err(IncoherenceError::Generic)
        }
        Ok(Self{
            cp1: self.cp1 - rhs.cp1,
            cp2: self.cp2 - rhs.cp2,
        })
    }
}

impl Sub<usize> for Hardened<usize>{
    type Output = Result<Hardened<usize>, IncoherenceError>;
    fn sub(self, rhs: usize) -> Self::Output {
        if self.incoherent(){
            return Err(IncoherenceError::Generic)
        }
        return Ok(Self{
            cp1: self.cp1 - rhs,
            cp2: self.cp2 - rhs,
        })
    }
}
//------------------------------------------------------------------------

//------------------------OPERAZIONI DI CONFRONTO-------------------------
impl<T> PartialEq for Hardened<T>
where T:PartialEq+Eq+Debug+Copy+Clone{
    fn eq(&self, other: &Self) -> bool {
        if  other.incoherent(){
            panic!("Found an incoherence!")
        }
        self.cp1.eq(&other.cp1)
    }
}

impl<T> Eq for Hardened<T>
where T:PartialEq+Eq+Debug+Copy+Clone{      }

impl<T> PartialOrd for Hardened<T>
where T:PartialEq+PartialOrd+Eq+Debug+Copy+Clone{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.incoherent(){
            panic!("Found an incoherence!")
        }
        self.cp1.partial_cmp(&other.cp1)
    }
}

impl<T> Ord for Hardened<T>
where T:PartialEq+PartialOrd+Ord+Eq+Debug+Copy+Clone{
    fn cmp(&self, other: &Self) -> Ordering {
        if other.incoherent(){
            panic!("Found an incoherence!");
        }
        self.cp1.cmp(&other.cp1)
    }
}

//Funzioni per indicizzare un Vec usando un Hardened<usize>
impl<T> Index<Hardened<usize>> for Vec<Hardened<T>>{
    type Output=Hardened<T>;
    ///Estrae un riferimento immutabile
    fn index(&self, index: Hardened<usize>) -> &Self::Output {
        if index.incoherent(){
            panic!("Found an incoherence!");
        }
        self.index(index.cp1)
    }
}

impl<T> IndexMut<Hardened<usize>> for Vec<Hardened<T>>{
    fn index_mut(&mut self, index: Hardened<usize>) -> &mut Self::Output {
        if index.incoherent(){
            panic!("Found an incoherence");
        }
        self.index_mut(index.cp1)
    }
}

//Per poter stampare il tipo Hardened<T> con la macro println!() e il
// modificatore {:?}
impl<T> Debug for Hardened<T> where T:Debug+PartialEq+Eq+Copy+Clone{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.incoherent(){
            panic!("Found an incoherence");
        }
        self.cp1.fmt(f)
    }
}


///Funzione di prova
/// <h3>Caso di studio 1: Selection Sort</h3>
fn selection_sort(vet: &mut Vec<Hardened<i32>>)->Result<(), IncoherenceError>{
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
    //------------------------------------------------------
    Ok(())
}



//-------------------------------------------------------------
///Tipo di errore generato tutte le volte che fallisce il controllo
/// di coerenza delle due copie all'interno di una variabile di tipo
/// ```Hardened<T>```.
#[derive(Error, Debug)]
pub enum IncoherenceError{
    #[error("IncoherenceError::AssignFail: assignment failed")]
    AssignFail,
    #[error("IncoherenceError::AddFail: due to incoherence add failed")]
    AddFail,
    #[error("IncoherenceError::MulFail: due to incoherence mul failed")]
    MulFail,
    #[error("IncoherenceError::Generic: generic incoherence error")]
    Generic
}

#[cfg(test)]
mod tests{
    use crate::Hardened;
    use crate::hardened::selection_sort;
    use crate::IncoherenceError;
    #[test]
    fn test_add_ok(){
        //Arrange
        let a = Hardened::from(3);
        let b = Hardened::from(2);
        //Act
        let c = (a+b);
        //Assert
        assert_eq!(c.is_ok(), true);
        assert_eq!(c.unwrap().inner().unwrap(), 5);
    }
    #[test]
    fn test_add_err(){
        let mut a = Hardened::from(3);
        let  b = Hardened::from(2);
        a.cp1 = a.cp1 & 0;      //Injection

        let c=a+b;
        assert!(c.is_err());
    }
    #[test]
    fn test_ord(){
        let a = Hardened::from(5);
        let b = Hardened::from(4);
        assert!(a>b);
    }
    #[test]
    fn test_add_with_usize(){
        let a = Hardened::from(4);
        let ris = a+5;
        assert!(ris.is_ok());
        assert_eq!(ris.unwrap().inner().unwrap(), 9);
    }

    #[test]
    //Provo a usare il nuovo tipo per ordinare un vettore
    fn test_sort(){
        let mut myvec = Hardened::from_vec(vec![31, 10, 15, 6, 4, 3]);
        assert!(selection_sort(&mut myvec).is_ok());
        let mut myvec2 = Hardened::from_vec(vec![3,4,6,10,15,31]);
        assert_eq!(myvec, myvec2);
    }


}
