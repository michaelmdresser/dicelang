# WASM dicelang

WASM-ready version of dicelang for browser usage.

## Build
``` sh
wasm-pack build --no-typescript --target web
```

## Use direcly via JS

Place the `.js` and `.wasm` files in a directory (e.g. `dicelang`) on the web
server. Then use a snippet like this to load and use:

```html
<script type="module">
import init, { roll } from './dicelang/dicelang_wasm.js';
async function run() {
  await init();
  let {result, rolls} = roll("d20+1d4-1d6+3");
  console.log("result:", result);
  console.log("rolls:", rolls);
}
run();
</script>
```

Reference for the above: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html 
