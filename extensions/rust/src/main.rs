// main.rs 1.0.2
//
// Neutralino RustExtension.
//
// (c)2024 Harald Schneider - marketmix.com

use std::time::Duration;

mod neutralino;

const DEBUG_EXT_RUST: bool = true;

fn process_app_event(ext: &mut neutralino::Extension, d: &mut serde_json::Value) {
    //
    // Handle Neutralino app-events
    // ext: Reference to the extension-object itself.
    // d: IPC data-package

    // Check if the frontend requests a function-call via runRust-event.
    // If so, extract the embedded function-data.
    //
    if ext.is_event(d, "runRust") {
        let data = ext.get_data(d);

        // Get the called function's name and its parameters.
        //
        if data["function"].as_str().unwrap() == "ping" {
            let p = data["parameter"].as_str().unwrap();

            // Reply to the frontend.
            //
            let msg = format!("Rust says PONG in reply to '{}'", &p);
            ext.send_message("pingResult", &msg);
        }
        
        // This starts a long-running background-task, which reports
        // its progress to stdout.
        //
        // Problem:
        // Using a scoped thread here instead won't work:
        // In this case the thread borrows the ext object, which will block
        // until finished due to its lifetime. So using ext.send_message()
        // to report progress won't be possible at this point.
        //
        // Solution:
        // Progress messages are pushed into a queue and a startPolling event
        // is sent to the frontend. At the end the spawned task pushes a
        // stopPolling event into the queue.
        // The frontend sends a poll event, which triggers pulling messages from
        // the queue.
        //
        if data["function"].as_str().unwrap() == "longRun" {
            long_run(ext);
        }
    }
}

fn long_run(ext: &mut neutralino::Extension) {
    //
    // Spawn a background-task

    // Clone thread-save queue
    let q = ext.messages.clone();

    // Signal frontend to start polling
    ext.send_message("startPolling", "_");

    // Spawn background-task
    //
    std::thread::spawn(move || {
        let mut p: String;
        for i in 1..=10 {
            p = format!("Long-running task progress: {} / 10", i);
            println!("{}", p);

            // Push progress message to the queue
            //
            let mut msg: neutralino::Data = neutralino::Data {
                event: "".to_string(),
                data: "".to_string(),
            };
            msg.event = "pingResult".to_string();
            msg.data = p;
            q.push(msg);

            std::thread::sleep(Duration::from_secs(1));
        }

        // Signal frontend to stop polling
        //
        let mut msg: neutralino::Data = neutralino::Data {
            event: "".to_string(),
            data: "".to_string(),
        };
        msg.event = "stopPolling".to_string();
        msg.data = "".to_string();
        q.push(msg);
    });
}

fn main() {
    //
    // Activate Extension

    let mut ext = neutralino::Extension::new();
    ext.run(process_app_event);
}
