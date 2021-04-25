pub mod executor;

use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

pub struct Thread {
    future: Pin<Box<dyn Future<Output = ()>>>
}

impl Thread {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Thread {
        Thread {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}


