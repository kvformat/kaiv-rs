# kaiv - a Kv Format Swiss-Army knife

kaiv is a Rust CLI tool using the kvf parser and providing querying and import/export functionality.

## Usage

Get value for key `KEY`
```
cat file.kv | kaiv get KEY
```

Export `.kv` file
```
cat file.kv | kaiv export json
```

Import JSON file
```
cat file.json | kaiv import json
```
