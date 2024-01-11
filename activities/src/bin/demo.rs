#[derive(Debug)]
struct YoungPeople<'a> {
    inner: Vec<&'a IdCard>,
}


impl<'a> YoungPeople<'a> {
    fn living_in_fooville(&self) -> Self {
        Self {
            inner: self
                .inner.iter()
                .filter(|id| id.city == City::Fooville)
                .map(|id| *id)
                .collect()
        }
    }
}