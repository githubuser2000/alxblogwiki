// src/tables/maintable.rs
#[derive(Debug)]
pub struct MainTable {
    tables: Option<*const crate::tables::Tables>,
}

impl MainTable {
    pub fn new() -> Self {
        Self { tables: None }
    }
    
    pub fn set_tables(&mut self, tables: &crate::tables::Tables) {
        self.tables = Some(tables as *const crate::tables::Tables);
    }
    
    fn tables(&self) -> &crate::tables::Tables {
        unsafe { &*self.tables.unwrap() }
    }
}
