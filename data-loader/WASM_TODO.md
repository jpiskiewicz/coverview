# Moving the logic into WASM

The function that takes the most time to run currently is `parseInfo()`. It builds JS objects that are then used to lazily calculate coverage.
In order to move the logic of `parseInfo()` we'd also have to move all of these objects and their methods to Rust which is doable but takes some work.
Although this would essentially be like killing two birds with one stone since the methods used for lazy calculations will also gain a big speedup
from that move.

## Command to build and server WASM test page
```sh
wasm-pack build --target web && python -m http.server --directory .
```