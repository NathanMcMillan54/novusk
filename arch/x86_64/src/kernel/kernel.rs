pub use kernel::KERNEL;
use novuskinc::fb::FrameBuffer;

extern "C" {
    static mut FB: FrameBuffer<'static>;
}

pub unsafe fn set_kernel_framebuffer(fb: FrameBuffer<'static>) {
    KERNEL.set_fb(fb);
}

pub unsafe fn set_all_default_kernel_drivers() {
    if FB.name == "VGA FrameBuffer" {
        set_kernel_framebuffer(FB)
    } else { set_kernel_framebuffer(FrameBuffer::new()); }
}
