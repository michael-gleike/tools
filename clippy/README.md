# Clippy
Clippy ist ein Linter der meistens beim Verwenden von ``rustup`` bereits mit installiert wird.

## Benutzung
Cargo check kann ganz einfach per Shell-Command gestartet werden.
````shell
cargo clippy
````
Dabei gibt es noch verschiedene Optionen, die sich in der [Dokumentation](https://doc.rust-lang.org/clippy/usage.html) nachlesen oder per Aufruf einenr der folgenden Shell-Commands anzeigen lassen.
````shell
cargo clippy -h
cargo clippy --help
````

## Ownership und Borrowing
Clippy verwendet zum Testen von Ownership und Borrowing ``cargo check``. Somit sind die Einschränkungen der beiden Cargo Subcommands hier gleich. Genaueres dazu im Kapitel zu cargo check.