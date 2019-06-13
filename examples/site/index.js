import("./node_modules/lalrpop-lambda/lalrpop_lambda.js").then(wasm => {
  let input = document.getElementById('input');
  let output = document.getElementById('output');

  function display(text, lines) {
    var content = "";

    for (let [key, value] of Object.entries(lines)) {
      try {
        let exp = new wasm.Exp(text);
        content += `${key}: <code>${value(exp)}</code>\n`;
      } catch(e) {
        content += `${key}: <code class="error">${e}</code>\n`;
        if (key == "-p ") {
          break;
        }
      }
    }
    output.innerHTML = content;
  }

  function change() {
    display(input.value, {
      "-p ": (exp) => exp,
      "-bv": (exp) => exp.weak_normal(),      // CallByValue
      "-bn": (exp) => exp.weak_head_normal(), // CallByName
      "-ao": (exp) => exp.normal(true),       // Applicative
      "-he": (exp) => exp.head_normal(true),  // HeadSpine
      "=n ": (exp) => exp.toNumber(),
      "=b ": (exp) => exp.toBool(),
    });
  }

  change()
  document.getElementById('input').addEventListener('keyup', change);;
});
