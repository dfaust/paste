//! [![github]](https://github.com/dtolnay/paste)&ensp;[![crates-io]](https://crates.io/crates/paste)&ensp;[![docs-rs]](https://docs.rs/paste)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! The nightly-only [`concat_idents!`] macro in the Rust standard library is
//! notoriously underpowered in that its concatenated identifiers can only refer to
//! existing items, they can never be used to define something new.
//!
//! [`concat_idents!`]: https://doc.rust-lang.org/std/macro.concat_idents.html
//!
//! This crate provides a flexible way to paste together identifiers in a macro,
//! including using pasted identifiers to define new items.
//!
//! This approach works with any stable or nightly Rust compiler 1.30+.
//!
//! <br>
//!
//! # Pasting identifiers
//!
//! Within the `paste!` macro, identifiers inside `[<`...`>]` are pasted
//! together to form a single identifier.
//!
//! ```
//! use paste::paste;
//!
//! paste! {
//!     // Defines a const called `QRST`.
//!     const [<Q R S T>]: &str = "success!";
//! }
//!
//! fn main() {
//!     assert_eq!(
//!         paste! { [<Q R S T>].len() },
//!         8,
//!     );
//! }
//! ```
//!
//! <br><br>
//!
//! # More elaborate examples
//!
//! This program demonstrates how you may want to bundle a paste invocation inside
//! of a more convenient user-facing macro of your own. Here the `routes!(A, B)`
//! macro expands to a vector containing `ROUTE_A` and `ROUTE_B`.
//!
//! ```
//! use paste::paste;
//!
//! const ROUTE_A: &str = "/a";
//! const ROUTE_B: &str = "/b";
//!
//! macro_rules! routes {
//!     ($($route:ident),*) => {{
//!         paste! {
//!             vec![$( [<ROUTE_ $route>] ),*]
//!         }
//!     }}
//! }
//!
//! fn main() {
//!     let routes = routes!(A, B);
//!     assert_eq!(routes, vec!["/a", "/b"]);
//! }
//! ```
//!
//! The next example shows a macro that generates accessor methods for some struct
//! fields.
//!
//! ```
//! use paste::paste;
//!
//! macro_rules! make_a_struct_and_getters {
//!     ($name:ident { $($field:ident),* }) => {
//!         // Define a struct. This expands to:
//!         //
//!         //     pub struct S {
//!         //         a: String,
//!         //         b: String,
//!         //         c: String,
//!         //     }
//!         pub struct $name {
//!             $(
//!                 $field: String,
//!             )*
//!         }
//!
//!         // Build an impl block with getters. This expands to:
//!         //
//!         //     impl S {
//!         //         pub fn get_a(&self) -> &str { &self.a }
//!         //         pub fn get_b(&self) -> &str { &self.b }
//!         //         pub fn get_c(&self) -> &str { &self.c }
//!         //     }
//!         paste! {
//!             impl $name {
//!                 $(
//!                     pub fn [<get_ $field>](&self) -> &str {
//!                         &self.$field
//!                     }
//!                 )*
//!             }
//!         }
//!     }
//! }
//!
//! make_a_struct_and_getters!(S { a, b, c });
//!
//! fn call_some_getters(s: &S) -> bool {
//!     s.get_a() == s.get_b() && s.get_c().is_empty()
//! }
//! #
//! # fn main() {}
//! ```
//!
//! <br><br>
//!
//! # Case conversion
//!
//! Use `$var:lower` or `$var:upper` in the segment list to convert an
//! interpolated segment to lower- or uppercase as part of the paste. For
//! example, `[<ld_ $reg:lower _expr>]` would paste to `ld_bc_expr` if invoked
//! with $reg=`Bc`.
//!
//! Use `$var:snake` to convert CamelCase input to snake\_case.
//! Use `$var:camel` to convert snake\_case to CamelCase.
//! These compose, so for example `$var:snake:upper` would give you SCREAMING\_CASE.
//!
//! The precise Unicode conversions are as defined by [`str::to_lowercase`] and
//! [`str::to_uppercase`].
//!
//! [`str::to_lowercase`]: https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase
//! [`str::to_uppercase`]: https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase

#![allow(clippy::needless_doctest_main)]

mod error;

use crate::error::{Error, Result};
use proc_macro::{
    token_stream, Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree,
};
use std::iter::{self, FromIterator, Peekable};
use std::panic;

#[proc_macro]
pub fn paste(input: TokenStream) -> TokenStream {
    let mut contains_paste = false;
    match expand(input, &mut contains_paste) {
        Ok(expanded) => expanded,
        Err(err) => err.to_compile_error(),
    }
}

#[doc(hidden)]
#[proc_macro]
pub fn item(input: TokenStream) -> TokenStream {
    paste(input)
}

#[doc(hidden)]
#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    paste(input)
}

