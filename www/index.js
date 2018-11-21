import "./style.css";
import * as wasm from "lox";
import CodeFlask from 'codeflask';
import { programs } from './programs.js';

let options = document.getElementById("options");
let selected = document.getElementById("selected");

let dropdown = document.getElementById("dropdown");
dropdown.addEventListener("mouseover", () => {
  dropdown.classList.add("hover");
});
dropdown.addEventListener("mouseout", () => {
  dropdown.classList.remove("hover");
})


let select_program = (prog) => {
  dropdown.classList.remove("hover");
  selected.innerText = prog;
  editor.updateCode(programs[prog]);

  while (options.firstChild) {
    options.removeChild(options.firstChild);
  }

  for (let key in programs) {
    if (prog === key) {
      continue;
    }
    let option = document.createElement("a");
    option.setAttribute("href", "#");
    option.addEventListener("click", () => {
      select_program(key);
    });
    option.innerText = key;
    options.appendChild(option);
  }
}

const editor = new CodeFlask("#code", {
  language: "js",
  lineNumbers: true,
});

const output = new CodeFlask("#output", {
  readonly: true,
});


let prog_output = [];

export let print_js = (text) => {
  prog_output.push(text);
  let all = prog_output.join("\n");
  output.updateCode(all);
}

let runButton = document.getElementById("run");
runButton.addEventListener("click",  () => {
  const code = editor.getCode();
  output.updateCode("");
  prog_output = [];
  wasm.run(code);
});

select_program("Hello World");
