use crate::*;

#[component]
pub fn MembershipIcon(
    #[props(extends = SvgAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        svg {
            class: "transition-all [&>path]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80",
            fill: "none",
            height: "24",
            view_box: "0 0 24 24",
            width: "24",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path {
                d: "M13 10H18V16L15.5 14.5L13 16V10Z",
                stroke: "black",
                stroke_linecap: "square",
                stroke_width: "2",
            }
            path {
                d: "M22 10H2M22 10V4H2V10M22 10V20H2V10",
                stroke: "black",
                stroke_linecap: "square",
                stroke_width: "2",
            }
        }
    }
}

#[component]
pub fn HomeIcon(
    #[props(extends = SvgAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {

        svg {
            class: "transition-all [&>path]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80",
            fill: "none",
            height: "24",
            view_box: "0 0 25 24",
            width: "24",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M6.27778 10.2222V18C6.27778 19.1046 7.17321 20 8.27778 20H12.5M6.27778 10.2222L11.7929 4.70711C12.1834 4.31658 12.8166 4.31658 13.2071 4.70711L18 9.5M6.27778 10.2222L4.5 12M18.7222 10.2222V18C18.7222 19.1046 17.8268 20 16.7222 20H12.5M18.7222 10.2222L20.5 12M18.7222 10.2222L18 9.5M18 9.5V6M12.5 20V15",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}

#[component]
pub fn EnIcon(
    #[props(extends = SvgAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        svg {
            class: "object-cover w-4 h-4 rounded-full cursor-pointer",
            fill: "none",
            height: "72",
            view_box: "0 0 72 72",
            width: "72",
            "xlink": "http://www.w3.org/1999/xlink",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                fill: "url(#pattern0_6944_16122)",
                height: "72",
                width: "72",
            }
            defs {
                pattern {
                    height: "1",
                    id: "pattern0_6944_16122",
                    pattern_content_units: "objectBoundingBox",
                    width: "1",
                    r#use {
                        href: "#image0_6944_16122",
                        transform: "scale(0.0138889)",
                    }
                }
                image {
                    height: "72",
                    href: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABICAMAAABiM0N1AAAAVFBMVEUZL13Rd3zLaG0+UHfgoqbZjZEhN2P///+9PUTEUVhWSWkrP2lpeJaJlaxZaIpKXIA1SHB6h6Kdprrj5uzQ1d6wuMi/xdIyMVnTfYLovb/JYmf09fed3V51AAAACXBIWXMAAAsTAAALEwEAmpwYAAADCElEQVR4nO1W2XLcIBCcBAfW4T43Cf//nymYGdmWjZS49OTarn3oWqHW0PQAAJ/C/bbH/Fvz8/cE5IfPFkKJR9tAJHoizhAJloXqSkg3led7sqrsJrG9RxTsHQVMVnV+ThRVxKIiUxJ+S5RG1edCny+Zqm1ZUNUlrqYmYIzRAEISCTroSXTQAV2ZzzSPDr/32EzV6bUHb0lioy1VBULtwWNdVnUO91mVaXkoCmejN+uS6uZMCGqZgwFSQ58hNirSFaotsIcrofHhAOEVCZKIDESmM3L8L1ZC2jc/JyN9NXMy0qeIboiYMFHCVD/XL/ji9cdCMqmCbkZyBYzq+L7v5IrICmcs6wjUIkeJzUhsRm0YH9kqhb0m8tAmu8qRnA3l35Dxe01gPhNMFr02PWhuzk+44nD5mWjXyDpXuQfXQrqwB03RRBN1F1j20LCHB0KQLGUtWc6RpYY3lqQ9ewjhaQ8cyVN3TLwmotk66dgxs8yRzRa3kVhwaWRsFeszlXLuUom4jdhsxSLZRlHFXhV0JaiMrujccfllUeRzUnbVIsaRGc4QEdGSkKWIgzWUo+jiYdPOqZ8Rx3vyWsin/OLBrGZYhy0YMzUcewhwf95jK6gp3kYoUBA5UE7R7iHYw6McWc8eeJqJ8aToPFnnnT0WohcADHuAzTEbBRByMyoujyObE/aSzRXfi6VRjto4M4ZyLVhNsN2G1TbCZgg2Qys6hWTutPEXxTkYOdI/9phfE5sZRIQzlCPj6P3oKZBGmOMcxX8gRp/nqGb0QKSOx5hO/cU6T6S58+PopZdofeIwY8BxLwbFJ/lCaJRixkK/JRq3F3CBiBcjH3IttJ3W1KojjEQ8x0dv15ox+v5nj3kk94JmpE7Hrc2ZcpQzCrjS8RYVmmqrHLnMOcqUI3l0rcnLa42XNBHBJGhBORLjWoODSMiP0+mgaf8P9/dCPz+HX3uAugjwEDoFfLsIcLsI8IWFvl8EUBcBHkKngB8XAS7L0e3rCj1fBFAXAR5Cp4CniwCX5ej2ELqdePQX+dR/cj3FIWUAAAAASUVORK5CYII=",
                    id: "image0_6944_16122",
                    preserve_aspect_ratio: "none",
                    width: "72",
                }
            }
        }

    }
}

