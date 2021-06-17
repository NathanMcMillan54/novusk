use net::info::set_net;

pub unsafe fn stm32f4xx_init() {
    set_net("stm32-eth");
}
