// Copied and edited from: Phillip Opperman https://os.phil-opp.com/async-await/

use conquer_once::spin::OnceCell;
use core::task::Waker;
use crossbeam_queue::ArrayQueue;
use kinfo::info::set_info;

pub mod runner;

pub(crate) static INTERRUPT_WAKEUPS: OnceCell<ArrayQueue<Waker>> = OnceCell::uninit();

pub fn x86_64_thread_init() {
    INTERRUPT_WAKEUPS.try_init_once(|| ArrayQueue::new(7)).expect("Couldn't initialize x86_64 thread and task runner");
}

pub(crate) unsafe fn interrupt_wake(waker: Waker) {
    if let Err(_) = interrupt_wakeups().push(waker) {
        set_info("not ok");
        kinfo!("Dropping interrupt wakeup");
    }
}

pub(crate) fn interrupt_wakeups() -> &'static ArrayQueue<Waker> {
    INTERRUPT_WAKEUPS
        .try_get()
        .expect("interrupt wakeup queue not initialized")
}
