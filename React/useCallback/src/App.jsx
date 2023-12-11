import { useState, useCallback } from 'react'
import Child from './Child';

function App() {
  const [add, setAdd] = useState(0)
  const [count, setCount] = useState(0)

  const childFn = useCallback( () => {
    console.log("Child fn");
  }, [count])

  return (
    <>
    <h1>useCallback</h1>
    <Child childFn={childFn} />
    <button onClick={e => setAdd(prevVal => prevVal + 1)}>Add</button>
    <h2>{add}</h2>
    <button onClick={e => setCount(prevVal => (prevVal + 2))}>Count</button>
    <h2>{count}</h2>
    </>
  )
}



export default App
