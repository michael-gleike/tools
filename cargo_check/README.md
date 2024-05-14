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

### Liste der vom Borrow Checker gefundenen Errors
- [E0161:](https://doc.rust-lang.org/error_codes/E0161.html) A value was moved whose size was not known at compile time.
- [E0373:](https://doc.rust-lang.org/error_codes/E0373.html) A captured variable in a closure may not live long enough.
- [E0381:](https://doc.rust-lang.org/error_codes/E0381.html) It is not allowed to use or capture an uninitialized variable.
- [E0382:](https://doc.rust-lang.org/error_codes/E0382.html) A variable was used after its contents have been moved elsewhere.  
- [E0384:](https://doc.rust-lang.org/error_codes/E0384.html) An immutable variable was reassigned.
- [E0499:](https://doc.rust-lang.org/error_codes/E0499.html) A variable was borrowed as mutable more than once.
- [E0500:](https://doc.rust-lang.org/error_codes/E0500.html) A borrowed variable was used by a closure.
- [E0501:](https://doc.rust-lang.org/error_codes/E0501.html) A mutable variable is used but it is already captured by a closure.
- [E0502:](https://doc.rust-lang.org/error_codes/E0502.html) A variable already borrowed as immutable was borrowed as mutable.
- [E0503:](https://doc.rust-lang.org/error_codes/E0503.html) A value was used after it was mutably borrowed.
- [E0505:](https://doc.rust-lang.org/error_codes/E0505.html) A value was moved out while it was still borrowed
- [E0506:](https://doc.rust-lang.org/stable/error_codes/E0506.html) An attempt was made to assign to a borrowed value.
- [E0507:](https://doc.rust-lang.org/stable/error_codes/E0507.html) A borrowed value was moved out.
- [E0508:](https://doc.rust-lang.org/stable/error_codes/E0508.html) A value was moved out of a non-copy fixed-size array.
- [E0509:](https://doc.rust-lang.org/stable/error_codes/E0509.html) This error occurs when an attempt is made to move out of a value whose type implements the `Drop` trait.
- [E0510:](https://doc.rust-lang.org/stable/error_codes/E0510.html) The matched value was assigned in a match guard.
- [E0515:](https://doc.rust-lang.org/stable/error_codes/E0515.html) A reference to a local variable was returned.
- [E0521:](https://doc.rust-lang.org/stable/error_codes/E0521.html) Borrowed data escapes outside of closure.
- [E0524:](https://doc.rust-lang.org/stable/error_codes/E0524.html) A variable which requires unique access is being used in more than one closure at the same time.
- [E0594:](https://doc.rust-lang.org/error_codes/E0594.html) A non-mutable value was assigned a value.
- [E0596:](https://doc.rust-lang.org/stable/error_codes/E0596.html) This error occurs because you tried to mutably borrow a non-mutable variable.  
- [E0597:](https://doc.rust-lang.org/stable/error_codes/E0597.html) This error occurs because a value was dropped while it was still borrowed.
- [E0626:](https://doc.rust-lang.org/error_codes/E0626.html) This error occurs because a borrow in a coroutine persists across a yield point.
- [E0713:](https://doc.rust-lang.org/error_codes/E0713.html) This error occurs when an attempt is made to borrow state past the end of the lifetime of a type that implements the `Drop` trait.
- [E0716:](https://doc.rust-lang.org/error_codes/E0716.html) A temporary value is being dropped while a borrow is still in active use.

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