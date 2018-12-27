const Server = require('./server');

const server = Server();

server
  .listen()
  .then(() => console.log(`Listening on ${server.port}`))
  .catch(() => console.log('port in use'));

module.exports = require('../native/index.node');
