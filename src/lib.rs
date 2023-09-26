pub mod api;

use std::io;
use std::io::Write;
pub use mvrocketlib_macro::main;
pub use api::{API, init, send};
pub use uuid::Uuid;

pub struct RpcOut {
    buffer: Vec<u8>
}

impl RpcOut {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }
}

impl Write for RpcOut {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        send(self.buffer.drain(..).collect());
        Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*) => {
        let _ = write!($crate::api::rpc_out, $($arg)*);
    };
}

macro_rules! println {
    ($($arg:tt)*) => {
        let _ = writeln!($crate::api::rpc_out, $($arg)*);
    };
}