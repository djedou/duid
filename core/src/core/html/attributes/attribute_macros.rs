use crate::core::html::attributes::{AttributeValue, Value, attr};


#[macro_export]
macro_rules! declare_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            doc_comment::doc_comment!{
                concat!("Creates html [",stringify!($name),"](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/",stringify!($name),") attribute"),

                $(#[$attr])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<V, MSG>(v: V) -> crate::core::html::attributes::Attribute<MSG>
                    where V: Into<Value>,
                    {
                        attr(stringify!($name), AttributeValue::<MSG>::from_value(v.into()))
                }
            }
         )*

    };
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        $(
            doc_comment::doc_comment!{
                concat!("Creates html [",$attribute,"](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/",$attribute,") attribute"),
                $(#[$attr])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<V, MSG>(v: V) -> crate::core::html::attributes::Attribute<MSG>
                    where V: Into<Value>,
                    {
                        attr($attribute, AttributeValue::<MSG>::from_value(v.into()))
                }
             }
         )*
    }
}

macro_rules! declare_html_attributes{
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_attributes!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        pub const HTML_ATTRS:[&'static str; 115] = [$(stringify!($name),)*];
    }
}

macro_rules! declare_html_attributes_special{
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        declare_attributes!{ $($name => $attribute;)*}

        #[cfg(feature = "with-lookup")]
        pub const HTML_ATTRS_SPECIAL:[(&'static str,&'static str); 8] = [$((stringify!($name),$attribute),)*];
    }
}

declare_html_attributes! {
    accept;
    accesskey;
    action;
    align;
    allow;
    alt;
    autocapitalize;
    autocomplete;
    autofocus;
    autoplay;
    background;
    bgcolor;
    border;
    buffered;
    challenge;
    charset;
    cite;
    class;
    codebase;
    color;
    cols;
    colspan;
    content;
    contenteditable;
    contextmenu;
    controls;
    coords;
    crossorigin;
    csp;
    data;
    datetime;
    decoding;
    default;
    defer;
    dir;
    dirname;
    download;
    draggable;
    dropzone;
    enctype;
    enterkeyhint;
    formaction;
    formnovalidate;
    headers;
    height;
    hidden;
    high;
    href;
    hreflang;
    http;
    icon;
    id;
    importance;
    integrity;
    intrinsicsize;
    inputmode;
    ismap;
    itemprop;
    keytype;
    kind;
    lang;
    language;
    loading;
    list;
    low;
    manifest;
    max;
    maxlength;
    minlength;
    media;
    method;
    min;
    multiple;
    muted;
    name;
    novalidate;
    open;
    optimum;
    pattern;
    ping;
    placeholder;
    poster;
    preload;
    radiogroup;
    readonly;
    referrerpolicy;
    rel;
    required;
    reversed;
    rows;
    rowspan;
    sandbox;
    scope;
    scoped;
    selected;
    shape;
    size;
    sizes;
    slot;
    spellcheck;
    src;
    srcdoc;
    srclang;
    srcset;
    start;
    step;
    summary;
    tabindex;
    target;
    title;
    translate;
    usemap;
    value;
    width;
    wrap;
}

declare_html_attributes_special! {
    accept_charset => "accept-charset";

    r#async => "async";

    r#for => "for";

    font_family => "font-family";
    font_size => "font-size";
    flex_direction => "flex-direction";

    r#loop => "loop";

    r#type => "type";
}