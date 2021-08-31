use core::pin::Pin;
use core::task::{Context, Poll};
use crossbeam_queue::ArrayQueue;
use futures_util::Stream;
use crate::{SCAN_CODE, WAKER};

pub struct KeyboardScancode;

impl KeyboardScancode {
    pub fn new() -> Self {
        return KeyboardScancode;
    }

    pub fn init(&mut self) {
        SCAN_CODE.try_init_once(|| ArrayQueue::new(9)).unwrap();
    }
}

impl Stream for KeyboardScancode {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCAN_CODE.try_get().unwrap();

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}