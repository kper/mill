# Mill 

Mill is a compiler based on LLVM-10.

```
greaterThan(a,b) 
	cond 
		a >= b	-> return a; break; 
				-> return b; break; 
	end;
end;

main() 
	var a = 10;
	var b = 5;
	return greaterThan(a,b);
end;
```

it supports ...

* addition, subtraction, multiplication
* if statements
* loops
* function calls
