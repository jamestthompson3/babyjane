const messageHandler = message => {
        return message.data && console.log(`message in master: ${message.data}`)
    }
exports.messageHandler = messageHandler
