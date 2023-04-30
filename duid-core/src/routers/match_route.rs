#[doc(hidden)]
pub use lazy_static;
#[doc(hidden)]
pub use regex::{Regex, Captures};
use std::fmt;
use std::collections::BTreeMap;

/// Possible failures for the `match_route` and
/// `match_route_regex` macros.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The path did not match any patterns.
    NotFound
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            &Error::NotFound => "Not found"
        })
    }
}

impl std::error::Error for Error {}

/// Converts a URL pattern to a String suitable for compilation as a
/// regular expression.
pub fn pattern(src: &str) -> String {
    use lazy_static::lazy_static;
    lazy_static! {
        static ref TAIL: Regex = Regex::new(r"^\\\*|/\\\*").unwrap();
        static ref NAMED: Regex = Regex::new(
            ":([0-9a-zA-Z][_0-9a-zA-Z]*)"
        ).unwrap();
    }
    let dest = regex::escape(src);
    let dest = TAIL.replace(&dest, "(?P<__tail__>/.*)");
    let dest = NAMED.replace_all(&dest, |caps: &Captures| {
        format!("(?P<{}>[^/]+)", &caps[1])
    });
    format!("^{}$", dest)
}

/// Parameters matched by a URL pattern.
#[derive(Debug, Clone)]
pub struct Params {
    named: BTreeMap<String, String>,
    tail: Option<String>,
}

impl Params {
    /// Gets a named parameter from the matched pattern.
    ///
    /// For example, the pattern `/post/:id` would produce a named
    /// parameter that can be retrieved using `get("id")`.
    pub fn get<'a>(&'a self, name: &str) -> Option<&'a str> {
        self.named.get(name).map(|s| &s[..])
    }

    /// If the pattern included `/*`, this is the rest of the path
    /// from that point (including the leading forward-slash).
    ///
    /// For example, the path `/foo/bar` applied to the pattern
    /// `/foo/*` would result in a tail() value of `Some("/bar")`). If
    /// the pattern did not include `/*`, this method returns None.
    pub fn tail(&self) -> Option<&str> {
        self.tail.as_ref().map(|s| &s[..])
    }

    /// The total number of matched parameters (including the tail, if
    /// any).
    pub fn len(&self) -> usize {
        self.named.len() + (if self.tail.is_some() { 1 } else { 0 })
    }

    /// Returns true if the pattern matched no named parameters and no
    /// tail, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.named.is_empty() && self.tail.is_none()
    }

    /// Creates a Params struct from the provided Regex capture names
    /// and matches (the tail capture should be named "\_\_tail\_\_").
    ///
    /// You should not need to call this directly.
    pub fn from_captures<'t>(
        names: regex::CaptureNames,
        caps: Captures<'t>
    ) -> Self {
        let mut named = BTreeMap::new();
        let mut tail = None;
        for (name, value) in names.zip(caps.iter()) {
            match name {
                Some("__tail__") => {
                    tail = value.map(|s| s.as_str().to_string());
                },
                Some(name) => {
                    if let Some(value) = value {
                        named.insert(
                            name.to_string(),
                            value.as_str().to_string()
                        );
                    }
                },
                None => (),
            }
        }
        Params { named, tail }
    }
}

/// Matches a request method and path against the provided patterns,
/// returning the matched value (if any) and any captured parameters
/// from the path.
///
/// # Returns
///
/// `Result<(T, match_route::Params), match_route::Error>` where
/// `T` is the type of the match arms.
#[macro_export]
macro_rules! match_route {
    ($request_path:expr, {
        $($pattern:literal => {
            $($result:expr),* $(,)?
        }),* $(,)?
    }) => {{
        let result = $crate::_match_route_regex!(
            $request_path,
            {
                $($crate::duid_routers::pattern($pattern) => {
                    $($result),*
                }),*
            }
        );
        result.map(move |(value, names, captures)| {
            (value, $crate::duid_routers::Params::from_captures(
                names,
                captures
            ))
        })
    }};
}

