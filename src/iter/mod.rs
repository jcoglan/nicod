mod tests;

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

        let inner = Inner::new(Box::new(iters));
        Flatten { inner }
    }
}

impl<T> Iterator for Flatten<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

struct Inner<'a, T> {
    thunk: Thunk<'a, Option<Interleave<'a, T>>>,
}

impl<'a, T> Inner<'_, T> {
    fn new(mut streams: BoxIter<'a, BoxIter<'a, T>>) -> Inner<'a, T> {
        let thunk = Thunk::new(|| {
            streams.next().map(|head| {
                let tail = Box::new(Inner::new(streams));
                Interleave::new(vec![head, tail])
            })
        });

        Inner { thunk }
    }
}

impl<T> Iterator for Inner<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.thunk.as_mut() {
            Some(stream) => stream.next(),
            _ => None,
        }
    }
}

enum Thunk<'a, T: 'a> {
    Pending(Option<Box<dyn 'a + FnOnce() -> T>>),
    Forced(T),
}

impl<'a, T> Thunk<'a, T> {
    fn new<F>(gen: F) -> Thunk<'a, T>
    where
        F: 'a + FnOnce() -> T,
    {
        Thunk::Pending(Some(Box::new(gen)))
    }
}

impl<T> AsMut<T> for Thunk<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        if let Thunk::Pending(gen) = self {
            let gen = gen.take().unwrap();
            *self = Thunk::Forced(gen());
        }

        match self {
            Thunk::Forced(ref mut value) => value,
            _ => panic!(),
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
