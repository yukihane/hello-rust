import init, { greet, is_even, sum } from "../pkg/hello_wasm.js";

const output = document.getElementById("output");
const nameInput = document.getElementById("name-input");
const greetButton = document.getElementById("run-greet");
const sumAInput = document.getElementById("sum-a");
const sumBInput = document.getElementById("sum-b");
const sumButton = document.getElementById("run-sum");
const evenNInput = document.getElementById("even-n");
const evenButton = document.getElementById("run-even");

await init();

greetButton.addEventListener("click", () => {
  const name = nameInput.value;
  output.textContent = `greet("${name}") => ${greet(name)}`;
});

sumButton.addEventListener("click", () => {
  const a = Number(sumAInput.value);
  const b = Number(sumBInput.value);
  output.textContent = `sum(${a}, ${b}) => ${sum(a, b)}`;
});

evenButton.addEventListener("click", () => {
  const n = Number(evenNInput.value);
  output.textContent = `is_even(${n}) => ${is_even(n)}`;
});
