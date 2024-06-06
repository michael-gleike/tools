# Seminararbeit Rust Ownership Tools

Die Speicherverwaltung in Rust zeichnet sich durch einen einzigartigen Ansatz aus. Im Gegensatz zur traditionellen Garbage Collection definiert Ownership eine Reihe von Regeln dafür, wie ein Programm Speicher verwaltet. Jeder Wert in Rust hat einen einzigen Owner, und wenn dieser Owner den out of scope geht, wird der Speicher automatisch freigegeben. Dieses System sorgt für Speichersicherheit, indem es „Dangling Pointers“ und Speicherlecks verhindert. Der Ansatz der Ownership mag auf den ersten Blick als neuartig erscheinen, erweist sich jedoch bei näherer Betrachtung als mächtiges Werkzeug, das die Erstellung sicherer und effizienter Rust-Programme ermöglicht.

Diese Seminararbeit beschäftigt sich damit, Tools zu evaluieren, die beim Verstehen des Konzepts helfen können und dazu beitragen Ownership-Fehler zu vermeiden in dem sie diese Visuell oder Textuell darstellen. Genauer betrachtet werden dabei die folgenden Tools:
- [cargo check](https://github.com/michael-gleike/tools/blob/main/cargo_check/README.md) ist ein Tool zum Überprüfen von lokalen Packages und Dependencies. Zur überprüfung von Ownership und Borrowing verwendet es den Rust Borrow-Checker.
- [clippy](https://github.com/michael-gleike/tools/blob/main/clippy/README.md) ist der mit Rust zusammen vorinstallierte Linter. Zum Überprüfen von Ownership und Borrowing wird dabei cargo check verwendet.
- [rustviz](https://github.com/michael-gleike/tools/blob/main/rustviz/README.md) ist ein Tool zum Visualisieren von Ownership und Borrowing. Es führt selber keine Checks auf Richtigkeit des Codes durch.
- [aquascope](https://github.com/michael-gleike/tools/blob/main/aquascope/README.md) ist ein Tool zum Visualisieren von Ownership und Borrowing. Dabei zeigt die Visualisierung das Verhalten des Rust Borrow-Checkers und gegebenenfalls Ownership Fehler.

Eine genaue Evaluierung der Tools findet sich in den jeweiligen Kapiteln zu den einzelnen Tools. Dabei wird besonders auf die benötigten Vorkenntnisse und qualität der Ergebnisse geachtet.
