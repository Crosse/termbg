use std::io::{self, Error, ErrorKind, Read, Result, Write};
use std::mem::MaybeUninit;
use std::os::unix::io::AsRawFd;
use std::{env, fmt};

use libc::{ECHO, ICANON, TCSANOW, VMIN, VTIME};

#[derive(Debug, Clone, Default)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

pub enum BackgroundStyle {
    Light,
    Dark,
    Unknown,
}

impl fmt::Display for BackgroundStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BackgroundStyle::*;
        match self {
            Light => "light",
            Dark => "dark",
            Unknown => "unknown",
        }
        .fmt(f)
    }
}

impl From<RgbColor> for BackgroundStyle {
    fn from(rgb: RgbColor) -> Self {
        hsp(rgb)
    }
}

struct TerminalSettings(libc::termios);

impl From<libc::termios> for TerminalSettings {
    fn from(termios: libc::termios) -> TerminalSettings {
        Self(termios)
    }
}

fn set_terminal_raw_mode() -> Result<TerminalSettings> {
    let stdin = io::stdin();
    let stdin_fd = stdin.as_raw_fd();
    let mut termios = MaybeUninit::<libc::termios>::uninit();

    let termios = unsafe {
        let retval = libc::tcgetattr(stdin_fd, termios.as_mut_ptr());
        if retval != 0 {
            return Err(Error::last_os_error());
        }
        termios.assume_init()
    };

    let mut raw = termios;
    raw.c_iflag = 0;
    raw.c_lflag = 0;
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;
    raw.c_lflag &= !(ICANON | ECHO);

    unsafe {
        let retval = libc::tcsetattr(stdin_fd, TCSANOW, &raw as *const _);
        if retval != 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(termios.into())
}

fn set_terminal_mode(settings: &TerminalSettings) -> Result<()> {
    let stdin = io::stdin();
    let stdin_fd = stdin.as_raw_fd();

    unsafe {
        let retval = libc::tcsetattr(stdin_fd, TCSANOW, &settings.0 as *const _);
        if retval != 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn get_background_color() -> Result<RgbColor> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let term_settings = set_terminal_raw_mode()?;

    let magic = match env::var("TMUX") {
        Ok(_) => "\x1bPtmux;\x1b\x1b]11;?\x1b\x1b\\\x1b\\",
        Err(_) => "\x1b]11;?\x1b\\",
    };

    if let Err(e) = write!(stdout, "{}", magic) {
        set_terminal_mode(&term_settings)?;
        return Err(e);
    }

    if let Err(e) = stdout.flush() {
        set_terminal_mode(&term_settings)?;
        return Err(e);
    }

    // if env::var("TMUX").is_ok() {
    //     use std::{thread, time};
    //     thread::sleep(time::Duration::from_millis(250));
    // }

    let mut buf: Vec<u8> = Vec::new();
    if let Err(e) = stdin.read_to_end(&mut buf) {
        set_terminal_mode(&term_settings)?;
        return Err(e);
    }

    if let Err(e) = set_terminal_mode(&term_settings) {
        set_terminal_mode(&term_settings)?;
        return Err(e);
    }
    buf.retain(|b| b.is_ascii() && !b.is_ascii_control());

    let sanitized = String::from_utf8(buf)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "response is not valid UTF-8"))?;

    if !sanitized.starts_with("]11;rgb:") {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("terminal result does not match (got: \"{}\")", sanitized),
        ));
    }

    let mut vals: Vec<std::result::Result<u16, _>> = sanitized[8..]
        .split('/')
        .map(|v| u16::from_str_radix(v, 16))
        .collect();

    vals.retain(|v| v.is_ok());

    if vals.len() < 3 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "not enough values for r, g, b",
        ));
    }
    let vals: Vec<u8> = vals
        .iter()
        .map(|v| (v.clone().unwrap() & 0xff) as u8)
        .collect();

    Ok(RgbColor {
        red: vals[0],
        green: vals[1],
        blue: vals[2],
    })
}

fn hsp(color: RgbColor) -> BackgroundStyle {
    let r: f64 = color.red as f64 * color.red as f64;
    let g: f64 = color.green as f64 * color.green as f64;
    let b: f64 = color.blue as f64 * color.blue as f64;

    let hsp = ((0.299 * r) + (0.587 * g) + (0.114 * b)).sqrt();

    if hsp > 127.5 {
        BackgroundStyle::Light
    } else {
        BackgroundStyle::Dark
    }
}
