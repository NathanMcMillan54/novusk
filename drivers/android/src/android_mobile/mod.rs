use net::set_net;

pub unsafe fn mobile_init() {
    set_net("on-board");
}
