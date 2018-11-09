const fs = require('fs');
const path = require('path');
const bench = require('./bench');
const js = require('./search');
const neon = require('../native/index.node');

const wc = {
  js,
  neon
};

const ROOT = path.resolve(__dirname, '..');
const DATA = path.resolve(ROOT, 'data');

const string = fs.readFileSync(path.resolve(DATA, 'shakespeare-plays.csv'), 'utf8');
const buffer = fs.readFileSync(path.resolve(DATA, 'shakespeare-plays.csv'));

console.log(bench(() => wc.js.search(string, 'thee')));
console.log(bench(() => wc.neon.search(buffer, 'thee')));
