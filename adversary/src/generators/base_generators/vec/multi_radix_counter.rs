pub struct MultiRadixCounter {
    wrap_at: usize,
    current: Option<Vec<usize>>,
}

impl MultiRadixCounter {
    pub fn new(wrap_at: usize, len: usize) -> Self {
        MultiRadixCounter {
            wrap_at,
            current: Some(vec![0; len]),
        }
    }
}

impl Iterator for MultiRadixCounter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take()?;

        let mut next = current.clone();
        for index in next.iter_mut() {
            if *index + 1 < self.wrap_at {
                *index += 1;
                break;
            } else {
                *index = 0;
            }
        }

        // Unless we rolled over to all 0s, we still have more work to do
        if next.iter().any(|n| *n != 0) {
            self.current = Some(next);
        }

        Some(current)
    }
}
