use std::collections::HashMap;

pub mod template;

// Use this file to add helper functions and additional modules.
pub fn count_element_function<I>(it: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut result = HashMap::new();

    for item in it {
        *result.entry(item).or_insert(0) += 1;
    }

    result
}

#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct FilterOnce<I, F> {
    inner: I,
    func: F,
    done: bool,
}

pub fn filter_once<I, F>(it: I, f: F) -> FilterOnce<I::IntoIter, F>
where
    I: IntoIterator,
    F: FnMut(&I::Item) -> bool,
{
    FilterOnce {
        inner: it.into_iter(),
        func: f,
        done: false,
    }
}

impl<I, F> Iterator for FilterOnce<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.done {
            self.inner.next()
        } else if let Some(v) = self.inner.next() {
            if (self.func)(&v) {
                Some(v)
            } else {
                self.done = true;
                self.inner.next()
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lo, hi_opt) = self.inner.size_hint();
        (lo.saturating_sub(1), hi_opt)
    }
}
