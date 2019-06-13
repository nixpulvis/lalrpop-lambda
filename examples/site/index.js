import("./node_modules/lalrpop-lambda/lalrpop_lambda.js").then(wasm => {
  let input = document.getElementById('input');

  let parse_output = document.getElementById('parse');
  let norm_output = document.getElementById('norm');
  let eta_output = document.getElementById('eta');
  let numeral_output = document.getElementById('numeral');
  let bool_output = document.getElementById('bool');

  function display([parse, norm, eta, numeral, bool]) {
    parse_output.innerHTML = parse;
    norm_output.innerHTML = norm;
    eta_output.innerHTML = eta;
    numeral_output.innerHTML = numeral;
    bool_output.innerHTML = bool;
  }

  function change() {
    try {
      display([null, null, null, null, null]);
      let exp = new wasm.Exp(input.value);
      display([
        exp,
        exp.normalize(false),
        exp.normalize(true),
        exp.toNumber(),
        exp.toBool(),
      ]);
      parse_output.className = null;
    } catch(e) {
      parse_output.innerHTML = e;
      parse_output.className = "error";
    }
  }

  change()
  document.getElementById('input').addEventListener('keyup', change);;
});
