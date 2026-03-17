const { CancellationToken, nodeReleaseDateWithAbort } = require(".");

const timeout = Number(process.argv[2]) || 10000;

async function main() {
    const version = process.version;
    const ctrl = new AbortController();
    const token = new CancellationToken();

    ctrl.signal.addEventListener("abort", () => token.cancel());
    setTimeout(() => ctrl.abort(), timeout).unref();

    const date = await nodeReleaseDateWithAbort(version, token);

    console.log(date);
}

main();
