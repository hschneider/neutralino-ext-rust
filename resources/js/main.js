
function onWindowClose() {
    Neutralino.app.exit();
}

async function onPingResult(e) {
    console.log("DBG RECEIVED: " + e.detail );

    let msg = document.getElementById("msg");
    msg.innerHTML += e.detail + '<br>';
}

// Init Neutralino
//
Neutralino.init();
Neutralino.events.on("windowClose", onWindowClose);
Neutralino.events.on("pingResult", onPingResult);

// Set title
//
(async () => {
    await Neutralino.window.setTitle(`Neutralino RustExtension ${NL_APPVERSION}`);
})();

// Init Bun Extension
const RUST = new RustExtension(true)

