/* --- BEGIN Variable Definitions ---
Owner x;
Owner y;
Owner s1;
Owner s1_clone;
Owner s2;
Owner s3;
Owner string1;
Owner string2;
Function println!();
Function String::from();
Function clone();
Function take_ownership();
Function take_ownership_and_give_Back();
--- END Variable Definitions --- */
fn main(){
    let x = 5; // x = 5 !{ Bind(x) }
    let y = x; // y = 5 !{ Copy(x->y) }
    // Speicher im Stack wird kopiert
    println!("x:{} y:{}", x, y); // !{ PassByStaticReference(x->println!()), PassByStaticReference(y->println!()) }

    // s1 ist Pointer zu in Heap angelegtem String Object
    let s1 = String::from("String1"); // s1 -> "Hello World" !{ Move(String::from()->s1) }

    let s1_clone = s1.clone(); // Kopie des Speicherbereichs von "Hello World!" wird angelegt !{ Copy(s1->s1_clone) }
    println!("s1 -> {}, s1_clone -> {}", s1, s1_clone); // !{ PassByStaticReference(s1->println!()), PassByStaticReference(s1_clone->println!()) }
    let s2 = s1; // s2 -> "String1" s1 ungültig !{ Move(s1->s2) }
    // Ownership des Heap Speicherbereichs wurde auf s2 übertragen

    println!("s2 -> {}", s2); // !{ PassByStaticReference(s2->println!()) }
    take_ownership(s2); // Ownership wird an Methode übertragen !{ Move(s2->take_ownership()) }
    // println!("s2-> {}", s2); // Funktioniert nicht da s2 nicht mehr Owner ist

    let s3 = take_ownership_and_give_Back(s1_clone); // !{ Move(s1_clone->take_ownership_and_give_Back()), Move(take_ownership_and_give_Back()->s3) }
    println!("s3 -> {}", s3); // !{ PassByStaticReference(s3->println!()) }

} // Stack wird freigegeben. Speicher auf den s2 und s3 zeigen wird freigegeben !{ GoOutOfScope(x), GoOutOfScope(y), GoOutOfScope(s1), GoOutOfScope(s1_clone), GoOutOfScope(s2), GoOutOfScope(s3) }

fn take_ownership(string1: String){ // !{ InitOwnerParam(string1) }
    println!("take_ownership string -> {}", string1); // !{ PassByStaticReference(string1->println!()) }
} // string Variable out of scope --> Speicher wird freigegeben !{ GoOutOfScope(string1) }

fn take_ownership_and_give_Back(string2: String) -> String { // !{ InitOwnerParam(string2) }
    println!("take_ownership_and_give_back string -> {}", string2); // !{ PassByStaticReference(string2->println!()) }
    string2 // Return Wert gibt Ownership zurück !{ Move(string2->None) }
} // !{ GoOutOfScope(string2) }