/// Like `match_route!` but each pattern is a raw Regex pattern. If
/// you want to match entire URLs be sure to prefix each pattern with
/// `^` and end with `$` as no conversion is made before passing the
/// pattern to `Regexp::new()` for compilation.
///
/// Intstead of returning a Params struct with the match path
/// parameters, this macro will return the raw regexp::Captures.
///
/// # Returns
///
/// `Result<(T, regex::Captures<'t>), match_route::Error>` where `T`
/// is the type of the match arms, and the lifetime `'t` relates to
/// the provided `path` argument.
///
/// # Panics
///
/// If any of the regular expressions are invalid this macro will
/// panic on first invocation.
#[macro_export]
macro_rules! match_route_regex {
    ($request_path:expr, {
        $($pattern:expr => {
            $($result:expr),* $(,)?
        }),* $(,)?
    }) => {{
        let result = $crate::_match_route_regex!(
            $request_path,
            {$($pattern => {$($result),*}),*}
        );
        result.map(move |(value, _names, captures)| {
            (value, captures)
        })
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _match_route_regex {
    ($request_path:expr, {
        $($pattern:expr => {
            $($value:expr),* $(,)?
        }),* $(,)?
    }) => {{
        use $crate::duid_routers::{Error, Regex};
        use $crate::duid_routers::lazy_static::lazy_static;
        let path = $request_path;
        loop {
            $({
                lazy_static! {
                    static ref RE: Regex = Regex::new(
                        $pattern.as_ref()
                    ).unwrap();
                }
                if let Some(captures) = RE.captures(path) {
                    $(
                        break Ok((
                            $value,
                            RE.capture_names(),
                            captures,
                        ));
                    )*
                }
            };)*
            break Err(Error::NotFound);
        }
    }};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_exact_path() {
        let result = match_route!("/bar", {
            "/foo" => {
                111
            },
            "/bar" => {
                222
            },
            "/baz" => {
                333
            }
        });
        let (value, params) = result.unwrap();
        assert!(params.is_empty());
        assert_eq!(params.len(), 0);
        assert_eq!(value, 222);
    }

    #[test]
    fn match_method() {
        let result = match_route!("/foo", {
            "/foo" => {
                111
            },
            "/bar" => {
                222
            },
            "/baz" => {
                333
            }
        });
        let (value, params) = result.unwrap();
        assert!(params.is_empty());
        assert_eq!(params.len(), 0);
        assert_eq!(value, 111);
    }

    #[test]
    fn no_matching_path() {
        let result = match_route!("/asdf", {
            "/foo" => {
                111,
            },
            "/bar" => {
                222
            },
            "/baz" => {
                333
            }
        });
        assert_eq!(result.unwrap_err(), Error::NotFound);

    }

    #[test]
    fn match_named_params() {
        let result = match_route!("/user/11/articles/foo-bar", {
            "/user/:id/articles/:slug" => {
                123
            },
        });
        let (value, params) = result.unwrap();
        assert_eq!(params.get("id").unwrap(), "11");
        assert_eq!(params.get("slug").unwrap(), "foo-bar");
        assert_eq!(params.len(), 2);
        assert!(!params.is_empty());
        assert_eq!(value, 123);
    }

    #[test]
    fn tail_capture_includes_leading_forward_slash() {
        let result = match_route!("/user/11/articles/foo-bar", {
            "/user/*" => {
                123
            },
        });
        let (value, params) = result.unwrap();
        assert_eq!(params.tail().unwrap(), "/11/articles/foo-bar");
        assert_eq!(params.len(), 1);
        assert_eq!(value, 123);
    }

    #[test]
    fn capture_entire_path() {
        let result = match_route!("/user/11/articles/foo-bar", {
            "/foo/bar" => {
                111,
            },
            "*" => {
                123,
            },
        });
        let (value, params) = result.unwrap();
        assert_eq!(params.tail().unwrap(), "/user/11/articles/foo-bar");
        assert_eq!(params.len(), 1);
        assert_eq!(value, 123);
    }

    #[test]
    fn pattern_must_match_full_path() {
        let result = match_route!("/foo/bar", {
            "/bar" => {
                123
            },
            "/foo" => {
                456
            },
        });
        assert_eq!(result.unwrap_err(), Error::NotFound);
    }

    #[test]
    fn regex_capture_groups() {
        let result = match_route_regex!("/user/11/articles/foo-bar", {
            r"^/user/(\d+)/articles/([a-z-]+)$" => {
                123
            },
        });
        let (value, captures) = result.unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), "/user/11/articles/foo-bar");
        assert_eq!(captures.get(1).unwrap().as_str(), "11");
        assert_eq!(captures.get(2).unwrap().as_str(), "foo-bar");
        assert_eq!(captures.len(), 3);
        assert_eq!(value, 123);
    }

}