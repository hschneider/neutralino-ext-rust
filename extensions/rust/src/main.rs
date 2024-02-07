// main.rs 1.0.0
//
// Neutralino RustExtension.
//
// (c)2024 Harald Schneider - marketmix.com

mod neutralino;

const DEBUG: bool = true;

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
    }
}

fn main() {
    //
    // Activate Extension

    let mut ext = neutralino::Extension::new();
    ext.run(process_app_event);
}
