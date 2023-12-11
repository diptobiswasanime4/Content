import { useReducer } from 'react'
import { useState } from 'react'

function App() {
  const reducer = (state, action) => {
    switch (action) {
      case "Add":
        return state + 1
      case "Sub":
        return state -1
      default:
        return state
    } 
  }

  const initialState = 0

  const [count, dispatch] = useReducer(reducer, initialState)

  return (
    <>
    <h1>useReducer</h1>
    <button onClick={() => dispatch("Add")}>Add</button>
    <button onClick={() => dispatch("Sub")}>Sub</button>
    <h2>{count}</h2>

    </>
  )
}

export default App