fn expand(input: TokenStream, contains_paste: &mut bool) -> Result<TokenStream> {
    let mut expanded = TokenStream::new();
    let mut lookbehind = Lookbehind::Other;
    let mut prev_none_group = None::<Group>;
    let mut tokens = input.into_iter().peekable();
    loop {
        let token = tokens.next();
        if let Some(group) = prev_none_group.take() {
            if match (&token, tokens.peek()) {
                (Some(TokenTree::Punct(fst)), Some(TokenTree::Punct(snd))) => {
                    fst.as_char() == ':' && snd.as_char() == ':' && fst.spacing() == Spacing::Joint
                }
                _ => false,
            } {
                expanded.extend(group.stream());
                *contains_paste = true;
            } else {
                expanded.extend(iter::once(TokenTree::Group(group)));
            }
        }
        match token {
            Some(TokenTree::Group(group)) => {
                let delimiter = group.delimiter();
                let content = group.stream();
                let span = group.span();
                if delimiter == Delimiter::Bracket && is_paste_operation(&content) {
                    let segments = parse_bracket_as_segments(content, span)?;
                    let pasted = paste_segments(span, &segments)?;
                    expanded.extend(pasted);
                    *contains_paste = true;
                } else if delimiter == Delimiter::None && is_flat_group(&content) {
                    expanded.extend(content);
                    *contains_paste = true;
                } else if delimiter == Delimiter::Bracket
                    && matches!(lookbehind, Lookbehind::Pound | Lookbehind::PoundBang)
                    && is_pasted_doc(&content)
                {
                    unimplemented!()
                } else {
                    let mut group_contains_paste = false;
                    let nested = expand(content, &mut group_contains_paste)?;
                    let group = if group_contains_paste {
                        let mut group = Group::new(delimiter, nested);
                        group.set_span(span);
                        *contains_paste = true;
                        group
                    } else {
                        group.clone()
                    };
                    if delimiter != Delimiter::None {
                        expanded.extend(iter::once(TokenTree::Group(group)));
                    } else if lookbehind == Lookbehind::DoubleColon {
                        expanded.extend(group.stream());
                        *contains_paste = true;
                    } else {
                        prev_none_group = Some(group);
                    }
                }
                lookbehind = Lookbehind::Other;
            }
            Some(TokenTree::Punct(punct)) => {
                lookbehind = match punct.as_char() {
                    ':' if lookbehind == Lookbehind::JointColon => Lookbehind::DoubleColon,
                    ':' if punct.spacing() == Spacing::Joint => Lookbehind::JointColon,
                    '#' => Lookbehind::Pound,
                    '!' if lookbehind == Lookbehind::Pound => Lookbehind::PoundBang,
                    _ => Lookbehind::Other,
                };
                expanded.extend(iter::once(TokenTree::Punct(punct)));
            }
            Some(other) => {
                lookbehind = Lookbehind::Other;
                expanded.extend(iter::once(other));
            }
            None => return Ok(expanded),
        }
    }
}

#[derive(PartialEq)]
enum Lookbehind {
    JointColon,
    DoubleColon,
    Pound,
    PoundBang,
    Other,
}

// https://github.com/dtolnay/paste/issues/26
fn is_flat_group(input: &TokenStream) -> bool {
    #[derive(PartialEq)]
    enum State {
        Init,
        Ident,
        Literal,
        Apostrophe,
        Lifetime,
        Colon1,
        Colon2,
    }

    let mut state = State::Init;
    for tt in input.clone() {
        state = match (state, &tt) {
            (State::Init, TokenTree::Ident(_)) => State::Ident,
            (State::Init, TokenTree::Literal(_)) => State::Literal,
            (State::Init, TokenTree::Punct(punct)) if punct.as_char() == '\'' => State::Apostrophe,
            (State::Apostrophe, TokenTree::Ident(_)) => State::Lifetime,
            (State::Ident, TokenTree::Punct(punct))
                if punct.as_char() == ':' && punct.spacing() == Spacing::Joint =>
            {
                State::Colon1
            }
            (State::Colon1, TokenTree::Punct(punct))
                if punct.as_char() == ':' && punct.spacing() == Spacing::Alone =>
            {
                State::Colon2
            }
            (State::Colon2, TokenTree::Ident(_)) => State::Ident,
            _ => return false,
        };
    }

    state == State::Ident || state == State::Literal || state == State::Lifetime
}

fn is_pasted_doc(input: &TokenStream) -> bool {
    #[derive(PartialEq)]
    enum State {
        Init,
        Doc,
        Equal,
        First,
        Rest,
    }

    let mut state = State::Init;
    for tt in input.clone() {
        state = match (state, &tt) {
            (State::Init, TokenTree::Ident(ident)) if ident.to_string() == "doc" => State::Doc,
            (State::Doc, TokenTree::Punct(punct)) if punct.as_char() == '=' => State::Equal,
            (State::Equal, tt) if is_stringlike(tt) => State::First,
            (State::First, tt) | (State::Rest, tt) if is_stringlike(tt) => State::Rest,
            _ => return false,
        };
    }

    state == State::Rest
}

