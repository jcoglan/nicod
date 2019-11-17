mod tests;

use std::mem;

pub type BoxIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

pub struct Flatten<'a, T> {
    inner: Inner<'a, T>,
}

impl<'a, T> Flatten<'a, T> {
    pub fn new<I, J>(streams: I) -> Flatten<'a, T>
    where
        I: IntoIterator<Item = J> + 'a,
        J: IntoIterator<Item = T> + 'a,
    {
        let iters = streams
            .into_iter()
            .map(|s| Box::new(s.into_iter()) as BoxIter<'a, T>);

        let inner = Inner::Pending(Some(Box::new(iters)));
        Flatten { inner }
    }
}

impl<'a, T: 'a> Iterator for Flatten<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

enum Inner<'a, T> {
    Pending(Option<BoxIter<'a, BoxIter<'a, T>>>),
    Forced(Interleave<'a, T>),
}

impl<'a, T: 'a> Inner<'a, T> {
    fn force(&mut self) {
        if let Inner::Pending(streams) = self {
            if let Some(streams) = streams.take() {
                self.expand_streams(streams);
            }
        }
    }

    fn expand_streams(&mut self, mut streams: BoxIter<'a, BoxIter<'a, T>>) {
        if let Some(head) = streams.next() {
            let tail = Box::new(Inner::Pending(Some(streams)));
            let merged = Interleave::new(vec![head, tail]);
            mem::replace(self, Inner::Forced(merged));
        }
    }
}

impl<'a, T: 'a> Iterator for Inner<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.force();

        match self {
            Inner::Forced(stream) => stream.next(),
            _ => None,
        }
    }
}

pub struct Interleave<'a, T> {
    cursor: usize,
    iters: Vec<BoxIter<'a, T>>,
}

impl<'a, T> Interleave<'a, T> {
    pub fn new<I, J>(iters: I) -> Interleave<'a, T>
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = T> + 'a,
    {
        let iters: Vec<_> = iters
            .into_iter()
            .map(|s| Box::new(s.into_iter()) as BoxIter<'a, T>)
            .collect();

        let cursor = if iters.is_empty() { 0 } else { iters.len() - 1 };

        Interleave { cursor, iters }
    }
}

impl<T> Iterator for Interleave<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        for _ in 0..self.iters.len() {
            self.cursor = (self.cursor + 1) % self.iters.len();
            let item = self.iters[self.cursor].next();

            if item.is_some() {
                return item;
            }
        }
        None
    }
}
