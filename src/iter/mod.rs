mod tests;

type BoxIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

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
