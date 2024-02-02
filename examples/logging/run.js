"use strict";

const debug = require("debug");
const { hello, init } = require(".");

// Must be called to initialized logging
init(debug);

// Call an example function
hello();

// Give logs a chance to flush
setTimeout(() => {}, 500);