fn is_stringlike(token: &TokenTree) -> bool {
    escaped_string_value(token).is_some()
}

fn escaped_string_value(token: &TokenTree) -> Option<String> {
    match token {
        TokenTree::Ident(ident) => Some(ident.to_string()),
        TokenTree::Literal(literal) => {
            let mut repr = literal.to_string();
            if repr.starts_with('b') || repr.starts_with('\'') {
                None
            } else if repr.starts_with('"') {
                repr.truncate(repr.len() - 1);
                repr.remove(0);
                Some(repr)
            } else if repr.starts_with('r') {
                let begin = repr.find('"').unwrap() + 1;
                let end = repr.rfind('"').unwrap();
                Some(repr[begin..end].escape_default().to_string())
            } else {
                Some(repr)
            }
        }
        TokenTree::Group(group) => {
            if group.delimiter() != Delimiter::None {
                return None;
            }
            let mut inner = group.stream().into_iter();
            let first = inner.next()?;
            if inner.next().is_none() {
                escaped_string_value(&first)
            } else {
                None
            }
        }
        TokenTree::Punct(_) => None,
    }
}

struct LitStr {
    value: String,
    span: Span,
}

struct Colon {
    span: Span,
}

enum Segment {
    String(String),
    Apostrophe(Span),
    Env(LitStr),
    Modifier(Colon, Ident),
}

fn is_paste_operation(input: &TokenStream) -> bool {
    let mut tokens = input.clone().into_iter();

    match &tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '<' => {}
        _ => return false,
    }

    let mut has_token = false;
    loop {
        match &tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                return has_token && tokens.next().is_none();
            }
            Some(_) => has_token = true,
            None => return false,
        }
    }
}

fn parse_bracket_as_segments(input: TokenStream, scope: Span) -> Result<Vec<Segment>> {
    let mut tokens = input.into_iter().peekable();

    match &tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '<' => {}
        Some(wrong) => return Err(Error::new(wrong.span(), "expected `<`")),
        None => return Err(Error::new(scope, "expected `[< ... >]`")),
    }

    let segments = parse_segments(&mut tokens, scope)?;

    match &tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {}
        Some(wrong) => return Err(Error::new(wrong.span(), "expected `>`")),
        None => return Err(Error::new(scope, "expected `[< ... >]`")),
    }

    match tokens.next() {
        Some(unexpected) => Err(Error::new(
            unexpected.span(),
            "unexpected input, expected `[< ... >]`",
        )),
        None => Ok(segments),
    }
}

fn parse_segments(
    tokens: &mut Peekable<token_stream::IntoIter>,
    scope: Span,
) -> Result<Vec<Segment>> {
    let mut segments = Vec::new();
    while match tokens.peek() {
        None => false,
        Some(TokenTree::Punct(punct)) => punct.as_char() != '>',
        Some(_) => true,
    } {
        match tokens.next().unwrap() {
            TokenTree::Ident(ident) => {
                let mut fragment = ident.to_string();
                if fragment.starts_with("r#") {
                    fragment = fragment.split_off(2);
                }
                if fragment == "env"
                    && match tokens.peek() {
                        Some(TokenTree::Punct(punct)) => punct.as_char() == '!',
                        _ => false,
                    }
                {
                    tokens.next().unwrap(); // `!`
                    let expect_group = tokens.next();
                    let parenthesized = match &expect_group {
                        Some(TokenTree::Group(group))
                            if group.delimiter() == Delimiter::Parenthesis =>
                        {
                            group
                        }
                        Some(wrong) => return Err(Error::new(wrong.span(), "expected `(`")),
                        None => return Err(Error::new(scope, "expected `(` after `env!`")),
                    };
                    let mut inner = parenthesized.stream().into_iter();
                    let lit = match inner.next() {
                        Some(TokenTree::Literal(lit)) => lit,
                        Some(wrong) => {
                            return Err(Error::new(wrong.span(), "expected string literal"))
                        }
                        None => {
                            return Err(Error::new2(
                                ident.span(),
                                parenthesized.span(),
                                "expected string literal as argument to env! macro",
                            ))
                        }
                    };
                    let lit_string = lit.to_string();
                    if lit_string.starts_with('"')
                        && lit_string.ends_with('"')
                        && lit_string.len() >= 2
                    {
                        // TODO: maybe handle escape sequences in the string if
                        // someone has a use case.
                        segments.push(Segment::Env(LitStr {
                            value: lit_string[1..lit_string.len() - 1].to_owned(),
                            span: lit.span(),
                        }));
                    } else {
                        return Err(Error::new(lit.span(), "expected string literal"));
                    }
                    if let Some(unexpected) = inner.next() {
                        return Err(Error::new(
                            unexpected.span(),
                            "unexpected token in env! macro",
                        ));
                    }
                } else {
                    segments.push(Segment::String(fragment));
                }
            }
            TokenTree::Literal(lit) => {
                let mut lit_string = lit.to_string();
                if lit_string.contains(&['#', '\\', '.', '+'][..]) {
                    return Err(Error::new(lit.span(), "unsupported literal"));
                }
                lit_string = lit_string
                    .replace('"', "")
                    .replace('\'', "")
                    .replace('-', "_");
                segments.push(Segment::String(lit_string));
            }
            TokenTree::Punct(punct) => match punct.as_char() {
                '_' => segments.push(Segment::String("_".to_owned())),
                '\'' => segments.push(Segment::Apostrophe(punct.span())),
                ':' => {
                    let colon = Colon { span: punct.span() };
                    let ident = match tokens.next() {
                        Some(TokenTree::Ident(ident)) => ident,
                        wrong => {
                            let span = wrong.as_ref().map_or(scope, TokenTree::span);
                            return Err(Error::new(span, "expected identifier after `:`"));
                        }
                    };
                    segments.push(Segment::Modifier(colon, ident));
                }
                _ => return Err(Error::new(punct.span(), "unexpected punct")),
            },
            TokenTree::Group(group) => {
                if group.delimiter() == Delimiter::None {
                    let mut inner = group.stream().into_iter().peekable();
                    let nested = parse_segments(&mut inner, group.span())?;
                    if let Some(unexpected) = inner.next() {
                        return Err(Error::new(unexpected.span(), "unexpected token"));
                    }
                    segments.extend(nested);
                } else {
                    return Err(Error::new(group.span(), "unexpected token"));
                }
            }
        }
    }
    Ok(segments)
}

