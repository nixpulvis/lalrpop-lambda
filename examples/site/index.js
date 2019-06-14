import("./node_modules/lalrpop-lambda/lalrpop_lambda.js").then(wasm => {
  let input = document.getElementById('input');
  let output = document.getElementById('output');

  function display(text, lines) {
    var content = "<table>";
    for (let [key, value] of Object.entries(lines)) {
      content += "<tr>";
      content += `<th>${key}</th>`;
      try {
        let exp = new wasm.Exp(text);
        content += `<td><code>${value(exp)}</code></td>`;
      } catch(e) {
        content += `<td><code class="error">${e}</code></td>`;
        if (key == "-p ") {
          break;
        }
      }
      content += "</tr>";
    }
    content += "</table>"

    output.innerHTML = content;
  }

  function change() {
    display(input.value, {
      "parse":       (exp) => exp,
      "by value":    (exp) => exp.weak_normal(),      // CallByValue
      "by name":     (exp) => exp.weak_head_normal(), // CallByName
      "applicative": (exp) => exp.normal(true),       // Applicative
      "head spine":  (exp) => exp.head_normal(true),  // HeadSpine
      "= numeral":   (exp) => exp.toNumber(),
      "= boolean":   (exp) => exp.toBool(),
    });
  }

  change()
  document.getElementById('input').addEventListener('keyup', change);;
});
