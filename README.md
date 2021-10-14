# wgpu_segfault
A repository to test/investigate a segfault that should not occur

On my machine a `cargo run` is all that it takes for this code to segfault. Right after the `panic!()` something goes wrong in the cleanup and a segfault occurs.
