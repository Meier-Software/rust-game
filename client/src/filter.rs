//! This is a regex filtering system that only runs clientside.
pub struct Filters {
    filters: Vec<String>,
}
impl Filters {
    pub fn new() -> Self {
        Self { filters: vec![] }
    }

    pub fn filter_to_wimble_text(&self, string: String) -> String {
        for filter in self.filters.clone().into_iter() {
            println!("{}", filter);
        }
        string
    }
}
