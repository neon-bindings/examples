const addon = require('../native/index.node');

console.log(new Float32Array(addon.hello()));
