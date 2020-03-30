use std::io;
fn main() -> io::Result<()>{
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let n_bytes = nic.recv(&mut buf[..])?;
        eprintln!("read {} bytes: {:x?}", n_bytes, &buf[..n_bytes]);
    }
    Ok(())
}
