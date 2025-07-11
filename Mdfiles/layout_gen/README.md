# layout_gen (Rust)

This is a Rust binary that generates a JSON file (`layout_rust.json`) representing the folder and file structure of the Mdfiles directory.

## Build and Run

```
cd Mdfiles/layout_gen
cargo build --release
cd ..
./layout_gen/target/release/layout_gen
```

The output file `layout_rust.json` will be created in the `Mdfiles` directory. 