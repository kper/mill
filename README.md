# Mill

Mill is a compiler based on LLVM-10. This project is actually a backup project idea for my undergrad bakk. thesis.

```
fn greaterThan(a: int,b: int) {
  match
        a >= b  -> return a; break; 
        _       -> return b; break; 
  end;
}

fn main() {
 let a : int = 10;
 let b : int = 5;
 return greaterThan(a,b);
}
```

```
struct Point { 
	x: int, 
	y: int 
}

fn getx() {
	let p = new Point;
	p.x = 100;
	return p.x;
}

fn main() {
 return getx();
}
```

it supports ...

* addition, subtraction, multiplication
* if statements
* loops
* function calls

However, an important feature is still in planning: heap allocation
