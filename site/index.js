const wasm = import("./node_modules/@chris_whealy/coin_sort/coin_sort.js")

wasm.then(wasm => wasm.main(123))
