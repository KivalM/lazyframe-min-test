const worker = new Worker("./worker.js", {type:"module"});

worker.addEventListener("message", async ({ data }) => {
    console.log(data);
});

worker.onerror = function (event) {
    throw new Error( event.message + " (" + event.filename + ":" + event.lineno + ")");
}

window.sendMessage = function (args) {
    worker.postMessage(args);
};


async function onclick() {
    var files = document.getElementById('file').files;
    var file = files[0];

    sendMessage(file)
}

document.getElementById("btn").addEventListener("click", onclick, false);