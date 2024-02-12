// RustExtension
//
// Run RustExtension functions by sending dispatched event messages.
//
// (c)2024 Harald Schneider - marketmix.com

class RustExtension {
    constructor(debug=false) {
        this.version = '1.0.1';
        this.debug = debug;

        this.pollSigStop = true;
        this.pollID = 0;

        // Init callback handlers for polling.
        //
        Neutralino.events.on("startPolling", this.onStartPolling);
        Neutralino.events.on("stopPolling", this.onStopPolling);
    }
    async run(f, p=null) {
        //
        // Call a RustExtension function.

        let ext = 'extRust';
        let event = 'runRust';

        let data = {
            function: f,
            parameter: p
        }

        if(this.debug) {
            console.log(`EXT_RUST: Calling ${ext}.${event} : ` + JSON.stringify(data));
        }

        await Neutralino.extensions.dispatch(ext, event, data);
    }

    async stop() {
        //
        // Stop and quit the Bun-extension and its parent app.
        // Use this if Neutralino runs in Cloud-Mode.

        let ext = 'extRust';
        let event = 'appClose';

        if(this.debug) {
            console.log(`EXT_RUST: Calling ${ext}.${event}`);
        }
        await Neutralino.extensions.dispatch(ext, event, "");
        await Neutralino.app.exit();
    }

    async onStartPolling(e)  {
        //
        // This starts polling long-running tasks.
        // Since this is called back from global context,
        // we have to refer 'RUST' instead of 'this'.

        RUST.pollSigStop = false
        RUST.pollID = setInterval(() => {
            if(RUST.debug) {
                console.log("EXT_RUST: Polling ...")
            }
            RUST.run("poll")
            if(RUST.pollSigStop) {
                clearInterval(RUST.pollID);
            };
        }, 500);
    }

    async onStopPolling(e)  {
        //
        // Stops polling.
        // Since this is called back from global context,
        // we have to refer 'RUST' instead of 'this'.

        RUST.pollSigStop = true;
        if(RUST.debug) {
            console.log("EXT_RUST: Polling stopped!")
        }
    }
}