# EasyZ3:

This is a simplified API to get started with the z3 SAT solver and solve easy problems. It is not intended to fully replace the z3 and z3-sys API. They are complicated for a reason.



## Motivation: 

Using the official z3 bindings, this line adds a simple constraint to the z3 solver:

```rust
solver.assert(&Int::_eq(&Int::add(&ctx,&[&Int::add(&ctx,&[&Int::mul(&ctx, &[&a, &a]),&Int::mul(&ctx,&[&Int::from_i64(&ctx, 7), &a],),],),&Int::from_i64(&ctx, 8),],),&Int::from_i64(&ctx, 268),),);
```

Here is the same constraint in easyz3:
```rust
z3_constraint!(a*a + 7*a + 8 == 268);
```

If you prefer the second syntax, this crate is for you.


## Examples:

Here is a simple example with easyz3:

```rust
// initialize z3 and declare symbolic variable "a"
z3_init!(a);

// add constraints
z3_constraint!(a*a + 7*a + 8 == 268);  

// find solution
if let Some(a) = z3_solve!(a) {
    println!("solution:  a:{} ", a);
}
```

---

If a problem has more than one solution, you can iterate over solutions, which the regular z3 API does not natively support:
```rust
z3_init!(a, b, c);

z3_distinct!(a,b,c); // no two values can be the same 
z3_constraint!(a >= 1 && a <= 3); // all between 1 and 3
z3_constraint!(b >= 1 && b <= 3);
z3_constraint!(c >= 1 && c <= 3);

// Here we can iterate over all possible solutions
// please note that each solution will take longer than the last
while let Some((a, b, c)) = z3_solve!(a, b, c) {
    println!("example3: a:{} b:{} c:{}", a, b, c);
}
```

---

You can use external i64 variables by prefixing them with "::":
```rust
fn is_prime(potential_prime: i64) -> bool {
    z3_init!(a, b); // symbolic variables

    z3_constraint!(a * b == ::potential_prime); // prefix "::" means concrete i64 variable
    z3_constraint!(a > 1); // we only want positive solutions
    z3_constraint!(b > 1);

    if let Some((a, b)) = z3_solve!(a, b) {
        println!("{} is not a prime as it can be expressed as {}*{}",potential_prime, a, b);
        false
    } else {
        println!("{} is a prime", potential_prime);
        true
    }
}
```

---
You can use ```z3_distinct!()``` to say that no two variables can have the same value:  

```rust
    z3_distinct!(a,b,c);  // shorthand for:
    z3_constraint!(a != b && b != c && a != c);
```

# Bitvecs:

By default, easyz3 uses z3::Int, which has infinite size (even though easyz3 uses i64 on the rust size to interface with it).

But sometimes we need to reason about "bitvecs", which is the z3 name for unsigned ints of fixed size.
What you call "u8" in rust, Mathematicians would call the ring Z256, and z3 call it BV(8).
There, 255+1 == 0.

easyz3 supports the common sizes 8,16,32 and 64 and pairs them with the rust types u8,u16,u32 and u64.

```rust
z3_init_u8!(a,b);
z3_constraint_u8!(a + 1 == 0);
let const_55:u8 = 0x55; // when working with 8 bit integers, input and output are rust u8 
z3_constraint_u8!(b == a ^ ::const_55); // :: prefix means external int variable, to distinguish from z3 symbolic vars

if let Some((a, b)) = z3_solve_u8!(a, b) {
    // a and b are rust u8
    println!("example6:  a: 0x{:x} b: 0x{:x}", a, b);
    assert_eq!(a,0xff);  // in Z256 world, 0xff + 1 == 0  
    assert_eq!(b,0xaa);
}
```