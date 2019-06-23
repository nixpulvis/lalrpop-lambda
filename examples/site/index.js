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
        var display = <LambdaParseError message={this.state.error} />
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
      let outputs = [
        { label: 'parse',    func: 'toString' },
        { label: 'app ->',   func: 'applicative' },
        { label: 'cbv ->',   func: 'call_by_value' },
        { label: 'norm ->',  func: 'normal' },
        { label: 'cbn ->',   func: 'call_by_name' },
        // TODO: Hybrid by-func and applicative.
        { label: 'spine ->', func: 'head_spine' },
        // TODO: Hybrid head spine and normal.
        { label: "numeral =", func: 'toNumber' },
        { label: "bool =",    func: 'toBool' },
      ];
      return (
        <table>
          <tbody>
            {outputs.map((o, i) => {
              return (<LambdaOutput key={i}
                                    label={o.label}
                                    exp={this.props.exp}
                                    func={o.func} />);
            })}
          </tbody>
        </table>
      )
    }
  }

  class LambdaOutput extends React.Component {
    render() {
      if (this.props.func) {
        try {
          var result = (<code>
            {this.props.exp[this.props.func]().toString()}
          </code>);
        } catch(e) {
          var result = <code className='error'>{e.toString()}</code>;
        }
      } else {
        var result = '';
      }
      return (
        <tr>
          <th>{this.props.label}</th>
          <td>{result}</td>
        </tr>
      );
    }
  }

  class LambdaParseError extends React.Component {
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