fn paste_segments(span: Span, segments: &[Segment]) -> Result<TokenStream> {
    let mut evaluated = Vec::new();
    let mut is_lifetime = false;

    for segment in segments {
        match segment {
            Segment::String(segment) => {
                evaluated.push(segment.clone());
            }
            Segment::Apostrophe(span) => {
                if is_lifetime {
                    return Err(Error::new(*span, "unexpected lifetime"));
                }
                is_lifetime = true;
            }
            Segment::Env(var) => {
                let resolved = match std::env::var(&var.value) {
                    Ok(resolved) => resolved,
                    Err(_) => {
                        return Err(Error::new(
                            var.span,
                            &format!("no such env var: {:?}", var.value),
                        ));
                    }
                };
                let resolved = resolved.replace('-', "_");
                evaluated.push(resolved);
            }
            Segment::Modifier(colon, ident) => {
                let last = match evaluated.pop() {
                    Some(last) => last,
                    None => {
                        return Err(Error::new2(colon.span, ident.span(), "unexpected modifier"))
                    }
                };
                match ident.to_string().as_str() {
                    "lower" => {
                        evaluated.push(last.to_lowercase());
                    }
                    "upper" => {
                        evaluated.push(last.to_uppercase());
                    }
                    "snake" => {
                        let mut acc = String::new();
                        let mut prev = '_';
                        for ch in last.chars() {
                            if ch.is_uppercase() && prev != '_' {
                                acc.push('_');
                            }
                            acc.push(ch);
                            prev = ch;
                        }
                        evaluated.push(acc.to_lowercase());
                    }
                    "camel" => {
                        let mut acc = String::new();
                        let mut prev = '_';
                        for ch in last.chars() {
                            if ch != '_' {
                                if prev == '_' {
                                    for chu in ch.to_uppercase() {
                                        acc.push(chu);
                                    }
                                } else if prev.is_uppercase() {
                                    for chl in ch.to_lowercase() {
                                        acc.push(chl);
                                    }
                                } else {
                                    acc.push(ch);
                                }
                            }
                            prev = ch;
                        }
                        evaluated.push(acc);
                    }
                    _ => {
                        return Err(Error::new2(
                            colon.span,
                            ident.span(),
                            "unsupported modifier",
                        ));
                    }
                }
            }
        }
    }

    let pasted = evaluated.into_iter().collect::<String>();
    let ident = match panic::catch_unwind(|| Ident::new(&pasted, span)) {
        Ok(ident) => TokenTree::Ident(ident),
        Err(_) => {
            return Err(Error::new(
                span,
                &format!("`{:?}` is not a valid identifier", pasted),
            ));
        }
    };
    let tokens = if is_lifetime {
        let apostrophe = TokenTree::Punct(Punct::new('\'', Spacing::Joint));
        vec![apostrophe, ident]
    } else {
        vec![ident]
    };
    Ok(TokenStream::from_iter(tokens))
}
