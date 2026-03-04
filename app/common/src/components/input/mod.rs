use crate::*;

#[component]
pub fn Input(
    #[props(optional)] class: Option<String>,
    #[props(default = "text".into())] r#type: String,
    #[props(default)] value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(default)] maxlength: usize,
    #[props(default)] disabled: bool,
    #[props(default)] oninput: EventHandler<FormEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base_class = "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]";
    let class = match class {
        Some(extra) if !extra.is_empty() => format!("{} {}", base_class, extra),
        _ => base_class.to_string(),
    };

    let mut attributes = attributes;
    if maxlength > 0 {
        attributes.push(Attribute::new(
            "maxlength",
            maxlength.to_string(),
            None,
            false,
        ));
    }

    rsx! {
        input {
            r#type,
            class,
            value,
            placeholder,
            disabled,
            oninput,
            ..attributes,
        }
    }
}
