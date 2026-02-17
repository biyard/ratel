use crate::interop::sign_in;
use crate::*;

#[component]
pub fn LoginModal() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-5 w-100 max-w-100 mx-1.25 max-mobile:w-full! max-mobile:max-w-full!",
            id: "login_popup",
            div { class: "flex flex-col gap-4 w-full",
                div { class: "flex flex-row gap-1 justify-start items-center w-full text-sm",
                    label { class: "font-medium text-text-primary", "New user?" }
                    button { class: "text-primary/70 light:text-primary hover:text-primary",
                        "Create an account"
                    }
                }
                div { class: "flex flex-col gap-2.5 w-full",
                    label { class: "text-sm", "Email address" }
                    div { class: "relative w-full",
                        input {
                            autocomplete: "email",
                            class: "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]",
                            "data-slot": "input",
                            "data-testid": "email-input",
                            name: "username",
                            placeholder: "Enter your email address",
                            r#type: "email",
                        }
                    }
                }
                div {
                    aria_hidden: "true",
                    class: "flex flex-col gap-2.5 w-full aria-hidden:hidden",
                    label { class: "text-sm", "Password" }
                    div { class: "relative w-full",
                        input {
                            class: "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] text-text-primary file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground border-input-box-border bg-input-box-bg placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 rounded-[10px] py-5.5 dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]",
                            "data-slot": "input",
                            "data-testid": "password-input",
                            placeholder: "Enter your password",
                            r#type: "password",
                        }
                    }
                }
                div { class: "flex flex-row gap-2.5 justify-between items-center w-full text-sm",
                    a {
                        class: "text-sm text-primary/70 hover:text-primary",
                        href: "/forgot-password",
                        "Forgot password?"
                    }
                    button {
                        class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none [&amp;_svg]:pointer-events-none [&amp;_svg]:size-[15px] shrink-0 [&amp;_svg]:shrink-0 font-[var(--font-raleway)] bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline web light:bg-neutral-600 hover:bg-btn-secondary-hover-bg hover:border-btn-secondary-hover-outline hover:text-btn-secondary-hover-text disabled:bg-btn-secondary--disable-bg disabled:border-btn-secondary-disable-outline disabled:text-btn-secondary-disable-text",
                        "data-slot": "button",
                        "data-testid": "continue-button",
                        "Continue"
                    }
                }
            }
            div { class: "font-light text-center rule-with-text align-center", "Or" }
            div { class: "flex flex-col gap-2.5",
                button {
                    class: "flex flex-row gap-5 items-center px-5 w-full cursor-pointer rounded-[10px] bg-[#000203] py-5.5",
                    onclick: move |_| async {
                        let u = sign_in().await.expect("Failed to sign in");
                        debug!("User info: {:?}", u);
                    },
                    svg {
                        fill: "none",
                        height: "24",
                        view_box: "0 0 24 24",
                        width: "24",
                        xmlns: "http://www.w3.org/2000/svg",
                        g { clip_path: "url(#clip0_2052_51930)",
                            path {
                                d: "M21.7623 12.1871C21.7623 11.3677 21.6958 10.7697 21.552 10.1497H12.1953V13.848H17.6874C17.5768 14.7671 16.9788 16.1512 15.65 17.0813L15.6314 17.2051L18.5898 19.4969L18.7948 19.5174C20.6771 17.7789 21.7623 15.221 21.7623 12.1871Z",
                                fill: "#4285F4",
                            }
                            path {
                                d: "M12.1937 21.9313C14.8844 21.9313 17.1432 21.0454 18.7932 19.5174L15.6484 17.0813C14.8069 17.6682 13.6774 18.0779 12.1937 18.0779C9.55834 18.0779 7.32163 16.3395 6.5243 13.9366L6.40743 13.9466L3.33124 16.3273L3.29102 16.4391C4.92979 19.6945 8.29598 21.9313 12.1937 21.9313Z",
                                fill: "#34A853",
                            }
                            path {
                                d: "M6.52477 13.9366C6.31439 13.3165 6.19264 12.6521 6.19264 11.9656C6.19264 11.279 6.31439 10.6147 6.51371 9.9946L6.50813 9.86253L3.3934 7.4436L3.29149 7.49208C2.61607 8.84299 2.22852 10.36 2.22852 11.9656C2.22852 13.5712 2.61607 15.0881 3.29149 16.439L6.52477 13.9366Z",
                                fill: "#FBBC05",
                            }
                            path {
                                d: "M12.1937 5.85336C14.065 5.85336 15.3273 6.66168 16.047 7.33718L18.8596 4.59107C17.1322 2.9855 14.8844 2 12.1937 2C8.29598 2 4.92979 4.23672 3.29102 7.49214L6.51323 9.99466C7.32163 7.59183 9.55834 5.85336 12.1937 5.85336Z",
                                fill: "#EB4335",
                            }
                        }
                        defs {
                            clipPath { id: "clip0_2052_51930",
                                rect {
                                    fill: "white",
                                    height: "20",
                                    transform: "translate(2 2)",
                                    width: "20",
                                }
                            }
                        }
                    }
                    div { class: "text-base font-semibold text-white", "Continue With Google" }
                }
            }
            div { class: "flex flex-row gap-2.5 justify-center items-center w-full",
                div { class: "font-medium cursor-pointer text-neutral-400 text-xs/3.5",
                    "Privacy Policy"
                }
                div { class: "font-medium cursor-pointer text-neutral-400 text-xs/3.5",
                    "Terms of Service"
                }
            }
        }
    }
}
