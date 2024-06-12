# Aquascope playground example

```aquascope,interpreter+permissions,shouldFail,stepper,boundaries
#fn main() {
let mut v = vec![1, 2, 3];
let n = &v[0];`[]`
v.push(0);`[]`
let x = *n;`[]`
#}
```