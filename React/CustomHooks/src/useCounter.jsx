import {useState} from 'react'

function useCounter(initialVal = 0) {
    const [count, setCount]  = useState(initialVal)

    function add () {
        setCount(prevCount => prevCount + 1)
    }

    function sub () {
        setCount(prevCount => prevCount - 1)
    }

    function reset () {
        setCount(0)
    }

    const action = {
        add, sub, reset
    }

  return [count, action]
}

export default useCounter