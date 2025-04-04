use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Db {
    page: BTreeMap<String, String>,
    max_page_entries: isize,
}

impl Db {
    pub fn new() -> Db {
        Db {
            page: BTreeMap::new(),
            max_page_entries: 8,
        }
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.page.get(key)
    }
    pub fn set(&mut self, key: &String, value: &String) -> Result<(), ()> {
        self.page.insert(key.clone(), value.clone());
        Ok(())
    }
}
