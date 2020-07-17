# rust-practice

## Cargo Commands

```bash
$ cargo create package_name
$ cargo build
$ cargo check
$ cargo run
```

## Ownership

* Assignment is basically **ownership transfer**
  * Like move constructor in C++
* If `Copy` trait is implemented, the value will be copied
  * Like copy constructor in C++
  * Primitive types have `Copy` trait
* To **borrow** ownership, use reference
  > At any given time, you can have either one mutable reference or any number of immutable references.

## Managing Projects

* Package
  * Defined by a single `Cargo.toml`
  * Consists of one or more crates
  * Zero or one library crates
  * No limit on binary crates
* Crate
  * `src/main.rs`: a binary crate with the same name as the package
  * `src/lib.rs`: a library crate with the same name as the package
  * and you can add more
* Module
  * Define by `mod name { ... }`
  * Form a module tree rooted with `crate`
  * Bring into scope with `use ...`
