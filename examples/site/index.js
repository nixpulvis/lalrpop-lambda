import("./node_modules/lalrpop-lambda/lalrpop_lambda.js").then(wasm => {
  let input = document.getElementById('input');
  let output = document.getElementById('output');

  function display(text, rows) {
    var content = "<table>";
    for (let row of rows) {
      content += "<tr>";
      for (let col of row) {
        console.log(col);
        if (typeof col === "function") {
          try {
            let exp = new wasm.Exp(text);
            content += `<td><code>${col(exp)}</code></td>`;
          } catch(e) {
            content += `<td><code class="error">${e}</code></td>`;
          }
        } else {
          content += `<th>${col}</th>`;
        }
      }
      content += "</tr>";
    }
    content += "</table>"

    output.innerHTML = content;
  }

  function change() {
    display(input.value, [
      ["parse",           (exp) => exp],
      ["applicative (N)", (exp) => exp.applicative(true)],
      ["by value (WN)",   (exp) => exp.call_by_value()],
      ["normal (N)",      (exp) => exp.normal(true)],
      ["by name (WHN)",   (exp) => exp.call_by_name()],

      // TODO: Hybrid by-value and applicative.
      // "head spine (HN)": (exp) => exp.head_spine(false),
      // TODO: Hybrid head spine and normal.

      ["= numeral", (exp) => exp.toNumber()],
      ["= boolean", (exp) => exp.toBool()],
    ]);
  }

  change()
  document.getElementById('input').addEventListener('keyup', change);;
});
