import React from 'react'

type HelloWorldProps = {
  message: string
}
function HelloWorld(props: HelloWorldProps) {
  const { message } = props

  return <h2>{message}</h2>
}

export default HelloWorld
