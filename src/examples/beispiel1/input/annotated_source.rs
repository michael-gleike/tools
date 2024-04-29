fn main(){
    let <tspan data-hash="1">x</tspan> = 5; // x = 5
    let <tspan data-hash="2">y</tspan> = <tspan data-hash="1">x</tspan>; // y = 5
    // Speicher im Stack wird kopiert
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("x:{} y:{}", <tspan data-hash="1">x</tspan>, <tspan data-hash="2">y</tspan>);

    // s1 ist Pointer zu in Heap angelegtem String Object
    let <tspan data-hash="3">s1</tspan> = String::from("String1"); // s1 -> "Hello World"

    let <tspan data-hash="4">s1_clone</tspan> = <tspan data-hash="3">s1</tspan>.<tspan class="fn" data-hash="0" hash="10">clone</tspan>(); // Kopie des Speicherbereichs von "Hello World!" wird angelegt
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("s1 -> {}, s1_clone -> {}", <tspan data-hash="3">s1</tspan>, <tspan data-hash="4">s1_clone</tspan>);
    let <tspan data-hash="5">s2</tspan> = <tspan data-hash="3">s1</tspan>; // s2 -> "String1" s1 ungültig
    // Ownership des Heap Speicherbereichs wurde auf s2 übertragen

    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("s2 -> {}", <tspan data-hash="5">s2</tspan>);
    <tspan class="fn" data-hash="0" hash="11">take_ownership</tspan>(<tspan data-hash="5">s2</tspan>); // Ownership wird an Methode übertragen
    // println!("s2-> {}", s2); // Funktioniert nicht da s2 nicht mehr Owner ist

    let <tspan data-hash="6">s3</tspan> = <tspan class="fn" data-hash="0" hash="12">take_ownership_and_give_Back</tspan>(<tspan data-hash="4">s1_clone</tspan>);
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("s3 -> {}", <tspan data-hash="6">s3</tspan>);

} // Stack wird freigegeben. Speicher auf den s2 und s3 zeigen wird freigegeben

fn <tspan class="fn" data-hash="0" hash="11">take_ownership</tspan>(<tspan data-hash="7">string1</tspan>: String){ //
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("take_ownership string -> {}", <tspan data-hash="7">string1</tspan>);
} // string Variable out of scope --> Speicher wird freigegeben

fn <tspan class="fn" data-hash="0" hash="12">take_ownership_and_give_Back</tspan>(<tspan data-hash="8">string2</tspan>: String) -> String {
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("take_ownership_and_give_back string -> {}", <tspan data-hash="8">string2</tspan>);
    <tspan data-hash="8">string2</tspan> // Return Wert gibt Ownership zurück
}