# Requirements
- Bazel 5.2.0 

# Build
```shell
$ CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index 
$ bazel test //src/B:B-test  
error[E0432]: unresolved import `A::foo`
  --> src/B/src/main.rs:14:9
   |
14 |     use A::foo;
   |         ^^^^^^ no `foo` in the root
```