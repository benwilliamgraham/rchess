"use strict";

async function main() {
  const response = await fetch(
    "./target/wasm32-unknown-unknown/release/rchess.wasm"
  );
  const { instance } = await WebAssembly.instantiateStreaming(response, {});

  const { test_fn } = instance.exports;

  console.log(test_fn());
}

main();
