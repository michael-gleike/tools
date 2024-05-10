fn main(){
    let x = 5;
    let y = x;
    println!("x:{} y:{}", x, y);

    let s1 = String::from("String1");

    let s1_clone = s1.clone();
    println!("s1 -> {}, s1_clone -> {}", s1, s1_clone);
    let s2 = s1;

    println!("s2 -> {}", s2);
    take_ownership(s2);

    let s3 = take_ownership_and_give_Back(s1_clone);
    println!("s3 -> {}", s3);

}

fn take_ownership(string1: String){ //
    println!("take_ownership string -> {}", string1);
}

fn take_ownership_and_give_Back(string2: String) -> String {
    println!("take_ownership_and_give_back string -> {}", string2);
    string2
}