# redacted

A drop-in replacement for `derive(Debug)` for structs with an additional `#[redacted]` marker.

Example:
```rust
#[derive(Redact)]
struct Sample {
  username: &'static str,
  #[redacted]
  password: &'static str
}

let instance = Sample {
  username: "FoseFx",
  password: "hunter2"
};

println!("{:?}", instance); // Sample { username: "FoseFx", password: "<redacted>" }
println!("{:?}", instance.password); // "hunter2"

```

MIT License
