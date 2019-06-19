# COIN_SORT

Training project written in Rust and compliled to Web Assembly.

You pass a positive integer to the WASM module and it randomly generates that many US coins.

It then prints to the browser console how many of those coins are State quarters, and the total value of the purse.
  
  
## Compilation

`wasm-pack build <your_npm_user_nmae>`

This will generate a `pkg/` directory.  Change into this directory and publish the package using npm

`npm publish --access=public`
  
  
## Configuration

To change the random number passed to the WASM module, edit the file `./site/index.js` and change the integer passed to function `wasm.main()` on line 3

```js
const wasm = import("./node_modules/@chris_whealy/coin_sort/coin_sort.js")

wasm.then(wasm => wasm.main(123))
```
  
  
## Execution

Change into the `../site` directory and install the npm modules

`npm install`

The run the webpack Web Server using

`npm run serve`
  
  
## Output

Open your browser and visit <http://localhost:8080>.  Nothing will be visible on the HTML screen since the output is written to the console.  Open your browser's dev tools to see this output.
