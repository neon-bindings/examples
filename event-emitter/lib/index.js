const Server = require('./server');

if (process.env.NODE_ENV !== 'test') {
  const server = Server();

  server
    .listen()
    .then(() => console.log(`Listening on ${server.port}`))
    .catch(() => console.log('port in use'));
}

module.exports = require('../native/index.node');
