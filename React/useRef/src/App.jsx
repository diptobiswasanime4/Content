import { useState, useRef } from 'react'
import './App.css'

function App() {
  const refElem = useRef("")
  const [input, setInput] = useState("")
  console.log(refElem);
  function clear () {
    setInput("")
    refElem.current.focus()
  }
  function bold () {
    const curFontWeight = refElem.current.style.fontWeight;
        if (curFontWeight === 'bold') {
          refElem.current.style.fontWeight = 'normal';
        } else {
          refElem.current.style.fontWeight = 'bold';
        }
        refElem.current.focus();
  }
  
  function italic() {
    const curFontStyle = refElem.current.style.fontStyle;
    if (curFontStyle === 'italic') {
      refElem.current.style.fontStyle = 'normal';
    } else {
      refElem.current.style.fontStyle = 'italic';
    }
    refElem.current.focus();
  }
  
  return (
    <>
    <h1>useRef</h1>
    <button onClick={clear}>Clear</button>
    <button onClick={bold}>Bold</button>
    <button onClick={italic}>Italic</button>
    <div>
    <textarea ref={refElem} type="text" placeholder='Write something' value={input} onChange={e => setInput(e.target.value)} />
    </div>
    </>
  )
}

export default App
