macro_rules! declare_svg_tags{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            doc_comment::doc_comment!{
                concat!("Creates an svg [",stringify!($name),"](/https://developer.mozilla.org/en-US/docs/Web/SVG/Element/",stringify!($name),") element"),

                $(#[$attr])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<MSG: Clone>(
                    attrs: &[$crate::core::html::attributes::Attribute<MSG>],
                    children: &[$crate::core::html::nodes::Node<MSG>]
                ) -> $crate::core::html::nodes::Node<MSG>
                {
                        $crate::core::svg::svg_element(
                            stringify!($name),
                            attrs,
                            children
                        )
                }
            }
         )*
    };

    ( $(
         $(#[$attr:meta])*
         $name:ident => $tagname:tt;
       )*
     ) => {
        $(
            doc_comment::doc_comment!{
                concat!("Creates an svg [",$tagname,"](/https://developer.mozilla.org/en-US/docs/Web/SVG/Element/",$tagname,") element"),

                $(#[$attr])*
                #[inline]
                #[allow(non_snake_case)]
                pub fn $name<MSG: Clone>(
                    attrs: &[$crate::core::html::attributes::Attribute<MSG>],
                    children: &[$crate::core::html::nodes::Node<MSG>]
                ) -> $crate::core::html::nodes::Node<MSG>
                {
                    $crate::core::svg::svg_element(
                        stringify!($name),
                        attrs,
                        children
                    )
                }
            }
         )*
    }

}

macro_rules! declare_common_svg_tags_and_macro {
    ($($(#[$attr:meta])* $name:ident;)*) => {

        pub(crate) mod commons {
            declare_svg_tags! { $($name;)* }
        }


        #[cfg(feature = "with-lookup")]
        pub const SVG_TAGS: [&'static str; 66] = [ $(stringify!($name),)* ];

    };
}

macro_rules! declare_svg_tags_special{
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        declare_svg_tags!{ $($name=>$attribute;)*}

        #[cfg(feature = "with-lookup")]
        pub const SVG_TAGS_SPECIAL:[(&'static str,&'static str); 2] = [$((stringify!($name),$attribute),)*];
    }
}

macro_rules! declare_svg_tags_non_common{

    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        declare_svg_tags!{ $($name;)*}

        #[cfg(feature = "with-lookup")]
        pub const SVG_TAGS_NON_COMMON:[&'static str;5] = [$(stringify!($name),)*];
    }
}

declare_common_svg_tags_and_macro! {
    animate;
    animateMotion;
    animateTransform;
    circle;
    clipPath;
    defs;
    desc;
    discard;
    ellipse;
    feBlend;
    feColorMatrix;
    feComponentTransfer;
    feComposite;
    feConvolveMatrix;
    feDiffuseLighting;
    feDisplacementMap;
    feDistantLight;
    feDropShadow;
    feFlood;
    feFuncA;
    feFuncB;
    feFuncG;
    feFuncR;
    feGaussianBlur;
    feImage;
    feMerge;
    feMergeNode;
    feMorphology;
    feOffset;
    fePointLight;
    feSpecularLighting;
    feSpotLight;
    feTile;
    feTurbulence;
    filter;
    foreignObject;
    g;
    hatch;
    hatchpath;
    image;
    line;
    linearGradient;
    marker;
    mask;
    mesh;
    meshgradient;
    meshpatch;
    meshrow;
    metadata;
    mpath;
    path;
    pattern;
    polygon;
    polyline;
    radialGradient;
    rect;
    set;
    solidcolor;
    stop;
    svg;
    switch;
    symbol;
    textPath;
    tspan;
    unknown;
    view;
}
declare_svg_tags_special! {
    color_profile => "color-profile";
    r#use => "use";
}

declare_svg_tags_non_common! {
    script;
    style;
    text;
    a;
    title;
}