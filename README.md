# ProgFaultInjection
> Realizzazione di un ambiente di Fault Injection per applicazione ridondata

>**Consiglio 0** <div style='color:red'>Prima di fare qualsiasi modifica al 
> progetto 
> presenta in 
> questa 
> cartella,
ricordarsi di fare `git pull` per evitare spiacevoli inconvenienti che
portano a perdita di tempo e salute.<div>


### Consigli:
- Leggere i paper indicati con **(todo)** nella traccia del progetto vedi 
  sotto `Group19/`
- Leggere le slide relative a _Moduli e test_ può essere utile soprattutto a 
  livello di "SoftEng".


## Casi di studio
1. **Bubble sort** o **Selection Sort** (Algoritmo di ordinamento)
2. **Moltiplicazioni tra due matrici** 5x5 (Algoritmo classico usato in una 
   varietà cospicua di contesti)

## Prima Parte (Mattia)
E' la parte che riguarda l'irrobustimento del codice e l'utilizzo del tipo 
`Hardened` modificandone/migliorandone l'implementazione se necessario, 
partendo da quella già presente nel modulo `hardened`. Un commento più 
approfondito viene riportato di seguito.

**Task da svolgere**:
1. Reperire il codice per gli algoritmi citati, scriverli in Rust e 
   verificarne la correttezza logica e sintattica con qualche test/esempio 
   che li utilizzi (Selection Sort già c'è); 
2. Scrivere l'algoritmo proposto con i tipi `Hardened<T>` (per dettagli 
   ulteriori vedi video relativo sulla cartella condivisa di Google Drive).
   - Nota che: in questa fase ci potrebbe essere la necessità di dover 
      implementare tratti mancanti nell'implementazione attuale. 
   - Ad esempio: 
      per la _moltiplicazione di matrici_ bisogna fare operazioni del tipo 
      `acc += r_el*c_el` dove `acc` è un accumulatore e `r_el`, `c_el` 
      sono l'elemento corrispondente di riga e colonna della matrice; 
     tutte queste variabili per gli scopi del progetto sono di tipo 
     Hardened. I tratti `Mul` e `AddAssign` per eseguire rispettivamente 
     `*` e `+=` non sono implementati. Bisogna quindi che vengano 
     implementati prima di poter scrivere il codice che li utilizzi, 
     diversamente il compilatore genererà un errore relativo al fatto che 
     per quel tipo non sono implementati certi tratti. Quello che ci 
     siamo detti fino a questo punto.
3. Testare tramite qualche esempio che le cose vadano come ci si aspetta (eg.
   Controllo di correttezza dell'output...). [Volendo, in base al tempo 
   disponibile,  si potrebbero anche  scrivere dei test d'unità per gli 
   algoritmi implementati secondo il 
   paradigma AAA]. 

> **Nota aggiuntiva** Nell'implementazione delle feature mancanti del tipo 
> `Hardened` vanno gestiti e fatti sempre i controlli di consistenza sulle 
> parti in cui è necessario (ricorda dalle "regole d'oro": ogni lettura di 
> una qualsiasi variabile deve essere sempre preceduto da un controllo di 
> consistenza). In particolare:
>  - Se la firma del metodo di un certo tratto ha un tipo associato `type 
      Output` allora posso ritornare un `Result<Hardened<T>,IncoherenceError>`;
>  - Altrimenti devo ritornare un `panic!(...)` prendendo come spunto quello che si è già fatto o cambiandolo se si crede che si possa fare meglio 
> e diversamente.


## Seconda Parte 
Questa parte è relativa allo sviluppo dell'ambiente di Fault Injection che 
prende come programmi attaccati (target) quelli implementati nella fase 
precedente.

### Fault List e Fault manager (Carlo)
Compiti:
 
### Iniettore (Alessandro)

### Analizzatore (Federico)