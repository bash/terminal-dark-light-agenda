use anyhow::{bail, Result};
use libc::{cfmakeraw, tcgetattr, tcsetattr, TCSADRAIN};
use std::ffi::c_int;
use std::io::{stderr, stdin, BufRead, Read as _, Write as _};
use std::os::fd::AsRawFd;
use std::time::{Duration, Instant};
use std::{io, mem};

fn main() -> Result<()> {
    let tty = stderr().as_raw_fd();
    let mut old_termios = unsafe { mem::zeroed() };
    to_io_result(unsafe { tcgetattr(tty, &mut old_termios) })?;
    let mut termios = old_termios;
    unsafe { cfmakeraw(&mut termios) };
    to_io_result(unsafe { tcsetattr(tty, TCSADRAIN, &mut termios) })?;
    let result = measure_repeatedly();
    _ = to_io_result(unsafe { tcsetattr(tty, TCSADRAIN, &mut old_termios) });
    let measurements = result?;
    for measurement in measurements.iter() {
        println!("{}", measurement.as_nanos());
    }
    eprintln!(
        "avg: {:?}, min: {:?}, max: {:?}",
        measurements.iter().sum::<Duration>() / measurements.len() as u32,
        measurements.iter().min().unwrap(),
        measurements.iter().max().unwrap()
    );
    Ok(())
}

fn measure_repeatedly() -> Result<Vec<Duration>> {
    (0..1000).map(|_| measure_once()).collect()
}

fn measure_once() -> Result<Duration> {
    let mut stderr = stderr().lock();
    let mut stdin = stdin().lock();
    write!(stderr, "\x1b[c")?;
    stderr.flush()?;
    let start = Instant::now();
    let mut buffer: [u8; 1] = Default::default();
    stdin.read_exact(&mut buffer)?;
    let duration = start.elapsed();

    if buffer[0] != b'\x1b' {
        bail!("Unexpected response: {:X}", buffer[0])
    }

    // We don't care about the reponse, drop everything until
    // we read the terminating 'c'
    while buffer[0] != b'c' {
        stdin.read_exact(&mut buffer)?;
    }

    Ok(duration)
}

fn to_io_result(code: c_int) -> io::Result<()> {
    if code == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
