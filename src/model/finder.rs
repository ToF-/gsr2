#[derive(Debug, Clone)]
pub struct Finder<T> {
    items: Vec<T>,
    pos: usize,
}

impl<T> Finder<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items, pos: 0 }
    }

    pub fn first<P>(&mut self, pred: P) -> Option<usize>
    where
        P: Fn(&T) -> bool,
    {
        self.first_from_index(0, pred)
    }

    pub fn first_from_index<P>(&mut self, start: usize, pred: P) -> Option<usize>
    where
        P: Fn(&T) -> bool,
    {
        self.pos = start;
        self.next(pred)
    }

    pub fn next<P>(&mut self, pred: P) -> Option<usize>
    where
        P: Fn(&T) -> bool,
    {
        let index = self.items[self.pos..]
            .iter()
            .position(pred)
            .map(|i| self.pos + i)?;

        self.pos = index + 1;
        Some(index)
    }
}

