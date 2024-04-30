import { useEffect } from 'react';
import init, { compile } from './rust_lib';

function App() {
  useEffect(() => {
    init().then(() => {
      console.log(compile());
    });
  }, []);

  return <div className="App">
    <h1>Hello from React and Rust!</h1>
  </div>;
}

export default App;