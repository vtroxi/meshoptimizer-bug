# meshoptimizer-bug

https://github.com/zeux/meshoptimizer/issues/477

A small Rust program that simplifies `input.obj` to a quarter of its triangle count using the meshopt-rs library.
Degenerate triangles can be observed in the `simplified.obj` file, causing z-fighting.