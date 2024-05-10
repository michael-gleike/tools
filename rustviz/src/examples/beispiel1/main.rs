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
    let x = 5; // !{ Bind(x) }
    let y = x; // !{ Copy(x->y) }
    println!("x:{} y:{}", x, y); // !{ PassByStaticReference(x->println!()), PassByStaticReference(y->println!()) }

    let s1 = String::from("String1"); // !{ Move(String::from()->s1) }

    let s1_clone = s1.clone(); // !{ Copy(s1->s1_clone) }
    println!("s1 -> {}, s1_clone -> {}", s1, s1_clone); // !{ PassByStaticReference(s1->println!()), PassByStaticReference(s1_clone->println!()) }
    let s2 = s1; // !{ Move(s1->s2) }

    println!("s2 -> {}", s2); // !{ PassByStaticReference(s2->println!()) }
    take_ownership(s2); // !{ Move(s2->take_ownership()) }

    let s3 = take_ownership_and_give_Back(s1_clone); // !{ Move(s1_clone->take_ownership_and_give_Back()), Move(take_ownership_and_give_Back()->s3) }
    println!("s3 -> {}", s3); // !{ PassByStaticReference(s3->println!()) }

} // !{ GoOutOfScope(x), GoOutOfScope(y), GoOutOfScope(s1), GoOutOfScope(s1_clone), GoOutOfScope(s2), GoOutOfScope(s3) }

fn take_ownership(string1: String){ // !{ InitOwnerParam(string1) }
    println!("take_ownership string -> {}", string1); // !{ PassByStaticReference(string1->println!()) }
} // !{ GoOutOfScope(string1) }

fn take_ownership_and_give_Back(string2: String) -> String { // !{ InitOwnerParam(string2) }
    println!("take_ownership_and_give_back string -> {}", string2); // !{ PassByStaticReference(string2->println!()) }
    string2 // !{ Move(string2->None) }
} // !{ GoOutOfScope(string2) }