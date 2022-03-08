# Experiment with Categorizing items to enum

## Run

```
wink@3900x:~/prgs/rust/myrepos/expr-categorizer (main)
$ cargo run
   Compiling expr-categorizer v0.1.0 (/home/wink/prgs/rust/myrepos/expr-categorizer)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/expr-categorizer`
c1: Cat1, c2: Cat2
0: Other
c1: 1
categorize("hello", closure): Cat1
categorize("bye", categorizing): Cat2
categorize_strings"something"): Other
categorize(1, num_to_category): Cat1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
