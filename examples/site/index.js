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
      "parse":           (exp) => exp,
      "by value (WN)":   (exp) => exp.call_by_value(),
      "applicative (N)": (exp) => exp.applicative(true),
      // TODO: Hybrid by-value and applicative.
      "by name (WHN)":   (exp) => exp.call_by_name(),
      "normal (N)":      (exp) => exp.normal(true),
      "head spine (HN)": (exp) => exp.head_spine(true),
      // TODO: Hybrid head spine and normal.

      "= numeral": (exp) => exp.toNumber(),
      "= boolean": (exp) => exp.toBool(),
    });
  }

  change()
  document.getElementById('input').addEventListener('keyup', change);;
});
