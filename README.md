# ProgettoPdS::prj_G19
#### Realizzazione di un ambiente di Fault Injection per applicazione ridondata

[Traccia del progetto](https://github.com/cmigliaccio00/ProgettoPdS_materiale)

>**<u>CONSIGLIO 0</u>** Prima di fare qualsiasi modifica al 
> progetto 
> presente in 
> questa 
> cartella,
ricordarsi di fare `git pull` per evitare spiacevoli inconvenienti che
portano a perdita di tempo e salute.


### Consigli:
- Leggere i paper indicati con `(todo)` nella traccia del progetto (vedi 
  link sopra)
- Leggere le slide relative a _Moduli e test_: può essere utile soprattutto a 
  livello di "SoftEng".
<hr style="border: 0.5px solid red">

## Casi di studio
Dopo un'indagine, e volendo mantenere una certa coerenza con il lavoro già 
fatto da altri nelle pubblicazioni scientifiche di cui sopra, si è giunti a 
una sorta di compromesso, decidendo di prendere in esame:
1. **Bubble sort** o **Selection Sort** (Algoritmo di ordinamento), da 
   scegliere uno dei due in base ai risultati prodotti;
2. **Moltiplicazioni tra due matrici** 5x5 (Algoritmo classico usato in una 
   varietà cospicua di contesti).

## Prima Parte (Mattia)
E' la parte che riguarda l'irrobustimento del codice e l'utilizzo del tipo 
`Hardened` modificandone/migliorandone l'implementazione se necessario, 
partendo da quella già presente nel modulo `hardened`. Un commento più 
approfondito viene riportato di seguito.

**Task da svolgere (orientativamente parlando...)**:
1. Reperire il codice per gli algoritmi citati, scriverli in Rust e 
   verificarne la correttezza logica e sintattica con qualche test/esempio 
   che li utilizzi (Selection Sort già c'è); in questo modo si ha la 
   certezza di andare avanti con un livello di astrazione diverso partendo 
   da una base corretta; 
2. Riscrivere/Adattare l'algoritmo proposto con i tipi `Hardened<T>` (per 
   dettagli 
   ulteriori vedi video relativo sulla cartella condivisa di Google Drive).
   - Nota che: in questa fase ci potrebbe essere la necessità di dover 
      implementare tratti mancanti nell'implementazione attuale. 
   - Ad esempio: 
      per la _moltiplicazione di matrici_ bisogna fare operazioni del tipo 
      `acc += r_el*c_el` dove `acc` è un accumulatore e `r_el`, `c_el` 
      sono l'elemento corrispondente di riga e colonna della matrice; 
     tutte queste variabili per gli scopi del progetto sono di tipo 
     Hardened. I tratti `Mul` e `AddAssign`, per eseguire rispettivamente 
     `*` e `+=`, non sono implementati. Bisogna quindi che vengano 
     implementati prima di poter scrivere il codice che li utilizzi, 
     diversamente il compilatore genererà un errore relativo al fatto che 
     per quel tipo non ci sono certi tratti. Quello che ci 
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
>  - Se la firma del metodo di un certo tratto ha un tipo associato `type Output` allora posso ritornare un `Result<Hardened<T>,IncoherenceError>`;
>  - Altrimenti devo ritornare un `panic!(...)` prendendo come spunto quello che si è già fatto o cambiandolo se si crede che si possa fare meglio 
> e diversamente.


## Seconda Parte 
Questa parte è relativa allo sviluppo dell'ambiente di Fault Injection che 
prende come programmi attaccati (target) quelli implementati nella fase 
precedente.

Si è pensato di implementare questa seconda parte realizzando una sorta di 
pipeline combinando l'utilizzo di thread e canali di tipo 
'multiple producer single consumer (mpsc)'.

>In questa parte non serve aver già pronti i casi di studio e l'analizzatore 
> per poter andare avanti con le due parti centrali (Fault Manager e 
> Iniettore). Si può utilizzare invece come "benchmark" il caso di studio 
> già implementato del __Selection Sort__.

### Implementazione della pipeline
![](pipeline_img.png)

La pipeline che implementa tutta la seconda parte del progetto è composta da 
tanti stadi quante sono le fasi di analisi: 
1. Fault Manager
2. Iniettore
3. Analizzatore

Questi fanno utilizzo di alcune fonti (indicate nello schema con dei cilindri):
1. La **fault list** (per ogni caso di studio): precedentemente generata e 
   serializzata.
2. Il **programma target** (caso di studio) utilizzato sia in fase di 
   iniezione che in fase di generazione della lista di guasti per l'analisi 
   statica del codice.
3. Il **report** che potrebbe essere salvato (volendo) su un file di testo 
   oltre che a essere visualizzato (CLI, GUI o altro).

Un'idea sarebbe quella di realizzare questa pipeline all'interno di una 
funzione wrapper fatta in questo modo o in un modo simile:

```rust
use std::sync::mpsc::channel;

fn fault_injection_env(fault_list: String,      //nome file fault-list
                       target: String,          //nome programma target
                       report_name: String) {   //nome file report
   let (tx_chan_fm_inj, rx_chan_fm_inj) = channel();
   let (tx_chan_inj_anl, rx_chan_inj_anl) = channel(); 
   
   //Questi possono essere a loro volta wrapper che faranno delle cose
   FaultManager(tx_chan_fm_inj,fault_list);
   Iniettore(rx_chan_fm_inj, tx_chan_inj_anl, target); 
   Analizzatore(rx_chan_inj_anl);
}
```

### Fault List e Fault manager (Carlo)
**Task da svolgere (orientativamente parlando...)**:
1. **Lista dei guasti**:
   1. Analisi statica di ogni caso di studio usando eventualmente `clippy`, 
      `rust-analyzer`, `syn`. Di particolare interesse sono: nome della 
      variabile, dimensione della variabile, numero di righe di codice di cui è 
      costituito il programma (caso di studio).
   2. Sulla base delle informazioni generare un certo numero di entry fatte 
      da `<variabile, tempo_iniezione, fault_mask>`
   3. Si potrebbe pensare di serializzare le informazioni su un file 
      utilizzando il crate `serde`, chiaramente questo prevede che venga 
      creato un tipo nuovo. Ad esempio:
   ```rust
    struct FaultListEntry{
        var: String,
        time: usize, 
        fault_mask: u64
    }
    ```
   A questa bisogna far implementare (tramite `derive`) i tratti 
   `Serialize` e `Deserialize`. (Si potrebbe pensare alla generazione della 
   lista di guasti come una funzionalità del programma, poiché c'è un pezzo 
   di codice che fa questo 'mestiere').
2. **Fault List Manager**:
   1. Deserializzare la lista dei guasti all'interno di una opportuna 
      collezione;
   2. Ogni guasto deve essere mandato tramite l'opportuno `Sender` 
      attraverso il canale che lo porta verso l'iniettore (stage della pipeline successivo).
   3. Una volta terminate le entry si può distruggere l'estremità 'TX' del 
      canale che collega i primi due stadi, in modo che l'iniettore non 
      resti in attesa all'infinito.

### Iniettore (Alessandro)
E' la parte centrale della pipeline che si occupa di iniettare nelle 
variabili ai tempi di iniezione stabiliti. Potrebbe essere utile in fase di 
presentazione del progetto chiarire tra le tante tecniche di _Software 
Fault-Injection_ disponibili quale è stata implementata. Il nostro caso si 
potrebbe riportare al caso di **Runtime injection** di tipo _Exception/trap_ 
(dall'articolo di riferimento) si cita:

>_Exception/trap._ In this case, a hardware exception or a software trap 
> transfer control to the
fault injector. Unlike time-out, exception/trap can
inject the fault whenever certain events or conditions occur. For example, a software trap
instruction inserted into a target program will
invoke the fault injection before the program executes a particular instruction. When the trap executes, an interrupt is generated that transfers
control to an interrupt handler. (...)

**Task da svolgere (orientativamente parlando...)**:
1. **Prelievo del guasto**, tramite l'opportuno `Receiver` (dal canale che 
   collega primo e secondo stadio), chi realizza lo stage prima (Carlo) deve 
   comunicare come minimo come è fatto il tipo associato alla 'fault-entry' e 
   qualche 
   metodo al fine di 
   poterlo utilizzare correttamente.
2. **Avvio del task e iniezione del guasto**: si fa partire il thread e al 
   momento opportuno viene iniettato il guasto.
3. **Aspetto la terminazione**: ci si arresta sulla `JoinHandle`, invocando poi
   su questa il metodo `join()` che come noto restituisce il Result.
4. **Mandare il risultato** verso l'opportuno `Sender`, descrivere inoltre a 
   modo di 
   documentazione qual è il risultato che l'analizzatore dovrà aspettarsi in 
   ottica di poter avere un 'manuale' con cui poter utilizzare queste 
   informazioni ai fini dell'analisi.

#### Aspetti pratici/realizzativi (spunti)
1. **Condivisione delle informazioni tra thread**
   1. Per la condivisione di variabili tra thread diversi vanno senza dubbio 
      utilizzati smart pointer di tipo `Arc`;
   2. Occorre dare possibilità alla coppia di thread iniettore-iniettato di 
      modificare le variabili. A questo punto le strade che si possono 
      utilizzare sono due: utilizzo di blocchi `unsafe`, utilizzo di `Mutex` 
      per accedere ai dati in maniera controllata.
   3. Si dovrebbe in qualche modo trasformare la stringa proveniente dalla 
      fault-entry in una variabile da utilizzare, ci dovrebbero essere in 
      rust delle macro o funzioni che mi permettono di fare una sorta di 
      `eval` di un'espressione.
2. **Temporizzazione tra thread iniettato e thread iniettore**
   1. A questo scopo si possono utilizzare i channel. Prendendo ispirazione 
      dal paper che utilizza il tracing per far partire l'injection a un 
      certo istante, si potrebbe far seguire ogni istruzione del caso di 
      studio analizzato da una `send()` 
      fatta all'interno di un canale creato tra i due thread. 
   2. Il thread 
      iniettore resta in ascolto di questi messaggi e li conta, al momento 
      opportuno prende il possesso del lock e inietta nelle variabili 
      utilizzando la fault mask. 

### Analizzatore (Federico)
Quest'ultima fase è quella in cui si **raccolgono** e **analizzano** tramite 
tabulati, indici di tendenza centrale, indici di dispersione... i dati 
ottenuti dalle simulazioni effettuate agli stage precedenti.

**Task da svolgere (orientativamente parlando...)**:
1. Per ogni elemento prelevato tramite un opportuno Receiver (del canale che 
   collega Iniettore e Analizzatore) si tengono aggiornati i contatori di (a 
   titolo di esempio):
   1. _Fault silent_
   2. _Errori rilevati tramite panic/IncoherenceError_
   Per questa parte si utilizzano le informazioni provenienti da chi 
      sviluppa lo stage di prima (Ale).
2. Per la presentazione dei risultati ci si può ispirare ai paper di 
   riferimento sia della prima che della seconda parte. In particolare 
   [questo articolo](https://github.com/cmigliaccio00/ProgettoPdS_materiale/blob/main/paper/SecondPart/1.pdf)

## Questioni ancora aperte
- Come interagisce un utente di questo programma con l'applicazione? 
  (esempio: CLI, GUI...) se abbiamo tempo sarebbe carino creare 
  un'interfaccia grafica semplice con pochissime funzioni essenziali e per 
  la visualizzazione dei risultati ottenuti.
- Varie ed eventuali... (modificate voi all'occorrenza questa parte)