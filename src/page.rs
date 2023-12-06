use crate::Resource;

pub trait Page<R: Sized> {
   fn next(&self, client: &crate::client::Client) -> Result<Option<Self>, crate::Error> where Self: Sized;
   fn prev(&self, client: &crate::client::Client) -> Result<Option<Self>, crate::Error> where Self: Sized;
   fn start(&self) -> usize;
   fn total_size(&self) -> usize;
   fn entries(&self) -> Vec<R>;
}

pub struct PagedIterator<'a, P: Page<R>, R> {
    client: &'a crate::client::Client,
    pending: Vec<R>,
    page: P,
}

impl<'a, P: Page<R>, R> PagedIterator<'a, P, R> {
    pub fn new(client: &'a crate::client::Client, page: P) -> Self {
        Self {
            client,
            pending: Vec::new(),
            page,
        }
    }
}

impl<P: Page<R>, R> Iterator for PagedIterator<'_, P, R> {
    type Item = Result<R, crate::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.pending.pop() {
            return Some(Ok(next));
        }

        match self.page.next(self.client) {
            Ok(Some(page)) => {
                self.page = page;
                self.pending = self.page.entries();
                self.pending.pop().map(Ok)
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
