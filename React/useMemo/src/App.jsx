import { useState, useMemo } from 'react'

function App() {
  const [add, setAdd] = useState(0)
  const [sub, setSub] = useState(100)

  const mul = useMemo(() => {
    console.log('mul')
    return add * 10
  }, [add])

  return (
    <>
    <h1>useMemo</h1>
    <div>
      <button onClick={e => setAdd(prevVal => prevVal + 1)}>Add</button>
      <h2>{add}</h2>
    </div>
    <div>
      <button onClick={e => setSub(prevVal => prevVal - 1)}>Sub</button>
      <h2>{sub}</h2>
    </div>
    <h3>Mul</h3>
    <h2>{mul}</h2>
    </>
  )
}

export default App
