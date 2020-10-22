use probe_rs::flashing::{Format,download_file_with_options, DownloadOptions};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    use std::sync::{Arc, Mutex};
    use probe_rs::Probe;
    use probe_rs_rtt::Rtt;

    // First obtain a probe-rs session (see probe-rs documentation for details)
    let probe = Probe::list_all()[0].open()?;
    let mut session = probe.attach("somechip")?;

    // Download the ELF file to target.
    let path_to_elf = Path::new("");
    download_file_with_options(&mut session, path_to_elf, Format::Elf, DownloadOptions {
        progress: None,
        keep_unwritten_bytes: false,
    })?;

    // Attach to RTT.
    let mut rtt = Rtt::attach(Arc::new(Mutex::new(session)))?;

    // Read from a channel.
    if let Some(input) = rtt.up_channels().take(0) {
        let mut buf = [0u8; 1024];
        let count = input.read(&mut buf[..])?;

        println!("Read data: {:?}", &buf[..count]);
    }

    // Write to a channel.
    if let Some(output) = rtt.down_channels().take(0) {
        output.write(b"Hello, computer!\n")?;
    }

    Ok(())
}
