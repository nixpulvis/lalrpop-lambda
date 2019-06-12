import("./node_modules/lalrpop-lambda/lalrpop_lambda.js").then(lambda => {
  function change() {
    let input = document.getElementById('lambda');
    let output = document.getElementById('norm');

    try {
      var result = lambda.normalize(input.value);
      output.className = null;
    } catch(e) {
      var result = e;
      output.className = "error";
    } finally {
      output.innerHTML = result;
    }
  }

  change()
  document.getElementById('lambda').addEventListener('keyup', change);;
});
