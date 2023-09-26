use mvutils::create_once;
use mvutils::once::CreateOnce;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct API {
    send: fn(Uuid, Vec<u8>),
    terminate: fn(Uuid)
}

create_once! {
    static api: API;
    static id: Uuid;
}

pub fn init(a: *const API, i: *const Uuid) {
    let _ = api.try_create(move || unsafe { *a } );
    let _ = id.try_create(move || unsafe { *i } );
}

pub fn send(message: Vec<u8>) {
    (api.send)(*id, message)
}

pub fn terminate() {
    (api.terminate)(*id)
}