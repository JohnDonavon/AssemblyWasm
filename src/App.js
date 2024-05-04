import { useEffect } from 'react';
import init, { compile } from './rust_lib';

function App() {
  useEffect(() => {
    init().then(() => {
      try {
        console.log(compile());
      } catch (e) {
        console.error('WASM ERROR', e.stack);
      }
    });
  }, []);

  return <div className="App">
    <h1>Hello from React and Rust!</h1>
  </div>;
}

export default App;