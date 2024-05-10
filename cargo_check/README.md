# cargo check
``cargo check`` ist ein tool um alle lokalen Pakete und Abhängikeiten zu überprüfen. Dazu werden alle wesentlichen Pakete kompiliert ohne dabei den letzten Schritt der Codegenerierung durchzuführen. Somit ist die Ausführung schneller als ``cargo build``.

## Benutzung
Cargo check kann ganz einfach per Shell-Command gestartet werden.
````shell
cargo check
````
Dabei gibt es noch verschiedene Optionen zur Auswahl des Packages, des Ziels, der Features und des Outputs. Dazu kommen dann noch Kompilieroptionen, Displayoptionen und Manifestoptionen. Die jeweilige Funktionalität der Optionen lässt sich in der [Dokumentation](https://doc.rust-lang.org/cargo/commands/cargo-check.html) nachlesen oder per Aufruf einenr der folgenden Shell-Commands anzeigen.
````shell
cargo check -h
cargo check --help
````

## Ownership und Borrowing
Cargo check verwendet nur den rust borrow checker und stellt somit keine Erweiterte Funktionalität zur Verfügung. Somit bietet cargo check gegenüber ``cargo build`` nur den Vorteil des Performancegewinns. Jedoch werden manche Fehler und Diagnosen erst während der Codegenerierung ausgegeben, was bedeutet, dass sie von cargo check nicht gemeldet werden.
### Konkrete Probleme
1. cargo check gibt nicht immer alle Fehler auf einmal an. Somit kann es ratsam sein, nach dem Beheben der Fehler erneut cargo check anzuwenden.
2. cargo check gibt einem nur die Informationen die cargo build ebenfalls findet. Da es sich dabei jeweils um Kommandozeilen-Tools handelt werden diese Fehler nicht grafisch visualisiert und es gibt kein Syntax highlighting der fehlerhaften Stelle.
3. cargo check verwendet den rust Borrow Checker. Dieser findet zwar viele Fehler, jedoch werden vor allem manche zyklischen Referenzen nicht als solche erkannt. Somit kann es zu einem Speicherleck trotz check kommen. Ein Beispiel hierfür wäre die folgende Funktion:
    ````rust
       fn leak() {
           let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        
           println!("a initial rc count = {}", Rc::strong_count(&a));
           println!("a next item = {:?}", a.tail());
        
           let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        
           println!("a rc count after b creation = {}", Rc::strong_count(&a));
           println!("b initial rc count = {}", Rc::strong_count(&b));
           println!("b next item = {:?}", b.tail());
        
           if let Some(link) = a.tail() {
           *link.borrow_mut() = Rc::clone(&b);
           }
        
           println!("b rc count after changing a = {}", Rc::strong_count(&b));
           println!("a rc count after changing a = {}", Rc::strong_count(&a));
        
           // Wenn man die nächste Zeile unkommentiert wird man sehen, dass es eine zyklische Referenz gibt und es zu einem Stackoverflow kommt.
           //println!("a next item = {:?}", a.tail());
       }
    ````