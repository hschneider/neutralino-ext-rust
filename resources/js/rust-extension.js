// RustExtension
//
// Run RustExtension functions by sending dispatched event messages.
//
// (c)2024 Harald Schneider - marketmix.com

class RustExtension {
    constructor(debug=false) {
        this.version = '1.0.0';
        this.debug = debug;
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
}