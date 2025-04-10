use askama::Template;

#[derive(Default)]
pub struct Raw<T> {
    pub html: String,
    _marker: core::marker::PhantomData<T>,
}

pub struct RawAny {
    pub html: String,
}

impl<T> Raw<T> {
    pub fn new(html: String) -> Self {
        Raw {
            html,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<T: Template> Raw<T> {
    pub fn to_raw(template: T) -> Raw<T> {
        let html = template.render().unwrap();
        Raw::new(html)
    }
}
