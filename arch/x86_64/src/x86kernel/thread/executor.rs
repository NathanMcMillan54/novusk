use alloc::collections::VecDeque;
use core::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};
use super::Thread;


pub struct ThreadExecutor {
    thread_queue: VecDeque<Thread>,
}

impl ThreadExecutor {
    pub fn new() -> ThreadExecutor {
        ThreadExecutor {
            thread_queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, thread: Thread) {
        self.thread_queue.push_back(thread)
    }

    pub fn run(&mut self) {
        while let Some(mut thread) = self.thread_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match thread.poll(&mut context) {
                Poll::Ready(()) => {}
                Poll::Pending => self.thread_queue.push_back(thread),
            }
        }
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const (), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}
