pub mod api;

pub use mvrocketlib_macro::main;
use mvutils::unsafe_utils::DangerousCell;
pub use api::{API, init, send};
pub use uuid::Uuid;

pub struct RpcOut {
    buffer: DangerousCell<Vec<u8>>
}

unsafe impl Sync for RpcOut;

unsafe impl Send for RpcOut;

impl RpcOut {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new().into()
        }
    }

    pub fn write(&self, data: &[u8]) {
        self.buffer.get_mut().extend_from_slice(data);
    }

    pub fn flush(&self) {
        send(self.buffer.get_mut().drain(..).collect());
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::api::rpc_out.write(format!($($arg)*).as_bytes());
        $crate::api::rpc_out.flush();
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::api::rpc_out.write(format!($($arg)*).as_bytes());
        $crate::api::rpc_out.write(b"\n");
        $crate::api::rpc_out.flush();
    };
}