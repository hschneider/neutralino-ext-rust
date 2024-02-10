
let POLL_SIG_STOP = true;
let POLL_ID = 0;

// Exit gracefully
//
function onWindowClose() {
    Neutralino.app.exit();
}

// Handle incoming PONGs
//
async function onPingResult(e) {
    console.log("DBG RECEIVED: " + e.detail );
    let msg = document.getElementById("msg");
    msg.innerHTML += e.detail + '<br>';
}

// Start polling progress messages of a long running task.
//
async function onStartPolling(e)  {
    POLL_SIG_STOP = false
    POLL_ID = setInterval(() => {
        console.log("Polling ...")
        RUST.run("poll")
        if(POLL_SIG_STOP) {
            clearInterval(POLL_ID);
        };
    }, 500);
}

// Stop polling.
//
async function onStopPolling(e)  {
    POLL_SIG_STOP = true;
    console.log("Polling stopped!")
}

// Start single instance of long running task
//
document.getElementById('link-long-run')
    .addEventListener('click', () => {
   if(POLL_SIG_STOP) {
       RUST.run('longRun');
   }
});

// Init Neutralino
//
Neutralino.init();
Neutralino.events.on("windowClose", onWindowClose);
Neutralino.events.on("pingResult", onPingResult);
Neutralino.events.on("startPolling", onStartPolling);
Neutralino.events.on("stopPolling", onStopPolling);

// Set title
//
(async () => {
    await Neutralino.window.setTitle(`Neutralino RustExtension ${NL_APPVERSION}`);
})();

// Init Rust Extension
const RUST = new RustExtension(true)

