import useCounter from "./useCounter"

function App() {
const [count, action] = useCounter(10)

  return (
   <>
    <h1>Custom Hooks</h1>
    <h2>useCounter</h2>
    <h2>{count}</h2>
    <button onClick={action.add}>Add</button>
    <button onClick={action.sub}>Sub</button>
    <button onClick={action.reset}>Reset</button>
   </>
  )
}

export default App
