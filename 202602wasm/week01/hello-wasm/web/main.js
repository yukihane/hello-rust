import init, { greet } from "../pkg/hello_wasm.js";

const output = document.getElementById("output");
const button = document.getElementById("run");

await init();

button.addEventListener("click", () => {
  output.textContent = greet();
});
