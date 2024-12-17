use crate::route::Language;

pub struct PagesTranslate<'a> {
    pub text: &'a str,
}

pub fn translate_pages<'a>(lang: &'a Language) -> PagesTranslate<'a> {
    if lang.clone() == Language::Ko {
        PagesTranslate {
            text: "homepage page",
        }
    } else {
        todo!()
    }
}
