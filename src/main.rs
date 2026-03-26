use debian_control::Control;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let usage = format!("USAGE: {} <path to debian/control>", args.next().unwrap());
    let path = args.next().ok_or(usage)?;

    let control = Control::from_file(path)?;

    for binary in control.binaries() {
        if let Some(name) = binary.name() {
            println!("{name}");
        }
    }

    Ok(())
}
