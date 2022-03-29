async function init() {
    const importObject = {
        console: {
            log: () => {
                console.log("Just logging something!");
            },
            error: () => {
                console.error("Error!");
            }
        }
    }

    const response = await fetch("sum.wasm");
    const buffer = await response.arrayBuffer();
    const wasm = await WebAssembly.instantiate(buffer, importObject);
    const sumFunction = wasm.instance.exports.sum;
    const result = sumFunction(200, 80);
    console.log(result);
}

init();