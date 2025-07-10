# `ast-utils` Command

The `ast-utils` command generates and displays the Abstract Syntax Tree (AST) representation of Rust source files in
JSON format.  
It uses the `syn` crate to parse Rust code and `syn-serde` for JSON serialization.

---

## Usage

```bash
cargo run -- ast-utils --file-path ./src/main.rs
```

**Arguments:**

- `--file-path` (or ): Path to the Rust source file to parse `-f`

## Behavior

The tool performs the following operations:

1. **File Reading**: Reads the specified Rust source file
2. **AST Parsing**: Uses `syn::parse_file()` to generate the AST
3. **JSON Output**: Converts the AST to pretty-printed JSON using `syn-serde`

The output includes detailed structural information about:

- Function definitions
- Struct and enum declarations
- Import statements
- Type definitions
- Expression trees
- And all other Rust language constructs

## Output

The command outputs a JSON representation of the AST directly to stdout. The JSON contains:

- **Structural Information**: Complete syntax tree with all language constructs
- **Pretty Formatting**: Human-readable JSON with proper indentation
- **Comprehensive Details**: All tokens, spans, and syntactic elements

## Example

``` bash
cargo run -- ast-utils --file-path examples/simple_program.rs
```

This would output something like:

``` json
{
  "shebang": null,
  "attrs": [],
  "items": [
    {
      "Fn": {
        "attrs": [],
        "vis": {
          "Public": {
            "pub_token": {
              "span": {
                "start": 0,
                "end": 3
              }
            }
          }
        },
        "sig": {
          "constness": null,
          "asyncness": null,
          "unsafety": null,
          "abi": null,
          "fn_token": {
            "span": {
              "start": 4,
              "end": 6
            }
          },
          "ident": "main",
          // ... more AST structure
        }
      }
    }
  ]
}
```

## Use Cases

- **Code Analysis**: Understanding the structure of Rust source code
- **Tooling Development**: Building custom analysis tools that work with Rust AST
- **Educational**: Learning about Rust's syntax tree representation
- **Debugging**: Inspecting how the compiler parses your code

## Related

- [SAST](./sast.md) — Static analysis that operates on similar AST structures
- [Build](./build.md) — Compiles the source files that can be analyzed with ast-utils
