const addon = require('../native/index.node');
const assert = require("assert");

const { User } = addon;

const email = "johndoe@gmail.com";

let user = new User(0, "John", "Doe", email);

assert.strictEqual(user.get("email"), email);

module.exports = addon;
