# Mill

Mill is a compiler based on LLVM-10.

```
fn greaterThan(a,b) {
  match
        a >= b  -> return a; break; 
        _       -> return b; break; 
  end;
}

fn main() {
 let a = 10;
 let b = 5;
 return greaterThan(a,b);
}
```

it supports ...

* addition, subtraction, multiplication
* if statements
* loops
* function calls
