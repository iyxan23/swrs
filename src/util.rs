/// Simply contains utilities for the library to function

/// A struct that counts every `next`
pub struct CountingIterator<I> {
    iter: I,
    count: u32,
}

impl<I> CountingIterator<I>
where I: Iterator {
    pub fn new(iterator: I) -> CountingIterator<I> {
        CountingIterator {
            iter: iterator,
            count: 0
        }
    }

    pub fn get_count(&self) -> u32 { self.count }
}

impl<I> Iterator for CountingIterator<I>
where
    I: Iterator {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        self.iter.next()
    }
}