import React, { useEffect, useState } from 'react'
import HelloWorld from '../templates/HelloWorld'

function HelloWorldView() {
  const [message, setMessage] = useState('')

  // API から取得した mesage setMessage
  useEffect(() => {
    fetch('http://localhost:8080/hello')
      .then((res) => res.json())
      .then((data) => setMessage(data.message))
      .catch((err) => console.error(err))
  }, [setMessage])

  return <HelloWorld message={message ? message : 'Loading...'} />
}

export default HelloWorldView
