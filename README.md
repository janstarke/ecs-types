# ecs_types
Rust types mapping to the elasticsearch common schema

This crate provides basic type to be used when importing data to elasticsearch. It is generated from the type definitions in <https://github.com/elastic/ecs>.

The crates documentation can be found at <https://docs.rs/crate/ecs_types>.


## Usage example
```rust
use ecs_types::types::Timestamp;
use ecs_types::*;
use serde_json::json;

let now: Timestamp = chrono::offset::Local::now().into();
let mut base = Base::new(now.clone());
let mut file = File::default();
file.set_name("readme.txt".into());
file.set_mtime(now);

base.with_file(file);
println!("{}", serde_json::to_string_pretty(&json!(base)).unwrap() );
```

creates the following result:
```json
{
  "@timestamp": 1669822098181,
  "file": {
    "attributes": [],
    "mtime": 1669822098181,
    "name": "readme.txt"
  },
  "tags": []
}
```



License: Apache-2.0
