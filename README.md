# json-parser-rs
A simple json parser written in Rust

Json format is here: <a> https://www.json.org/json-en.html</a>

A trivial project used to practice Rust trait and module system.

**Features :**

+ Some useful wrap-ups, as you can see in common/usize_wrapper, which implement some traits for the struct; Such as `Deref `and` Into<usize>`

  + Support chaining operations:
    + `idx.go_ahead(bytes).trim_whitespace(bytes)` things like this

  

+ Return `Result<JsonValue, Error> `for each function, which can be used to handle some errors

+ ...

**TODO :**

+ Parsing the value as real number, currently it's just String.
+ Refactoring
+ Using some more interesting Rust Grammar
  + Lifetime specifier
  + Use typed_arena to allocate Json value
  + ...



