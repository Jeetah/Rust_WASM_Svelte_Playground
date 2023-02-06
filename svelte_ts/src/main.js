// import './app.css'
// import App from './App.svelte'
//
// const app = new App({
//   target: document.getElementById('app')
// })
//
// export default app


import App from './App.svelte';
 // generated beforehand - TODO: Watch + HMR
import wasm from '/rust-crate/pkg/rust-crate';

import "carbon-components-svelte/css/g80.css";

// Wasm imports needs to be done asynchronously
const init = async () => {
  const bindings = await wasm();
  const app = new App({
    target:  document.body,
    props: {
      bindings,
    },
  })
}

init()