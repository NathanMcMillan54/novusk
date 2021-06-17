pub static mut NET_NAME: &'static str = "";
pub static mut NET_INIT: bool = false;

pub unsafe fn set_net(name: &'static str) {
    NET_NAME = name;
}

pub unsafe fn set_init() {
    NET_INIT = true;
}
