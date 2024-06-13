# Aquascope
[Aquascope](https://github.com/cognitive-engineering-lab/aquascope) ist ein Tool, das interaktive Visualisierungen von Rust-Programmen erzeugt. Diese größtenteils automatisch generierten Visualisierungen zeigen das Kompilierungs- und Laufzeit-Verhalten von Rust Programmen.
<br>

## Visualisierung
Aquascope stellt einen [mdBook](https://rust-lang.github.io/mdBook/) Präprozessor dar der Aquascope-Diagramme in ein mdBook integriert.
Das folgende Bild zeigt eine Beispielvisualisierung: <br>
![alt tag](pictures/example.png)

#### Symbol-Bedeutungen
|                Kategorien                | Typen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
|:----------------------------------------:|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|            Read-Berechtigung             | ![gainRead.png](pictures/notation/gainRead.png) Die Variable hatte bisher keine Read-Berechtigung erhällt diese aber in dieser Zeile.<br>![read.png](pictures/notation/read.png) Die Variable hatte bereits Read-Berechtigungen und es findet keine Änderung daran statt.<br>![dropRead.png](pictures/notation/dropRead.png) Die Variable hatte zuvor Read-Berechtigungen, aber verliert diese jetzt.                                                                                                                                                                                                                                                                                                                            |
|            Write-Berechtigung            | ![noWrite.png](pictures/notation/noWrite.png) Die Variable hatte bisher keine Write-Berechtigung und es findet keine Änderung daran statt. <br>![gainWrite.png](pictures/notation/gainWrite.png) Die Variable hatte bisher keine Write-Berechtigung erhällt diese aber in dieser Zeile. <br>![dropWrite.png](pictures/notation/dropWrite.png) Die Variable hatte zuvor Write-Berechtigungen, aber verliert diese jetzt.                                                                                                                                                                                                                                                                                                          |
|            Owner-Berechtigung            | ![noOwner.png](pictures/notation/noOwner.png) Die Variable hatte bisher kein Ownership über die Resource und es findet keine Änderung daran statt.<br>![gainOwner.png](pictures/notation/gainOwner.png) Die Variable hatte bisher kein Ownership über die Resource erhällt dies aber in dieser Zeile.<br>![dropOwner.png](pictures/notation/dropOwner.png) Die Variable hatte zuvor Ownership über die Resource, aber verliert dies jetzt.                                                                                                                                                                                                                                                                                       |
| Gründe für ein<br> Berechtigungsänderung | ![initialize.png](pictures/notation/initialize.png) Die Variable wird in dieser Zeile initialisiert.<br>![drop.png](pictures/notation/drop.png) Die Variable wird nach dieser Zeile nicht mehr verwendet.<br>![borrow.png](pictures/notation/borrow.png) Die Resource wird in dieser Zeile borrowed.<br>![regainBorrow.png](pictures/notation/regainBorrow.png) Die Resource erhällt in dieser Zeile ihre Berechtigungen zurück.                                                                                                                                                                                                                                                                                                 |
|         Erwartete Berechtigungen         | ![expectReadGranted.png](pictures/notation/expectReadGranted.png) Die Resource hält die erwartete Read-Berechtigung.<br>![expectRead.png](pictures/notation/expectRead.png) Die Resource hält die erwartete Read-Berechtigung nicht.<br>![expectWriteGranted.png](pictures/notation/expectWriteGranted.png) Die Resource hält die erwartete Write-Berechtigung.<br>![expectWrite.png](pictures/notation/expectWrite.png) Die Resource hält die erwartete Write-Berechtigung nicht.<br>![expectOwnerGranted.png](pictures/notation/expectOwnerGranted.png) Die Resource hält die erwartete Write-Berechtigung.<br>![expectOwner.png](pictures/notation/expectOwner.png) Die Resource hält die erwartete Write-Berechtigung nicht. |
<br>

## Installation
Die Installation des Tools ist momentan nur auf Linux-Systemen möglich. Die momentan neuste Version lässt sich wie folgt installieren:
````shell
cargo install mdbook
cargo install mdbook-aquascope --locked --version 0.3.2
rustup toolchain install nightly-2023-08-25 -c rust-src rustc-dev llvm-tools-preview miri
cargo +nightly-2023-08-25 install aquascope_front --git https://github.com/cognitive-engineering-lab/aquascope --tag v0.3.2 --locked
cargo +nightly-2023-08-25 miri setup
````
Die zweite Installationsmöglichkeit "from Source" bietet gegenüber der resten Möglichkeit nur den Vorteil den [Playground](https://cognitive-engineering-lab.github.io/aquascope/) lokal laufen lassen zu können. Dieser lässt sich aber in der momentan aktuellsten Version ``v0.3.2`` nicht bauen. Somit wird diese Installationsform hier nicht beleuchtet.
Um die Installation zu erleichtern wird ein Dockercontainer bereitgestellt.
````shell
~/tools$ docker compose up aquascope
````
<br>

## Benutzung
Die einfachste Möglichkeit Aquascope zu verwenden ist der zugehörige [Playground](https://cognitive-engineering-lab.github.io/aquascope/).

Um Aquascope bei einer lokalen Installation verwenden zu können muss zuerst (falls noch nicht geschehen) in dem jeweiligen Projekt ein [mdBook](https://rust-lang.github.io/mdBook/) erstellt werden:
````shell
~/project$ mdbook init <book-name>
````
Danach muss Aquascope in der `book.toml` Datei des mdBook's aktivieren. Dies geschieht durch Einfügen der folgenden Codezeile:
````toml
[preprocessor.aquascope]
````
Im Anschluss daran kann der annotierte Aquascope-Codeblock in einer der Markdown Quell-Files eingefügt werden:
````markdown
```aquascope,permissions,stepper,boundaries,shouldFail,horizontal
#fn main() {
let mut v = vec![1, 2, 3];`[]`
let n = &v[0];`[]``(focus,paths:v)`
v.push(0);`[]``{}`
let x = *n;`[]`
#}
```
````
Die Visualisierung lässt sich dann mit der Ausführung der folgenden Befehle im mdBook-Ordner generieren:
````shell
mdbook build
mdbook serve
````
### Syntax
1. Blockannotationen

   | Annotation    | Bedeutung                                                                                                                                                                                                                                                                               |
   |:--------------|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
   | aquascope     | Mit dieser Annotation aktiviert man den Aquascope Präprozessor auf dem Codeblock                                                                                                                                                                                                        |
   | interpreter   | Mit dieser Annotation aktiviert man die Interpreter-Zeilenannotationen.                                                                                                                                                                                                                 |
   | permissions   | Mit dieser Annotation aktiviert man die im Kapitel Visualisierung vorgestellten Berechtigungen. Dies muss zusammen mit `stepper` oder `boundaries` verwendet werden um die Visualisierung zu beeinflussen.                                                                              |
   | stepper       | Mit dieser Annotation aktiviert man das Anzeigen der zeilenweisen Stepper-Boxen ![stepperBox.png](pictures/stepperBox.png). Benötigt die Annotation `permissions`.                                                                                                                      |
   | boundaries    | Mit dieser Annotation aktiviert man das Anzeigen der erwarteten Berechtigungen ![expectRead.png](pictures/notation/expectRead.png),![expectWrite.png](pictures/notation/expectWrite.png), ![expectOwner.png](pictures/notation/expectOwner.png). Benötigt die Annotation `permissions`. |
   | shouldFail    | Mit dieser Annotation können Codeblöcke mit dem `interpreter` verarbeitet werden, die nicht kompilierbar sind.                                                                                                                                                                          |
   | horizontal    | Mit dieser Annotation lässt sich die Ausrichtung der Stack- und Heap-Darstellung des `interpreter` von vertikal (default) zu horizontal ändern.                                                                                                                                         |
   | concreteTypes | Mit dieser Annotation lassen sich in der Stack- und Heap-Darstellung des `interpreter` zusätzlich die konkreten Datentypen anzeigen.                                                                                                                                                    |

2. Zeilenannotationen

   | Annotationen  | Bedeutung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
   |:--------------|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
   | `#`           | Mit dieser Annotation am Zeilenanfang lassen sich einzelne Zeilen in der Ausgabe verstecken. Zur evaluierung werden diese trotzdem verwendet.<br>Beispiel: `` #fn main() { ``                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
   | `` `[]` ``    | Mit dieser Annotation am Zeilenende lassen sich einzelne Zeilen in der Ausgabe zu der Stack- und Heap-Darstellung hinzufügen. Benötigt die `interpretet` Blockannotation.<br>Beispiel: `` let n = &v[0];`[]` ``                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
   | `` `{}` ``    | Mit dieser Annotation am Zeilenende lassen sich in einzelnen Zeilen in der Ausgabe die Symbole der erwarteten Berechtigungen zu ihren alphabetischen Gegenstücken ändern. Benötigt die `boundaries` und `permissions` Blockannotation.<br>Beispiel: `` let n = &v[0];`{}` ``                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
   | `` `(...)` `` | Mit dieser Annotation am Zeilenende lässt sich das Anzeigeverhalten der Stepper-Boxen anpassen. Dabei wird zwischen drei unique keys unterschieden, welche durch `,` und ohne Leerzeichen voneinander getrennt werden.<br><br> <ul><li>`focus`: Nur Zeilen die mit diesem Key annotiert sind zeigen in der Ausgabe die Stepper-Box an. Wenn keine Zeilen damit annotiert sind werden Alle angezeigt.</li><li>`paths`: Nur die Variable die mit diesem Key gekennzeichnet ist (Beispiel: `` `(paths:x)` ``) wird in der Stepper-Box per default angezeigt. Alle anderen Variablen sind dann in einem ausklappbaren Menü verborgen.</li><li>`rxpaths`: Ist von der Funktionalität gleich wie der Key `paths`, nur das der String hinter dem `:` hierbei als Regex evaluiert wird. Somit ist hiermit eine Auswahl mehrerer Variablen möglich.</li></ul><br>Benötigt die `stepper` und `permissions` Blockannotation.<br>Beispiel: `` let n = &v[0];`(focus,paths:v)` `` |

### Verwendung des Docker Containers
Zum Erstellen der Visualisierung mithilfe des Docker Containers müssen die Markdown Quell-Files mit den annotierte Aquascope-Codeblöcken im Ordner [/tools/aquascope/markdowns](markdowns) oder einem beliebigen Unterordner abgelegt werden und danach der folgende Befehl ausgeführt werden.
````shell
docker compose up aquascope
````
<br>

## Evaluierung
1. Vorteile
    - Playground: Die Visualisierung findet automatisch statt. Dabei muss man über kein Wissen zu Annotation verfügen und kann den eigenen Code per Copy-and-paste evaluieren lassen.
    - Lokale Installation: Auch hier wird die Visualisierung größtenteils automatisch vorgenommen. Es wird zwar eine extra annotation des Codes benötigt, aber meist langt dafür die oben im Beispiel angeführte Standard-Blocknotation.
    - Unausführbarer Code: Aquascope setzt für die Visualisierung nicht voraus, dass der Code kompilierbar ist.
2. Probleme
    - Es ist nicht besonders trivial Ownership-Fehler aus der Darstellung von Aquascope abzuleiten. Beispiel:
        ````rust
        fn main() {
            let mut a = 1;
            foo(&mut a);
        }
        
        fn bar(x: &mut i32) {}
        fn foo(a: &mut i32) {
             let y = &a;
             bar(a);
             println!("{}", y);
        }
        ````
        > Vom Borrow-Checker gefundener Fehler:
        ````shell
            error[E0502]: cannot borrow `*a` as mutable because it is also borrowed as immutable
              --> src/main.rs:12:5
               |
            11 |     let y = &a;
               |             -- immutable borrow occurs here
            12 |     bar(a);
               |     ^^^^^^ mutable borrow occurs here
            13 |     println!("{}", y);
               |                    - immutable borrow later used here
            
        ````
        > Das Äquivalent lässt sich aus der Darstellung von Aquascope nur aus der Darstellung der erwarteten Berechtigungen ablesen. Hier ist ein Fehler zu erkennen, da bei <code>bar(a)</code> sowohl die Lese- als auch Schreibberechtigung erwartet wird, aber wie durch den blauen Kreis symbolisiert die Schreibberechtigung nicht gegeben ist.
    
        ![ownership_error.png](pictures/ownership_error.png)
    - Es werden nicht immer alle Codezeilen im Playground annotiert. Manchmal wird aber auch die Annotation mehrerer Zeilen zusammengefasst. Dies macht es unübersichtlich oder oft auch einfach unverständlich.
3. Fazit <br>
Das Tool bietet durch die automatisierte Erkennung und Visualisierung der Berechtigungen durchaus Vorteile gegenüber einer reinen Shell basierten Fehlerauswertung, jedoch ist im momentanen Zustand das Erkennen von Ownership- und Borrowing-Fehlern nicht trivial. Zudem kann es durchaus hilfreich sein sich zum besseren Verständnis zusätzlich die Fehlermeldungen des Borrow-Checkers anzuschauen. Das Tool kann aber trotzdem zum besseren Verständnis von Ownership und Borrowing eingesetzt werden. Da das Tool aber noch aktiv entwickelt wird, kann es durchaus sein, dass die Probleme mit der Darstellung in zukünftigen Versionen behoben werden.
