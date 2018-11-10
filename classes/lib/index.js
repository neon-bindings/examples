const addon = require('../native/index.node');

const { User } = addon;

new User(0, 'John', 'Doe', 'johndoe@gmail.com');

module.exports = addon;
