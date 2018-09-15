# Canapi-StdWeb

`Fetch` implementation for WASM projects.

```rust
use canapi_stdweb::WebFetch;

use my_api;

fn main() {
    let user = my_api::users.get::<WebFetch>(1).expect("fetch user");
    println!("{:?}", user);
}
```
