https://github.com/jgarzik/rust-protobuf-example

clone repo, then add file src/protos/FILE.proto
remove the extra annotations

then edit the build.rs file to add the new source (don't remove example or else it will fail.)
`cargo build`

rust-protobuf-example/target/debug/build/rust-protobuf-example-26ee9cb57adda940/out/protos/FILENAME.rs
move this to your src/protos in the contract