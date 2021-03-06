const cluster = require('cluster'),
  numCPUs = require('os').cpus().length,
  WebSocket = require('ws'),
  PORT = 8000,
  util = require('util'),
  lodash = require('lodash')

if (cluster.isMaster) {
  console.log(`Master ${process.pid} is running on port: ${PORT}`)
  for (let i = 0; i < numCPUs; i++) {
    cluster.fork()
  }

  const { workers } = cluster
  const workerArray = Object.values(workers)
  workerArray.map(worker =>
    worker.on('message', message => {
      if (message.data && message.id) {
        workerArray
          .filter(i => i.process.pid !== message.id)
          .map(listener => listener.send({ data: message.data }))
      } else {
        console.log('no data')
      }
    })
  )
  cluster.on('exit', (worker, code, signal) => {
    console.log(`worker ${worker.process.pid} died`)
  })
} else {
  const wss = new WebSocket.Server({
    port: PORT
  })
  wss.broadcast = data => {
    wss.clients.forEach(client => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(data)
      }
    })
  }
  wss.on
  sendBroadcast = data => wss.broadcast(data)
  setTimeout(sendBroadcast, 20000)
  wss.on('connection', (ws, req) => {
    console.log(`${process.pid} has received a connection`)
    ws.on('message', message => {
      console.log('message: ', message)
      process.send({ data: message, id: process.pid })
      messageHandler = message => {
        return message.data && message.id !== process.pid && sendBroadcast(message.data)
      }
      process.on('message', messageHandler)
    })
  })
}
