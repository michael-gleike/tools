# Seminar RustViz

*RustViz* ist ein Werkzeug, das interaktive Visualisierungen von einfachen Rust-Programmen erzeugt, um den Benutzern zu helfen, den Rust [Lifetime and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) Mechanismus besser zu verstehen.

RustViz ist ein Projekt des [Future of Programming Lab](http://fplab.mplse.org/) an der Universität von Michigan.

Diese Seminararbeit beschäftigt sich mit der Vorstellung des Tools und einer genauen Beleuchtung der [Ownership Types](https://github.com/PhilKrau/OwnershipTypesRust).

## Was macht RustViz?

*RustViz* erzeugt [SVG](https://developer.mozilla.org/en-US/docs/Web/SVG)-Dateien durch vom Benutzer annotierten Source Code mit grafischen Indikatoren, die mit [mdbook](https://github.com/rust-lang/mdBook) integriert werden können, um interaktive Visualisierungen von Eigentums- und Ausleihvorgängen in einem Rust-Programm zu erstellen. Hier ist eine Beispielansicht, wie eine Visualisierung aussehen kann:
![alt tag](https://github.com/michael-gleike/rustviz/blob/main/src/svg_generator/example.png)

Näheres dazu ist nochmal in dem zugehörigen [VL/HCC 2022 paper](https://web.eecs.umich.edu/~comar/rustviz-vlhcc22.pdf) beschrieben.

## Voraussetzungen
RustViz benötigt eine Instalation von Rust zusammen mit Cargo und mdBook.
1. [Cargo](https://doc.rust-lang.org/cargo/index.html) ist der Rust-Paketmanager. Cargo lädt die Abhängigkeiten der Rust-Pakets herunter, kompiliert die Pakete, erstellt verteilbare Pakete und lädt sie auf crates.io, die Paketregistrierung der Rust-Community, hoch.
2. [mdBook](https://github.com/rust-lang/mdBook) ist ein Kommandozeilenwerkzeug zum Erstellen von Büchern mit Markdown. Es ist ideal für die Erstellung von Produkt- oder API-Dokumentation, Tutorials, Kursmaterialien oder alles, was eine saubere, leicht navigierbare und anpassbare Präsentation erfordert. Näheres dazu ist in der [Dokumentation](https://rust-lang.github.io/mdBook/index.html) von mdBook zu finden.

## Benutzung
Nach der Instalation der Abhängigkeiten können die vorgefertigten Beispiele aus dem [/src/examples](src/examples) Ordner durch das Ausführen eines Scripts aus dem Ordner [/rustviz_mdbook](rustviz_mdbook) die Visaulisierungen erzeugt werden.

Linux-Shell:
```shell
~/rustviz/rustviz_mdbook$ ./view_examples.sh
```
Windows-Cmd:
```shell
~/rustviz/rustviz_mdbook> ./view_examples.bat
```

Nach dem Ausführen des entsprechenden Befehls lässt sich dann die erzeugte Visualisierung unter http://localhost:8000/ ansehen.


## Ordnerstruktur der Beispiele
```
beispiel
├── input
│   └── annotated_source.rs
├── main.rs
├── source.rs
├── vis_code.svg
└── vis_timeline.svg
```
- ``source.rs`` ist der nicht annotierte Source Code. In dieser Datei sind keine weiteren Änderungen zu machen.


- ``annotated_source.rs`` muss manuell erstellt werden und Annotation müssen ebenfalls manuell vorgenommen werden. Diese Datei dient als Grundlage der Darstellung des stilisierten Codes. Somit muss man auf die Formatierung oder Kommentare in den anderen Dateien nicht achten. Die [Syntax](#annotations-syntax-der-annotated_sourcers) wird gesondert beschrieben. 


- ``main.rs`` wird nach dem ersten Ausführen des ``view_examples`` Scripts automatisch erstellt, sofern die Annotationen in ``annotated_source.rs`` bereits vorgenommen wurden. Dabei wird ein Block mit Variablen Definitionen erzeugt, die eigentlichen Events müssen aber Händisch eingetragen werden. Bei der automatischen Variablen Definition gibt es aber Probleme beim erkennen von StaticRef und MutableRef. Diese werden meist bei der Generierung der Variablen Definition als Owner angegeben, was zu einer falschen Visualisierung führt. Die [Syntax](#syntax-der-mainrs) der Events und Variablen Definition wird gesondert beschrieben.


- ``vis_code.svg`` ist der aus der ``annotated_source.rs`` generierte stilisierte Codes.


- ``vis_timeline.svg`` ist die aus der ``main.rs`` generierte Timeline der Events.


## Annotations Syntax der ``annotated_source.rs``
```rust
// Beispielcode
fn main() {
    let <tspan data-hash="1">x</tspan> = <tspan class="fn" data-hash="0" hash="3">String::from</tspan>("hello");
    let <tspan data-hash="2">y</tspan> = <tspan data-hash="1">x</tspan>;
    <tspan class="fn" data-hash="0" hash="4">println!</tspan>("{}", <tspan data-hash="2">y</tspan>);
}
```
1. Annotiert werden nur Variablen und der Funktionsname. Dazu wird das jeweilige Element von ``<tspan></tspan>`` gekapselt.
2. ``class="fn"``  symbolisiert für ``main.rs`` dass ``String::from`` eine Funktion ist
3. ``data-hash="0"`` beeinträchtigt die Formatierung
   1. ``"0"`` formatiert den in ``<tspan></tspan>`` angegebenen code in grau, wird nur bei Funktionen verwendet
   2. ``"1"`` formatiert in blau
   3. ``"2"`` formatiert in gelb
   4. ``"3"`` formatiert in lila
   5. ...
   6. ``"9"`` ist der momentan letzte vorgesehen ``data-hash``
4. ``hash="4"`` wird nur bei ``data-hash="0"`` benötigt, sonst ist ``hash`` implizit der selbe wert wie ``data-hash``. Der angegebene Wert bestimmt dabei die Position in der Variablen Definition in der ``main.rs``. Variablen haben dabei immer die kleineren Werte als Funktionen und die Reihenfolge in der Variablen Definition bestimmt dabei auch die Reihenfolge in der Graphen Visualisierung. Der ``data-hash`` bestimmt dabei aber lediglich die Farbe des Elements der entsprechenden Nummer. Somit kann es durch ein Inkonsistenz zwischen dem ``data-hash`` und der Position in der Variablen Definition dazu kommen, dass eine Variable im Code und im Graphen zwei verschiedene Farben haben kann.


## Syntax der ``main.rs``
### 1. Variablen Definition
Das RustViz-Tool definiert alle möglichen Eigentümer, Referenzen oder Eingaben einer Speicherressource als [ResourceAccessPoint](#Data_Structures_and_Function_Specifications).
Das Format für jeden `ResourceAccessPoint` ist in dem nachstehend Enum dargestellt, wobei Felder mit vorangestelltem ':' ein optionales Feld bezeichnen:
```rust
ResourceAccessPoint Usage --
    Owner <:mut> <name>
    MutRef <:mut> <name>
    StaticRef <:mut> <name>
    Struct <:mut> <name>{<:mut> <member_1>, <:mut> <member_2>, ... }
    Function <name>
```
Damit wird am Anfang der ``main.rs`` in einem Blockkommentar  die Variablen Definition vorgenommen.

```rust
// Beispiel
/*--- BEGIN Variable Definitions ---
Owner x; 
Owner mut y;
StaticRef z;
Struct a{b, mut c}
Function String::from();
--- END Variable Definitions ---*/
```
> Dabei ist es wichtig zu beachten:
> <ol>
> <li>alle Definitionen <strong><em>müssen</em></strong> zwischen <code>BEGIN</code> und code>END</code> liegen</li>
> <li>alle Definitionen <strong><em>müssen</em></strong> in der <strong><em>gleichen Reihenfolge</em></strong> definiert werden, in der sie durch die Annotationen in der <code>annotated_source.rs</code> durchnummeriert wurden um zu verhindern, dass es Inkonsitenzen zwischen der Formatierung des Codes und der Timeline gibt.</li>
> <li>alle Definitionen <strong><em>müssen</em></strong> durch ein einzelnes Semikolon getrennt werden</li>
> <li>jedes Feld innerhalb einer ResourceAccessPoint-Definition <strong><em>muss</em></strong> durch ein Leerzeichen getrennt sein</li>
> </ol>
<br>

### 2. Events
Events werden in strukturierten Kommentaren spezifiziert. Dabei wird jedes Event in der Zeile, in der es auftritt, und innerhalb der Begrenzungszeichen ``!{`` und ``}`` definiert.
> Events können innerhalb von Blockkommentaren annotiert werden; der Block muss jedoch in der Zeile beginnen, in der die Events auftreten. Außerdem müssen alle Ereignisse innerhalb einer <code>!{}</code>-Begrenzung durch ein einzelnes Komma getrennt werden und jeweils dem Format entsprechen:
```rust
Format: <event_name>(<from>-><to>)
  Bsp.: // !{ PassByMutableReference(a->Some_Function()), ... }
```
> Bemerkungen:
> <ol>
> <li><code>GoOutOfScope</code>, <code>InitRefParam</code> und <code>InitOwnerParam</code> benötigen nur den <code>from</code> Parameter. Bsp.: <code>// !{ GoOutOfScope(x) }</code></li>
> <li>Der <code>None</code> Typ kann als <code>to</code> Parameter verwendet werden um z.B. die Rückgabe an den Funktionsaufrufer zu signalisieren. Bsp.: <code>// !{ Move(a->None) }</code></li>
> <li>Allen Verwendungen von <code>Struct</code>-Feldern muss der Name der übergeordneten Structs vorangestellt werden. Bsp.: <code>a.b = 1; // !{ Bind(a.b) }</code> wobei <code>a</code> das übergeordnete Struct und <code>b</code> das Feld ist</li>
> </ol>

| Mögliche Events                  | Beschreibung                                                                                                                                                                                                                                                                 |
|:---------------------------------| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Bind(a)`                        | Bindung per `let` oder Zuweisung. <br>Bsp.: `let a = 1;                                                                                                                                                                                                                      |
| `Copy(a->b)`                     | Kopiert die Ressource von `a` zu `b`. Hierbei implementiert `a` das `Copy`-Trait.                                                                                                                                                                                            |
| `Move(a->b)`                     | Verschiebt die Ressource von `a` zu `b`. Hierbei implementiert `a` das `Move`-Trait. <br>Hinweis: Das Verschieben an `None` (z.B.: `Move(a -> None)`) drückt ein Verschieben an die aufrufende Funktion aus. Bsp.: Rückgabe des per `Move(a -> fn())` erhaltenen Ownerships. |
| `StaticBorrow(a->b)`             | Weist `b` einer unveränderlichen Referenz auf `a` zu. Bsp.: `let b = &a;`                                                                                                                                                                                                    |
| `MutableBorrow(a->b)`            | Weist `b` einer veränderlichen Referenz auf `a` zu. <br>e.g.: `let b = &mut a;`                                                                                                                                                                                              |
| `StaticDie(a->b)`                | Beendet die nicht-lexikalische Lebensdauer der Referenzvariablen `a` und gibt die Ressource an ihren Besitzer `b` zurück.                                                                                                                                                    |
| `MutableDie(a->b)`               | Beendet die nicht-lexikalische Lebensdauer der Referenzvariablen `a` und gibt die Ressource an ihren Besitzer `b` zurück.                                                                                                                                                    |
| `PassByStaticReference(a->b())`  | Übergibt eine unveränderliche Referenz auf die Variable `a` an die Funktion `b()`. Nicht mit `StaticBorrow` zu verwechseln.                                                                                                                                                  |
| `PassByMutableReference(a->b())` | Übergibt eine veränderliche Referenz auf die Variable `a` an die Funktion `b()`. Nicht mit `MutableBorrow` zu verwechseln.                                                                                                                                                   |
| `GoOutOfScope(a)`                | Beendet die lexikalische Lebensdauer der Variable `a`.                                                                                                                                                                                                                       |
| `InitOwnerParam(a)`              | Initialisiert den Parameter `a` einer Funktion, der Owner der Ressource ist. Bsp.: `fn(a: String) {..}`<br>                                                                                                                                                                  |
| `InitRefParam(a)`                | Initialisiert den Parameter `a` einer Funktion, der eine Referenz auf die Ressource ist. Bsp.: `fn(a: &String) {..}`<br>                                                                                                                                                     |

## Einschränkungen der Visualisierungen
- Verzweigungslogik
- Looping
- explizite Lebensdaueranmerkung

## Probleme
Es kommt immer wieder zu inkonsitentem Verhalten und die Annotationen werden nicht automatisiert angelegt. Dies führt dazu, dass man sich bereits mit der Thematik des Ownerships und Borrowing auskennen muss um diese visualisieren zu können.
Zudem gibt es Probleme beim erkennen von StaticRef und MutableRef. Diese werden meist bei der Generierung der Variablen Definition als Owner angegeben, was zu einer falschen Visualisierung führt.