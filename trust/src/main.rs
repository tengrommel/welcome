use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let n_bytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        eprintln!("read {} bytes(flags: {:x}, proto: {:x}): {:x?}",
                  n_bytes - 4,
                  flags,
                  proto,
                  &buf[..n_bytes]);
    }
    Ok(())
}
