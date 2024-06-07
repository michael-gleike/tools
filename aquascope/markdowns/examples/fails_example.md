# Aquascope fails to indicate the problem

```aquascope,interpreter+permissions,stepper,boundaries,shouldFail,horizontal
fn main() {
    let mut a = 1;`[]`
    foo(&mut a);`[]`
}

fn bar(x: &mut i32) {
    println!("{}", x);`[]`
}
fn foo(a: &mut i32) {
     let y = &a;`[]`
     bar(a);`[]`
     println!("{}", y);`[]`
}
```