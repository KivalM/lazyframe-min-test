
import * as pd from "./pkg/qai_template.js"

async function start() {
    console.log("initializing")
    console.time("init")
    await pd.default()
    await pd.init_hooks()
    await pd.initThreadPool(navigator.hardwareConcurrency);
    console.timeEnd("init")
    console.log("initialized")

}


onmessage = async function (e) {
    console.time("read")
    let arr = await e.data.arrayBuffer()
    let b = new Int8Array(arr)
    // Order ID,Product ID,Product Name
    let id = "Product ID"
    let order = "Order ID"
    let name = "Product Name"

    var res = await pd.start(b, id, order, name);

    console.timeEnd("read")

    this.postMessage(res)
}

await start()
