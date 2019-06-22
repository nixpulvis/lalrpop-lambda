import React from 'react';
import ReactDOM from 'react-dom';

import("./node_modules/lalrpop-lambda/lalrpop_lambda.js").then(wasm => {
  class LambdaEditor extends React.Component {
    constructor(props) {
      super(props);
      this.state = { input: '', expression: null, error: null };
      this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
      let input = event.target.value;

      try {
        let expression = new wasm.Exp(input);
        this.setState({ input, expression, error: null });
      } catch(e) {
        this.setState({ input, expression: null, error: e });
      }
    }

    render() {
      if (this.state.input === '') {
        var display = (
          <p>
            <strong>Input a valid λ-expression, e.g.</strong>
            <code>\x.x x</code>
          </p>
        );
      } else if (this.state.error) {
        var display = <LambdaError message={this.state.error} />
      } else {
        var display = <LambdaOutputs exp={this.state.expression} />
      }

      return (
        <div className="lambda">
          <textarea onChange={this.handleChange}
                    value={this.state.input}></textarea>
          {display}
        </div>
      );
    }
  }

  class LambdaOutputs extends React.Component {
    render() {
      let exp = this.props.exp;
      var outputs;
      try {
        outputs = [
          { label: 'parse', value: exp ? exp.toString() : '' },
          { label: 'app', value: exp ? exp.applicative(false).toString() : '' },
          { label: 'cbv', value: exp ? exp.call_by_value().toString() : '' },
          { label: 'norm', value: exp ? exp.normal(false).toString() : '' },
          { label: 'cbn', value: exp ? exp.call_by_name().toString() : '' },
          // TODO: Hybrid by-value and applicative.
          { label: 'spine', value: exp ? exp.head_spine(false).toString() : '' },
          // TODO: Hybrid head spine and normal.
          { label: "= numeral", value: exp ? exp.toNumber().toString() : '' },
          { label: "= bool", value: exp ? exp.toBool().toString() : '' },
        ];
      } catch(e) {
        outputs = [];
      }
      return (
        <table>
          {outputs.map((o, i) => {
            return (<LambdaOutput key={i}
                                  label={o.label}
                                  value={o.value} />);
          })}
        </table>
      )
    }
  }

  class LambdaOutput extends React.Component {
    render() {
      return (
        <tr>
          <th>{this.props.label}</th>
          <td><code>{this.props.value}</code></td>
        </tr>
      );
    }
  }

  class LambdaError extends React.Component {
    render() {
      return (
        <p>
          <code className="error">{this.props.message}</code>
        </p>
      )
    }
  }

  ReactDOM.render(<LambdaEditor />, document.getElementById('mount'));
});
