use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub unsafe fn ps2_keyboard_init() {
    // If it fails try to reinitialize it seven times
    SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(7)).expect("Couldn't initialize PS2 Keyboard");
}
