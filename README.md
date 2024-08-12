# RouilleDB

RouilleDB is a personal project aimed at learning both the Rust programming language and the inner 
workings of database engines. The primary objective is to implement a key-value store using a B+Tree 
file structure similar to SQLite. A SQL/Relational database layer may be implemented over this 
key-value storage engine in the future.

## How to build and test

```bash
cargo build
cargo test
```

## Change log

The change log can be found in the [CHANGELOG.md](CHANGELOG.md) file.

## Versionning

Although RouilleDB is not intended for production use, the following versioning scheme will be
followed:

RouilleDB version numbers use the format: v[MAJOR].[MINOR].[PATCH] (example: v1.2.3)

- MAJOR is incremented for every major release. There may be breaking changes between major  
  releases.
- MINOR is incremented each time a new feature is implemented, an existing feature is improved or  
  removed.
- PATCH is optional. It is present and incremented when changes are made without significantly  
  modifying features (e.g., bug fixes). The value '0' is never used.
