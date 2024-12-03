import { module } from "module";

async function main() {    

    let response = await module.tryAsJsPromise({ test: "Hello World!"});
    console.log("1", response);

    console.log("2", await module.tryAsJsCallback("Hello World!", (param) => {
        return param;
    }));

    console.log("3", await module.tryAsJsCallback("Hello World!", async (param) => {
        return param;
    }));
}

main();