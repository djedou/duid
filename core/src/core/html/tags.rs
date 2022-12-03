
macro_rules! declare_tags {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            doc_comment::doc_comment!{
                concat!("Creates an html [",stringify!($name),"](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/",stringify!($name),") element"),

            $(#[$attr])*
            #[inline]
            #[allow(non_snake_case)]
            pub fn $name<MSG: Clone>(
                attrs: &[$crate::core::html::attributes::Attribute<MSG>],
                children: &[$crate::core::html::nodes::Node<MSG>]
            ) -> $crate::core::html::nodes::Node<MSG>
                {
                    $crate::core::html::nodes::create_element(
                        None, 
                        stringify!($name), 
                        attrs,
                        children
                    )
                }
            }

         )*
    }
}

macro_rules! declare_sc_tags {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        pub(crate) mod self_closing{
            $(
                doc_comment::doc_comment!{
                    concat!("Creates an html [",stringify!($name),"](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/",stringify!($name),") element"),

                $(#[$attr])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<MSG: Clone>(
                    attrs: &[$crate::core::html::attributes::Attribute<MSG>],
                    children: &[$crate::core::html::nodes::Node<MSG>]
                ) -> $crate::core::html::nodes::Node<MSG>
                    {
                        $crate::core::html::nodes::create_element(
                            None, 
                            stringify!($name), 
                            attrs,
                            children
                        )
                    }
                }

             )*
        }

        #[cfg(feature = "with-lookup")]
        pub const HTML_SC_TAGS: [&'static str; 16] = [$(stringify!($name),)*];
    }
}

macro_rules! declare_common_tags_and_macro {
    ($($(#[$attr:meta])* $name:ident;)*) => {

        pub(crate) mod commons {
            declare_tags! { $($name;)* }

        }

        #[cfg(feature = "with-lookup")]
        pub const HTML_TAGS: [&'static str; 98] = [$(stringify!($name),)*];
    };
}

macro_rules! declare_tags_and_macro {
    ($($(#[$attr:meta])* $name:ident;)*) => {

        declare_tags! { $($name;)* }

    };
}

macro_rules! declare_tags_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_tags!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        pub const HTML_TAGS_NON_COMMON:[&'static str;1] = [$(stringify!($name),)*];
    }
}

macro_rules! declare_tags_and_macro_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_tags_and_macro!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        pub const HTML_TAGS_WITH_MACRO_NON_COMMON:[&'static str;2] = [$(stringify!($name),)*];
    }
}

declare_common_tags_and_macro! {
    head;
    body;
    address;
    article;
    aside;
    footer;
    header;
    h1;
    h2;
    h3;
    h4;
    h5;
    h6;
    hgroup;
    main;
    nav;
    section;
    blockquote;
    dd;
    div;
    dl;
    dt;
    figcaption;
    figure;
    html;
    li;
    ol;
    p;
    pre;
    ul;
    a;
    abbr;
    b;
    bdi;
    bdo;
    cite;
    code;
    data;
    dfn;
    em;
    i;
    kbd;
    mark;
    q;
    rb;
    rp;
    rt;
    rtc;
    ruby;
    s;
    samp;
    small;
    span;
    strong;
    sub;
    sup;
    time;
    u;
    var;
    audio;
    map;
    video;
    iframe;
    object;
    picture;
    canvas;
    noscript;
    script;
    del;
    ins;
    caption;
    colgroup;
    table;
    tbody;
    td;
    tfoot;
    th;
    thead;
    tr;
    button;
    datalist;
    fieldset;
    form;
    label;
    legend;
    meter;
    optgroup;
    option;
    output;
    progress;
    select;
    textarea;
    details;
    dialog;
    menu;
    menuitem;
    summary;
    template;
}

declare_tags_non_common! {
    style;
}

declare_tags_and_macro_non_common! {
    title;
    slot;
}

declare_sc_tags! {
    area;
    base;
    br;
    col;
    command;
    embed;
    hr;
    img;
    input;
    keygen;
    link;
    meta;
    param;
    source;
    track;
    wbr;
}