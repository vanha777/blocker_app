import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { invoke } from '@tauri-apps/api'

function App() {
  const [count, setCount] = useState(0)
  const [message, setMessage] = useState()

  const callMessage = () => {
    invoke("greet", { name: "Roman" }).then((response) => setMessage(response))
  };

  const callCrash = () => {
    invoke("crash").then((response) => setMessage(response))
  };

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <button onClick={() => callMessage()}>
          Rust function : {message}
        </button>
        <button onClick={() => callCrash()}>
          Crash function
        </button>
        <p>
          Edit <code>src/App.jsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
