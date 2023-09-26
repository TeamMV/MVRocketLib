use mvutils::create_once;
use mvutils::once::CreateOnce;
use uuid::Uuid;
use crate::RpcOut;

#[derive(Clone, Copy)]
pub struct API {
    send: fn(Uuid, Vec<u8>),
    _terminate: fn(Uuid)
}

create_once! {
    static api: API;
    static id: Uuid;
    pub static rpc_out: RpcOut;
}

pub fn init(a: *const API, i: *const Uuid) {
    let _ = api.try_create(move || unsafe { *a } );
    let _ = id.try_create(move || unsafe { *i } );
    let _ = rpc_out.try_create(RpcOut::new);
}

pub fn send(message: Vec<u8>) {
    (api.send)(*id, message)
}