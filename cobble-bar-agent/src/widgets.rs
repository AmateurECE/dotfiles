use std::error::Error;

use futures::{Future, Sink};

pub mod clock;

pub trait Widget {
    fn run<S>(&mut self, sink: S) -> impl Future<Output = Result<(), Box<dyn Error>>>
    where
        S: Sink<String> + Unpin,
        <S as Sink<String>>::Error: std::error::Error + Send + Sync + 'static;
}
