use core::iter::{DoubleEndedIterator, ExactSizeIterator, Iterator};

#[allow(clippy::module_name_repetitions)]
pub trait Stream {
    type Item: ?Sized;

    fn next(&mut self) -> Option<&Self::Item>;

    fn cloned(self) -> Cloned<Self::Item, Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Cloned(self)
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait StreamMut: Stream {
    fn next(&mut self) -> Option<&mut Self::Item>;
}

#[allow(clippy::module_name_repetitions)]
pub trait DoubleEndedStream: Stream {
    fn next_back(&mut self) -> Option<&Self::Item>;
}

pub trait DoubleEndedStreamMut: StreamMut {
    fn next_back(&mut self) -> Option<&mut Self::Item>;
}

#[allow(clippy::module_name_repetitions)]
pub trait ExactSizeStream: Stream {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait ExactSizeStreaMut: StreamMut + ExactSizeStream {}

pub struct Cloned<I: Clone, S: Stream<Item = I>>(S);

impl<I: Clone, S: Stream<Item = I>> Iterator for Cloned<I, S> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        match Stream::next(&mut self.0) {
            None => None,
            Some(x) => Some(Clone::clone(x)),
        }
    }
}

impl<I: Clone, S: DoubleEndedStream<Item = I>> DoubleEndedIterator for Cloned<I, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match DoubleEndedStream::next_back(&mut self.0) {
            None => None,
            Some(x) => Some(Clone::clone(x)),
        }
    }
}

impl<I: Clone, S: ExactSizeStream<Item = I>> ExactSizeIterator for Cloned<I, S> {
    fn len(&self) -> usize {
        ExactSizeStream::len(&self.0)
    }
}
