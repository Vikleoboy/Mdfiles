# Mdfiles

This repository contains markdown files organized into folders such as `projects` and `blogs`. The structure is automatically mapped to a JSON file for use in frontend applications. 

## Generating the Layout JSON

### Rust

To generate the layout JSON using Rust:

```sh
# Build the Rust binary (only needed once or after changes)
cd layout_gen
cargo build --release
cd ..

# Run the binary from the project root
./layout_gen/target/release/layout_gen
```

This will create `layout_rust.json` in the project root directory. 