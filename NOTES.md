# Notes — ownership, borrowing, lifetimes

## Ownership

# What does it mean to own a value? What are the rules?
Ownership means you're the one responsible for that value's memory — only one variable owns a given value at a time, and when that owner's scope ends, the value is cleaned up. The rules: each value has exactly one owner; there's only one owner at a time; when the owner goes out of scope, the value is dropped.

# What happens when the owner goes out of scope?
Rust automatically calls drop on the value at the closing } of the owner's scope, freeing whatever memory it held (heap allocation, file handles, etc.). No one "frees" it manually — the compiler inserts the cleanup call for you at compile time, based on scope.

# Why does Rust do this instead of a garbage collector?
A GC works by scanning memory at runtime to find what's unused, which costs CPU cycles and makes pause times unpredictable. Rust's ownership rules let the compiler figure out, entirely at compile time, exactly when each value's memory should be freed — so there's zero runtime cost and no GC pauses, while still avoiding the manual-free() bugs (double frees, use-after-free) that languages like C leave to the programmer.

## Borrowing

# What is a borrow? Does the owner give up ownership?
A borrow is a reference (&value) — a way to access data without taking ownership of it. No, the owner keeps ownership; the borrow is temporary and the original owner is still responsible for cleanup when its scope ends.

# Difference between &T and &mut T?
&T is a shared/immutable reference — you can read the value but not change it, and any number of &T can exist at once. &mut T is a mutable reference — you can modify the value through it, but only one can exist at a time, and it excludes any &T to the same value for as long as it's active.

# Why can't you hold &mut T and &T to the same value at once? Name the bug this prevents.
Because a reader can't trust the value to stay the same while it's reading if someone else might be writing to it at the same moment. This rule prevents data races — specifically the case where two or more pointers access the same data simultaneously, at least one is writing, and there's no synchronization. Rust catches this at compile time instead of letting it become a runtime memory-corruption bug.

## Lifetime example 

# Paste the function from examples/lifetimes.rs that needed the explicit annotation, in a fenced rust block.

```rust
fn pick_higher<'a>(a: &'a Task, b: &'a Task) -> &'a Task {
    if a.priority >= b.priority {
        a
    } else {
        b
    }
}
```

# what does the annotation tell the compiler? 
What the annotation tells the compiler: it unifies a, b, and the return value under one named lifetime, 'a, so the compiler can treat them as a single group with one shared validity guarantee instead of three independent, unrelated references.

# Then the nuance: is the annotation saying the return value lives as long as the shorter of the inputs, or something else? 
The precise nuance: the annotation doesn't say the return value lives as long as the shorter of the two inputs — it says the compiler will enforce validity for whatever concrete lifetime gets substituted for 'a at the call site, and that concrete lifetime is the overlap (the shorter) of a's and b's actual scopes. So "shorter of the two" is the real-world consequence, not the literal content of the annotation — the annotation itself just says "these three are the same 'a"; it's the compiler's substitution rule (pick the lifetime that satisfies every usage, which is necessarily the smaller one) that produces the "shorter wins" behavior you observe.
