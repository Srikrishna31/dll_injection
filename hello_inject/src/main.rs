use dll_syringe::{process::OwnedProcess, Syringe};
use std::{thread, time};

fn main() {
    // find target process by name
    let target_process = OwnedProcess::find_first_by_name("create_window.exe").unwrap();

    // create a new syringe for the target_process
    let syringe = Syringe::for_process(target_process);

    // inject the payload into the target process
    let injected_payload = syringe.inject("target\\debug\\hello.dll").unwrap();
    println!("DLL injected successfully!");

    // do something else
    let ten_millis = time::Duration::from_secs(10);

    println!("Sleeping for 10 secs...");
    thread::sleep(ten_millis);

    // eject the payload from the target (optional)
    syringe.eject(injected_payload).unwrap();
}
