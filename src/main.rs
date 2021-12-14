use std::error::Error;
use evdev::{Device, EventType};

mod input;
mod select;
mod output;

fn event_loop(input_device: &mut Device) -> Result<(), Box<dyn Error>> {
    let mut output_device = output::build_device(input_device).unwrap();
    for _ in 0..5 {
        if !select::is_readable(input_device) { continue }

        for event in input_device.fetch_events().unwrap() {
            println!("event: {:?}", event);
            //if event.event_type() == EventType::KEY {
            //    println!("event: {:?}", event);
            //}
            output_device.emit(&[event]).unwrap();
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut device = input::select_device();
    device.grab()?;
    event_loop(&mut device)?;
    device.ungrab()?;
    Ok(())
}
