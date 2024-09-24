//! Pagination support.

/// A page of items.
pub trait Page {
    /// The type of item in the page.
    type Item;

    /// Return the next page, if any.
    fn next(&self, client: &dyn wadl::Client) -> Result<Option<Self>, crate::Error>
    where
        Self: Sized;

    /// Return the previous page, if any.
    fn prev(&self, client: &dyn wadl::Client) -> Result<Option<Self>, crate::Error>
    where
        Self: Sized;

    /// Return the index of the first entry on this page.
    fn start(&self) -> usize;

    /// Return the total number of entries in the page collection
    fn total_size(&self) -> Option<usize>;

    /// Get the entries on this page.
    fn entries(&self) -> Vec<Self::Item>;
}

/// A collection of items that may be paginated.
pub struct PagedCollection<'a, P: Page> {
    client: &'a dyn wadl::Client,
    pending: Vec<P::Item>,
    page: P,
}

impl<'a, P: Page> PagedCollection<'a, P> {
    /// Return the total number of entries in the collection.
    pub fn len(&self) -> Option<usize> {
        self.page.total_size()
    }

    /// Return true if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == Some(0)
            || (self.len().is_none() && self.page.entries().is_empty() && self.page.start() == 0)
    }

    /// Get the item at the given index.
    ///
    /// This will fetch pages as needed to satisfy the request.
    pub fn get(&mut self, index: usize) -> Result<Option<P::Item>, crate::Error> {
        if let Some(total_size) = self.len() {
            if index >= total_size {
                return Ok(None);
            }
        }

        while index < self.page.start() {
            self.page = if let Some(page) = self.page.prev(self.client)? {
                page
            } else {
                return Ok(None);
            };
        }

        while index >= self.page.start() + self.page.entries().len() {
            self.page = if let Some(page) = self.page.next(self.client)? {
                page
            } else {
                return Ok(None);
            };
        }

        let mut entries = self.page.entries();

        Ok(Some(entries.remove(index - self.page.start())))
    }
}

impl<'a, P: Page> PagedCollection<'a, P> {
    /// Create a new page
    pub fn new(client: &'a dyn wadl::Client, page: P) -> Self {
        let mut pending = page.entries();
        pending.reverse();
        Self {
            client,
            pending,
            page,
        }
    }
}

impl<P: Page> Iterator for PagedCollection<'_, P> {
    type Item = Result<P::Item, crate::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.pending.pop() {
            return Some(Ok(next));
        }

        match self.page.next(self.client) {
            Ok(Some(page)) => {
                self.page = page;
                self.pending = self.page.entries();
                self.pending.reverse();
                self.pending.pop().map(Ok)
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyMaster<I> {
        entries: Vec<I>,
        chunk_size: usize,
    }

    struct DummyPage<I> {
        start: usize,
        entries: std::rc::Rc<DummyMaster<I>>,
    }

    impl<I: Clone> Page for DummyPage<I> {
        type Item = I;

        fn next(&self, _: &dyn wadl::Client) -> Result<Option<Self>, crate::Error> {
            if self.start + self.entries.chunk_size >= self.entries.entries.len() {
                Ok(None)
            } else {
                Ok(Some(Self {
                    start: self.start + self.entries.chunk_size,
                    entries: self.entries.clone(),
                }))
            }
        }

        fn prev(&self, _: &dyn wadl::Client) -> Result<Option<Self>, crate::Error> {
            if self.start == 0 {
                Ok(None)
            } else {
                Ok(Some(Self {
                    start: self.start - self.entries.chunk_size,
                    entries: self.entries.clone(),
                }))
            }
        }

        fn start(&self) -> usize {
            self.start
        }

        fn total_size(&self) -> Option<usize> {
            Some(self.entries.entries.len())
        }

        fn entries(&self) -> Vec<Self::Item> {
            self.entries.entries[self.start
                ..std::cmp::min(
                    self.start + self.entries.chunk_size,
                    self.entries.entries.len(),
                )]
                .to_vec()
        }
    }

    #[test]
    fn test_iter() {
        let client = crate::client::Client::anonymous("just testing");
        let master = DummyMaster {
            entries: vec!["a", "b", "c"],
            chunk_size: 2,
        };
        let page = DummyPage {
            start: 0,
            entries: std::rc::Rc::new(master),
        };
        let collection = super::PagedCollection::new(&client, page);

        assert_eq!(collection.len(), Some(3));
        assert!(!collection.is_empty());
        assert_eq!(
            vec!["a", "b", "c"],
            collection.collect::<Result<Vec<_>, _>>().unwrap()
        );
    }

    #[test]
    fn test_empty() {
        let client = crate::client::Client::anonymous("just testing");
        let master: DummyMaster<&str> = DummyMaster::<&str> {
            entries: vec![],
            chunk_size: 2,
        };
        let page = DummyPage {
            start: 0,
            entries: std::rc::Rc::new(master),
        };
        let mut collection = super::PagedCollection::new(&client, page);

        assert_eq!(collection.len(), Some(0));
        assert_eq!(collection.is_empty(), true);

        assert_eq!(collection.next().is_none(), true);
    }
}

pub(crate) trait AsTotalSize {
    fn as_total_size(self) -> Option<usize>;
}

impl AsTotalSize for Option<usize> {
    fn as_total_size(self) -> Option<usize> {
        self
    }
}

impl AsTotalSize for usize {
    fn as_total_size(self) -> Option<usize> {
        Some(self)
    }
}
