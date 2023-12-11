import React, {memo} from 'react'

function Child({childFn}) {
    console.log("Child");
    childFn()
    return (
      <>
        <h2>Child Component</h2>
      </>
    )
  }

export default memo(Child)