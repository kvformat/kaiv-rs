# kaiv - a Kv Format Swiss-Army knife

`kaiv` is a Rust CLI tool using the [`kvf`](https://crates.io/crates/kvf) parser and providing
querying, validation, and import/export functionality for
[Kv Format](https://kvformat.org/) files.

## Installation

```
cargo install kaiv
```

## Usage

All commands accept an optional `[FILE]` argument. If omitted or set to `-`, input is read from
stdin.

### `kaiv get <KEY> [FILE]`

Print the value for the given key. Exits with code 1 if the key is not found, 2 on parse error.

```
kaiv get APP_NAME config.kv
cat config.kv | kaiv get APP_NAME
```

### `kaiv check [FILE]`

Validate a KV file. Prints nothing and exits 0 on success; prints all parse errors to stderr and
exits 1 on I/O error, and exits 2 on parse error.

```
kaiv check config.kv
cat config.kv | kaiv check
```

### `kaiv fmt [FILE]`

Re-output a KV file in canonical form: KV entries only (comments and blank lines stripped), keys
sorted alphabetically, one `KEY=value` entry per line. Exits with code 2 on parse error.

```
kaiv fmt config.kv
cat config.kv | kaiv fmt
```

### `kaiv export json [FILE]`

Convert a KV file to a JSON object with keys sorted alphabetically. Exits with code 2 on parse
error.

```
kaiv export json config.kv
cat config.kv | kaiv export json
# {"APP_NAME":"My Application","DEBUG":"true"}
```

### `kaiv import json [FILE]`

Convert a flat JSON object (string values only) to KV format, with keys sorted alphabetically. Keys
must match `[A-Za-z_][A-Za-z0-9_]*`. Exits with code 1 on invalid input.

```
kaiv import json data.json
cat data.json | kaiv import json
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Logical error (key not found, invalid import input) |
| 2 | KV parse error |
