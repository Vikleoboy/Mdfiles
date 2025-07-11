# Mdfiles

This repository contains markdown files organized into folders such as `projects` and `blogs`. The structure is automatically mapped to a JSON file for use in frontend applications. 

## Generating the Layout JSON

### Python

To generate the layout JSON using Python:

```sh
cd Mdfiles
python3 generate_layout.py
```

This will create `layout_python.json` in the `Mdfiles` directory.

### Rust

See `layout_gen/README.md` for details on building and running the Rust version.

### GitHub Actions

A GitHub Actions workflow is set up to automatically run both the Python and Rust scripts on every push to `main`. The generated JSON files are committed back to the repository automatically. 