# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-14

### Added

- Initial release of `kaiv`, a KV Format Swiss-Army knife CLI tool.
- `get` subcommand: print the value for a given key from a KV file.
- `check` subcommand: validate a KV file and report parse errors.
- `fmt` subcommand: re-output a KV file in canonical form (sorted keys, no comments or blank lines).
- `export json` subcommand: export a KV file as a JSON object.
- `import json` subcommand: import a JSON object as KV format.
- Stdin and file input support for all subcommands (omit the file argument or pass `-` to read from stdin).
