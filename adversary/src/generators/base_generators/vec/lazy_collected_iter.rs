pub struct LazyCollectedIter<I: Iterator> {
    iter: I,
    history: Vec<I::Item>,
}

impl<I: Iterator> LazyCollectedIter<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            history: Vec::new(),
        }
    }

    fn advance_to(&mut self, index: usize) {
        while self.history.len() <= index {
            let Some(next) = self.iter.next() else {
                break;
            };

            self.history.push(next);
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&I::Item> {
        self.advance_to(index);

        self.history.get(index)
    }
}
