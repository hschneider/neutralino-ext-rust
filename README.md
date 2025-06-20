
<p align="center">
<img src="https://marketmix.com/git-assets/neutralino-ext-rust/neutralino-rust-header.svg">
</p>

# neutralino-ext-rust
**A Rust Extension for Neutralino >= 5.0.0**

This extension adds a Rust backend to Neutralino with the following features:
- Requires only a few lines of code on both ends.
- Read all events from the Neutralino app in your Rust code.
- Run Rust functions from Neutralino.
- Run Neutralino functions from Rust.
- All communication between Neutralino and Rust runs asynchronously.
- All events are queued, so none will be missed during processing.
- Track the data flow between Neutralino and Rust in realtime.
- Works in Window- and headless Cloud-Mode.
- Terminates the Rust Runtime when the Neutralino app quits.

![Neutralino Rust Extension](https://marketmix.com/git-assets/neutralino-ext-rust/rust-neutralino.gif)

## Run the demo
The demo opens a Neutralino app. Clicking on the blue link sends a Ping to Rust, which replies with Pong.
This illustrates the data-flow in both directions. 

Before running the demo, the Rust extension needs to be compiled with Rust. Make this folder the project root for your Rust-compiler:
```bash
/extensions/rust
```
Then build with:
```bash
cargo build --release
```
The demo is configured to launch the Rust-extension binary directly from its build-target's release folder.

After this, run these commands in the **ext-rust** folder:
```commandline
neu update
neu run
```

## Integrate into your own project
Follow these steps:
- Adapt the Rust code in **extensions/rust/main.rs** to your needs.
- Build the Rust-binary.
- Create an empty **/extensions/rust** folder, used by your installer.
- Copy the Rust-binary to **/extensions/rust**
- Copy this **/extensions** folder to your project.
- Copy **resources/js/rust-extension.js** to **resources/js**.
- Add `<script src="js/rust-extension.js"></script>` to your **index.html**
- Add `const RUST = new RustExtension(true)` to your **main.js**
- Add **RUST.run(function_name, data) to main.js** to run Rust-functions from Neutralino.
- Add **event listeners to main.js**, to fetch result data from Rust.
- Modify **neutralino.config.json** (see below).

Make sure that **neutralino.config.json** contains this, adapted to your environment:
```json
"nativeAllowList": [
  "app.*",
  "os.*",
  "window.*",
  "events.*",
  "extensions.*",
  "debug.log"
],
"extensions": [
  {
    "id": "extRust",
    "commandDarwin": "${NL_PATH}/extensions/rust/target/release/ext-rust ${NL_PATH}",
    "commandWindows": "${NL_PATH}/extensions/rust/target/release/ext-rust.exe ${NL_PATH}"
  }
],
```

## ./extensions/rust/main.rs explained

```js
mod neutralino;

const DEBUG_EXT_RUST: bool = true;

fn process_app_event(ext: &mut neutralino::Extension, d: &mut serde_json::Value) {

    if ext.is_event(d, "runRust") {
        let data = ext.get_data(d);

        if data["function"].as_str().unwrap() == "ping" {
            let p = data["parameter"].as_str().unwrap();

            let msg = format!("Rust says PONG in reply to '{}'", &p);
            ext.send_message("pingResult", &msg);
        }
    }
}

fn main() {
    let mut ext = neutralino::Extension::new();
    ext.run(process_app_event);
}

```

The extension is activated in main(). 
**process_app_event** is a callback function, which is triggered with each event coming from the Neutralino app.

In the callback function, you can process the incoming events by their name. In this case we react to the **"runRust"** event.
**data["function"]** holds the requested Rust-function and **data["parameter"]** its data payload as JSON.

If the requested function is named **ping**, we send back a message to the Neutralino frontend. 

**send_message()** requires the following parameters:
- An event name, here "pingResult"
- The data package to send, which can be a string or stringified JSON.

The **DEBUG_EXT_RUST** constant tells the NeutralinoExtension to report each event to the console. Incoming and outgoing messages are printed in different colors.

This makes debugging easier, since you can track the data flow between Neutralino and Rust:

![Debug Rust](https://marketmix.com/git-assets/neutralino-ext-rust/rust-console.jpg)

## ./resources/js/main.js explained

```JS

async function onPingResult(e) {
...
}

// Init Neutralino
//
Neutralino.init();
...
Neutralino.events.on("pingResult", onPingResult);
...
// Init Bun Extension
const RUST = new RustExtension(true)
```

The last line initializes the JavaScript part of the Rust-extension. It's important to place this after Neutralino.init() and after all event handlers have been installed. Put it in the last line of your code and you are good to go. The const **RUST** is accessible globally and **must not be renamed** to something else.

The **RustExtension class** takes only 1 argument which instructs it to run in debug mode (here true). In this mode, all data coming from the extension is printed to the dev-console:

![Debug Meutralino](https://marketmix.com/git-assets/neutralino-ext-rust/rust-console-2.jpg)

The **pingResult event handler** listens to messages with the same name, sent by send_message() on Rust's side. 

In **index.html**, you can see how to send data from Neutralino to Rust, which is dead simple:
```html
<a href="#" onclick="RUST.run('ping', 'Neutralino says PING!');">Send PING to Rust</a><br>
```

**RUST.run()** takes 2 arguments:
- The Bun function to call, here "ping"
- The data package to submit, either as string or JSON.

### Long-running tasks and their progress

For details how to start a long-running background task in Rust and how to poll its progress,
see the comments in `extensions/rust/main.rs`and `resources/js/main.js`.

Before a new task is spawned, Rust sends a **startPolling-message** to the frontend. 
As a result, the frontend sends a **poll-message** every 500 ms.

All progress-messages of the long-running task are stored in a queue.
Before the task ends, it pushes a **stopPolling-message** to the queue:

```mermaid
graph LR;
  id[stopPolling]-->id2[Progress 3/3];
  id2[Progress 3/3]-->id3[Progress 2/3];
  id3[Progress 2/3]-->id4[Progress 1/3];
 
```

Each incoming **poll-message** forces Rust to stop listening on the WebSocket and processing 
the queue instead. When the **stopPolling-message** is sent back, the frontend stops polling.

## Modules & Classes Overview

### neutralino.rs (Rust)

Extension Module:

| Variable / Struct                        | Description                                                                                                                                                                          |
|------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| const DEBUG_EXT_RUST                     | This logs debug messages to stdout, if true. This is a global const.                                                                                                                 |
| struct EventMessage                      | An event-message with the following fields:<br>`event`: Event-name as `String`.<br>`data`: Payload as `String` or stringified JSON.                                                      |
| Arc<MessageQueue<EventMessage>> messages | Thread-save message queue, holding the progress event-messages. This is referenced with e.g.  `let q = ext.messages.clone()`. Queue a new event-message with `q.push(EventMessage)`. |

| Method             | Description                                                  |
| ------------------ | ------------------------------------------------------------ |
| new()              | Create a new Extension struct instance.                      |
| run(callback)      | Starts the extensions main processing loop. Each incoming message triggers the callback function. |
| callback(ext, d)   | The callback function referenced by `ext.run(callback)`.<br>ext: The extension instance as `&mut neutralino::Extension`.<br>d: The incoming data-package as `&mut serde_json`. |
| is_event(d, e)     | Checks the incoming event data-package for a particular event-name.<br>d: Data-package as `&serde_json`.<br>e: Event-name as `&str`. |
| get_data(d)        | Extracts a JSON data payload from the data-package's `data` field.<br>d: The data-package as `&serde_json`. |
| send_message(e, d) | Send an event-message to Neutralino. <br>e: Event-name as `&str`.<br>d: Data package as `&str` or stringified JSON. |

| Function                         | Description                                                  |
| -------------------------------- | ------------------------------------------------------------ |
| neutralino::send_queued(q, e, d) | Send queued messages from long-running tasks. Always use this to send messages from within a spawned thread. Outside a thread use the `ext.send_message()` method.<br>q: Thread-save message queue = `ext.messages.clone();`<br />e: Event-name as `&str`<br />d: Data package as `&str` or stringified JSON. |

Events sent from the extension to the frontend:

| Event        | Description                                       |
| ------------ | ------------------------------------------------- |
| startPolling | Starts polling lon-running tasks on the frontend. |
| stopPolling  | Stops polling.                                    |

### rust-extension.js

RustExtension Class:

| Method               | Description                                                  |
| -------------------- | ------------------------------------------------------------ |
| async run(f, p=null) | Call a Rust-function.<br>f: Function-name.<br>p: Parameter data package as string or JSON. |
| async stop()         | Stop and quit the Rust-extension and its parent app. Use this if Neutralino runs in Cloud-Mode. This is called automatically, when the browser tab is closed. |

| Property    | Description                                               |
| ----------- | --------------------------------------------------------- |
| debug       | If true,  data flow is printed to the dev-console.        |
| pollSigStop | If true, then polling for long running tasks is inactive. |

Events, sent from the frontend to the extension:

| Event    | Description                                                  |
| -------- | ------------------------------------------------------------ |
| appClose | Notifies the extension, that the app will close. This quits the extension. |
| poll     | Forces the extsension to process the long-running task's message queue. |

## More about Neutralino

- [NeutralinoJS Home](https://neutralino.js.org) 

- [Neutralino Build Automation for macOS, Windows, Linux](https://github.com/hschneider/neutralino-build-scripts)

- [Neutralino related blog posts at marketmix.com](https://marketmix.com/de/tag/neutralinojs/)



<img src="https://marketmix.com/git-assets/star-me-2.svg">

