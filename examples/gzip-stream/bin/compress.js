"use strict";

const { pipeline } = require("stream");
const { Gzip, constants: { Z_BEST_COMPRESSION } } = require("zlib");

const neonCompress = require("..");
const builtInCompress = () => new Gzip({ level: Z_BEST_COMPRESSION });

const compress = process.argv[2] === "built-in" ? builtInCompress : neonCompress;

pipeline(
    process.stdin,
    compress(),
    process.stdout,
    (err) => {
        if (err) {
            console.error(err);
            process.exitCode = -1;
        }
    }
);
