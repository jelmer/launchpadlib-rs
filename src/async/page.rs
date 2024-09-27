//! Pagination support
use crate::Error;

/// A page of items.
#[async_trait::async_trait]
pub trait Page: Sized {
    /// The type of item in the page.
    type Item;

    /// Return the next page, if any.
    async fn next<'a>(
        &'a self,
        client: &'a dyn wadl::r#async::Client,
    ) -> Result<Option<Self>, Error>
    where
        Self: Sized;

    /// Return the previous page, if any.
    async fn prev<'a>(
        &'a self,
        client: &'a dyn wadl::r#async::Client,
    ) -> Result<Option<Self>, Error>
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
#[allow(dead_code)]
pub struct PagedCollection<'a, P: Page> {
    client: &'a dyn wadl::r#async::Client,
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
    pub async fn get(&mut self, index: usize) -> Result<Option<P::Item>, Error> {
        if let Some(total_size) = self.len() {
            if index >= total_size {
                return Ok(None);
            }
        }

        while index < self.page.start() {
            self.page = if let Some(page) = self.page.prev(self.client).await? {
                page
            } else {
                return Ok(None);
            };
        }

        while index >= self.page.start() + self.page.entries().len() {
            self.page = if let Some(page) = self.page.next(self.client).await? {
                page
            } else {
                return Ok(None);
            };
        }

        let mut entries = self.page.entries();

        Ok(Some(entries.remove(index - self.page.start())))
    }

    /// Create a new page
    pub fn new(client: &'a dyn wadl::r#async::Client, page: P) -> Self {
        let mut pending = page.entries();
        pending.reverse();
        Self {
            client,
            pending,
            page,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyMaster<I: Send + Sync> {
        entries: Vec<I>,
        chunk_size: usize,
    }

    struct DummyPage<I: Send + Sync> {
        start: usize,
        entries: std::sync::Arc<DummyMaster<I>>,
    }

    #[async_trait::async_trait]
    impl<I: Clone + Send + Sync> Page for DummyPage<I> {
        type Item = I;

        async fn next<'a>(
            &'a self,
            _: &'a dyn wadl::r#async::Client,
        ) -> Result<Option<Self>, Error> {
            if self.start + self.entries.chunk_size >= self.entries.entries.len() {
                Ok(None)
            } else {
                Ok(Some(Self {
                    start: self.start + self.entries.chunk_size,
                    entries: self.entries.clone(),
                }))
            }
        }

        async fn prev<'a>(
            &'a self,
            _: &'a dyn wadl::r#async::Client,
        ) -> Result<Option<Self>, Error> {
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

    #[tokio::test]
    async fn test_iter() {
        let client = crate::r#async::client::Client::anonymous("just testing");
        let master = DummyMaster {
            entries: vec!["a", "b", "c"],
            chunk_size: 2,
        };
        let page = DummyPage {
            start: 0,
            entries: std::sync::Arc::new(master),
        };
        let mut collection = super::PagedCollection::new(&client, page);

        assert_eq!(collection.len(), Some(3));
        assert!(!collection.is_empty());
        assert_eq!("a", collection.get(0).await.unwrap().unwrap());
        assert_eq!("b", collection.get(1).await.unwrap().unwrap());
        assert_eq!("c", collection.get(2).await.unwrap().unwrap());
        assert_eq!(None, collection.get(3).await.unwrap());
    }

    #[tokio::test]
    async fn test_empty() {
        let client = crate::r#async::client::Client::anonymous("just testing");
        let master: DummyMaster<&str> = DummyMaster::<&str> {
            entries: vec![],
            chunk_size: 2,
        };
        let page = DummyPage {
            start: 0,
            entries: std::sync::Arc::new(master),
        };
        let collection = super::PagedCollection::new(&client, page);

        assert_eq!(collection.len(), Some(0));
        assert_eq!(collection.is_empty(), true);
    }
}
