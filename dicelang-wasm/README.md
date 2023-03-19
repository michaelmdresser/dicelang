# WASM dicelang

This is the WASM version of dicelang so dice can be evaluated in browser.

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
  function onclick() {
    let inputValue = document.getElementById("inputField").value;
    let {result, rolls} = roll(inputValue);
    console.log("result:", result);
    console.log("rolls:", rolls);
  }
  document.getElementById("rollButton").onclick = onclick;
}
run();
</script>
```

Reference for the above: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html 