#[component]
pub fn KrIcon(
    #[props(extends = SvgAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        svg {
            class: "object-cover w-4 h-4 rounded-full cursor-pointer",
            fill: "none",
            height: "72",
            view_box: "0 0 72 72",
            width: "72",
            "xlink": "http://www.w3.org/1999/xlink",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                fill: "url(#pattern0_6944_16121)",
                height: "72",
                width: "72",
            }
            defs {
                pattern {
                    height: "1",
                    id: "pattern0_6944_16121",
                    pattern_content_units: "objectBoundingBox",
                    width: "1",
                    r#use {
                        href: "#image0_6944_16121",
                        transform: "scale(0.0138889)",
                    }
                }
                image {
                    height: "72",
                    href: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABICAMAAABiM0N1AAAAmVBMVEX///8VFRb8/PyJiYn5+fkKCgt4eHkAR6DNLjoAAAHU1NQ1NTb08vPh4uTs7e7MzMyamppNTU7p5uakpKSxsbFCQkMqKiszbLN4Omba2trgfINaWlvZXWdeXl/US1VmkcZgPnNUVFWXtNnrqq8wQolRgr64MUWaNFNkZGXvub2rwuBOPXkTRZfExMTe3t7Bmq+knLuitc6pnKSkH2OTAAAACXBIWXMAAAsTAAALEwEAmpwYAAACqElEQVR4nO1X2ZKCMBAMkBAGucQDFlHE+1p3///rtkAEEhKOLfdly36y1DTT6TDTQeiNN/47XPc1/8dz0Ifw6DDHkh9goZp9aUx1AeIHfwIojga6+CkcsEY0RwH4FAqD6QcBYluoE5ZNgHxMQSROB1iqMwDPV7qKwppiewAjdSkQVwiDGRoDeO1EHsAYzQAycXNHJmwCMOqoSB0BTGwFiDnVsYtlwoB0OmcSKMRhnRVXCUsBtC4ehLRM3BhAs1jnWoRZ2yiK4yja+ry4aV4865xMGN7GRonVtvYEk+jYajgnE3ZeVTQ51bn6zcb6Q1z9WKZ1YWpZTmQ0EFVFpZlzxbFMn0v0UyaaFWZx5TwQW3Xn/FzcqX6A8zOWVVrWI+QxjBjzzrGnV9UU1jEJj2FEdecmgvfJYoR9yXgM48yIE7zhqkYqYQc50aq0Q9pzzPIv33IeoyqpRxdctxHFqDfsViKjR+crsLk2Vx/DdXh8fNz2Jkp2PE14oxluIXMCOrGnnGtX+sTuMIQooOwmhSUPpbshux1QepTwUBoOINpTejtKeOhtAFGSLVg/9ulQ7U+BY/892hTbsV5fdzwNpWF/+53m6hpCpn23I6AtCDuXVy9toU2Mr46XlmkjLSXtOtoI29gucqLv1sbWaLV7GU/CJQCuKK/IIZ3iArb5Z1O1Xs5jzjrMOBIyBYJxtCyLWogHZCLQhYUDclF850pG9oUrKrhwI7sIW+AyeXYGMGHEIYQ3gZgGmeSEcS6AybZ5rLlnAaMRa+xNsg+CfbJxGrEGY70RSHNxPtaHBa2T5TPCnuLQkndOBrNwLEV89HuKA3LvG0bvuWMniwujrHPd8Tgt43FjyD2d85w+gd3Pj6LwNvKyKwRyf3Opcf/ymoVedvFDr7qKvvHGP8IPz0FGDtZW12EAAAAASUVORK5CYII=",
                    id: "image0_6944_16121",
                    preserve_aspect_ratio: "none",
                    width: "72",
                }
            }
        }
    }
}

#[component]
pub fn SignInIcon() -> Element {
    rsx! {
        svg {
            class: "size-6 group-hover:[&amp;&gt;path]:stroke-menu-text/80",
            fill: "none",
            height: "24",
            view_box: "0 0 25 24",
            width: "25",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M15.5 16.5V19C15.5 20.1046 14.6046 21 13.5 21H6.5C5.39543 21 4.5 20.1046 4.5 19V5C4.5 3.89543 5.39543 3 6.5 3H13.5C14.6046 3 15.5 3.89543 15.5 5V8.0625M20.5 12L9.5 12M9.5 12L12 14.5M9.5 12L12 9.5",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}
