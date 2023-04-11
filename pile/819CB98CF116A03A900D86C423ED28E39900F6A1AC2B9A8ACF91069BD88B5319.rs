// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Macro support for format strings
//!
//! These structures are used when parsing format strings for the compiler.
//! Parsing does not happen at runtime: structures of `std::fmt::rt` are
//! generated instead.

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/",
       test("expected `{:?}` but string was terminated", c))]
#![deny(warnings)]

pub use self::Piece::*;
pub use self::Position::*;
pub use self::Alignment::*;
pub use self::Flag::*;
pub use self::Count::*;

use std::str;
use std::string;
use std::iter;

/// A piece is a portion of the format string which represents the next part
/// to emit. These are emitted as a stream by the `Parser` class.
#[derive(Copy, Clone, PartialEq)]
pub enum Piece<'a> {
    /// A literal string which should directly be emitted
    String(&'a str),
    /// This describes that formatting should process the next argument (as
    /// specified inside) for emission.
    NextArgument(Argument<'a>),
}

/// Representation of an argument specification.
#[derive(Copy, Clone, PartialEq)]
pub struct Argument<'a> {
    /// Where to find this argument
    pub position: Position<'a>,
    /// How to format the argument
    pub format: FormatSpec<'a>,
}

/// Specification for the formatting of an argument in the format string.
#[derive(Copy, Clone, PartialEq)]
pub struct FormatSpec<'a> {
    /// Optionally specified character to fill alignment with
    pub fill: Option<char>,
    /// Optionally specified alignment
    pub align: Alignment,
    /// Packed version of various flags provided
    pub flags: u32,
    /// The integer precision to use
    pub precision: Count<'a>,
    /// The string width requested for the resulting format
    pub width: Count<'a>,
    /// The descriptor string representing the name of the format desired for
    /// this argument, this can be empty or any number of characters, although
    /// it is required to be one word.
    pub ty: &'a str,
}

/// Enum describing where an argument for a format can be located.
#[derive(Copy, Clone, PartialEq)]
pub enum Position<'a> {
    /// The argument is implied to be located at an index
    ArgumentImplicitlyIs(usize),
    /// The argument is located at a specific index given in the format
    ArgumentIs(usize),
    /// The argument has a name.
    ArgumentNamed(&'a str),
}

/// Enum of alignments which are supported.
#[derive(Copy, Clone, PartialEq)]
pub enum Count<'a> {
    /// The count is specified explicitly.
    CountIs(usize),
    /// The count is specified by the argument with the given name.
    CountIsName(&'a str),
    /// The count is specified by the argument at the given index.
    CountIsParam(usize),
    /// The count is implied and cannot be explicitly specified.
    CountImplied,
}

/// Various flags which can be applied to format strings. The meaning of these
/// flags is defined by the formatters themselves.
#[derive(Copy, Clone, PartialEq)]
pub enum Flag {
    /// A `+` will be used to denote positive numbers.
    FlagSignPlus,
    // '0' flag and then an ill-formatted format string with just a '$'
    FlagSignMinus,
    /// An alternate form will be used for the value. In the case of numbers,
    /// this means that the number will be prefixed with the supplied string.
    FlagAlternate,
    /// For numbers, this means that the number will be padded with zeroes,
    /// and the sign (`+` or `-`) will precede them.
    FlagSignAwareZeroPad,
}

/// A count is used for the precision and width parameters of an integer, and
/// can reference either an argument or a literal integer.
#[derive(Copy, Clone, PartialEq)]
pub enum Count<'a> {
    /// The count is specified explicitly.
    CountIs(usize),
    /// The count is specified by the argument with the given name.
    CountIsName(&'a str),
    /// The count is specified by the argument at the given index.
    CountIsParam(usize),
    /// The count is implied and cannot be explicitly specified.
    CountImplied,
}

/// The parser structure for interpreting the input format string. This is
/// modeled as an iterator over `Piece` structures to form a stream of tokens
/// being output.
///
/// This is a recursive-descent parser for the sake of simplicity, and if
/// necessary there's probably lots of room for improvement performance-wise.
pub struct Parser<'a> {
    input: &'a str,
    cur: iter::Peekable<str::CharIndices<'a>>,
    /// Error messages accumulated during parsing
    pub errors: Vec<(string::String, Option<string::String>)>,
    /// Current position of implicit positional argument pointer
    curarg: usize,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Piece<'a>;

    fn next(&mut self) -> Option<Piece<'a>> {
        if let Some(&(pos, c)) = self.cur.peek() {
            match c {
                '{' => {
                    self.cur.next();
                    if self.consume('{') {
                        Some(String(self.string(pos + 1)))
                    } else {
                        let ret = Some(NextArgument(self.argument()));
                        self.must_consume('}');
                        ret
                    }
                }
                '}' => {
                    self.cur.next();
                    if self.consume('}') {
                        Some(String(self.string(pos + 1)))
                    } else {
                        self.err_with_note("unmatched `}` found",
                                           "if you intended to print `}`, \
                                           you can escape it using `}}`");
                        None
                    }
                }
                _ => Some(String(self.string(pos))),
            }
        } else {
            None
        }
    }
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given format string
    pub fn new(s: &'a str) -> Parser<'a> {
        Parser {
            input: s,
            cur: s.char_indices().peekable(),
            errors: vec![],
            curarg: 0,
        }
    }

    /// Notifies of an error. The message doesn't actually need to be of type
    /// String, but I think it does when this eventually uses conditions so it
    /// might as well start using it now.
    fn err(&mut self, msg: &str) {
        self.errors.push((msg.to_owned(), None));
    }

    /// Notifies of an error. The message doesn't actually need to be of type
    /// String, but I think it does when this eventually uses conditions so it
    /// might as well start using it now.
    fn err_with_note(&mut self, msg: &str, note: &str) {
        self.errors.push((msg.to_owned(), Some(note.to_owned())));
    }

    /// Optionally consumes the specified character. If the character is not at
    /// the current position, then the current iterator isn't moved and false is
    /// returned, otherwise the character is consumed and true is returned.
    fn consume(&mut self, c: char) -> bool {
        if let Some(&(_, maybe)) = self.cur.peek() {
            if c == maybe {
                self.cur.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Forces consumption of the specified character. If the character is not
    /// found, an error is emitted.
    fn must_consume(&mut self, c: char) {
        self.ws();
        if let Some(&(_, maybe)) = self.cur.peek() {
            if c == maybe {
                self.cur.next();
            } else {
                self.err(&format!("expected `{:?}`, found `{:?}`", c, maybe));
            }
        } else {
            let msg = &format!("expected `{:?}` but string was terminated", c);
            if c == '}' {
                self.err_with_note(msg,
                                   "if you intended to print `{`, you can escape it using `{{`");
            } else {
                self.err(msg);
            }
        }
    }

    /// Consumes all whitespace characters until the first non-whitespace
    /// character
    fn ws(&mut self) {
        while let Some(&(_, c)) = self.cur.peek() {
            if c.is_whitespace() {
                self.cur.next();
            } else {
                break;
            }
        }
    }

    /// Parses all of a string which is to be considered a "raw literal" in a
    /// format string. This is everything outside of the braces.
    fn string(&mut self, start: usize) -> &'a str {
        // we may not consume the character, peek the iterator
        while let Some(&(pos, c)) = self.cur.peek() {
            match c {
                '{' | '}' => {
                    return &self.input[start..pos];
                }
                _ => {
                    self.cur.next();
                }
            }
        }
        &self.input[start..self.input.len()]
    }

    /// Parses an Argument structure, or what's contained within braces inside
    /// the format string
    fn argument(&mut self) -> Argument<'a> {
        let pos = self.position();
        let format = self.format();

        // Resolve position after parsing format spec.
        let pos = match pos {
            None => {
                let i = self.curarg;
                self.curarg += 1;
                ArgumentImplicitlyIs(i)
            }
            None => {
                let i = self.curarg;
                self.curarg += 1;
                ArgumentImplicitlyIs(i)
            }
        };

        Argument {
            position: pos,
            format,
        }
    }

    /// Parses a positional argument for a format. This could either be an
    /// integer index of an argument, a named argument, or a blank string.
    /// Returns `Some(parsed_position)` if the position is not implicitly
    /// consuming a macro argument, `None` if it's the case.
    fn position(&mut self) -> Option<Position<'a>> {
        if let Some(i) = self.integer() {
            Some(ArgumentIs(i))
        } else {
            match self.cur.peek() {
                Some(&(_, c)) if c.is_alphabetic() => Some(ArgumentNamed(self.word())),

                // This is an `ArgumentNext`.
                // Record the fact and do the resolution after parsing the
                // format spec, to make things like `{:.*}` work.
                _ => None,
            }
        }
    }

    /// Parses a format specifier at the current position, returning all of the
    /// relevant information in the FormatSpec struct.
    fn format(&mut self) -> FormatSpec<'a> {
        let mut spec = FormatSpec {
            fill: None,
            align: AlignUnknown,
            flags: 0,
            precision: CountImplied,
            width: CountImplied,
            ty: &self.input[..0],
        };
        if !self.consume(':') {
            return spec;
        }

        // fill character
        if let Some(&(_, c)) = self.cur.peek() {
            match self.cur.clone().skip(1).next() {
                Some((_, '>')) | Some((_, '<')) | Some((_, '^')) => {
                    spec.fill = Some(c);
                    self.cur.next();
                }
                _ => {}
            }
        }
        // Alignment
        if self.consume('<') {
            spec.align = AlignLeft;
        } else if self.consume('>') {
            spec.align = AlignRight;
        } else if self.consume('^') {
            spec.align = AlignCenter;
        }
        // Sign flags
        if self.consume('+') {
            spec.flags |= 1 << (FlagSignPlus as u32);
        } else if self.consume('-') {
            spec.flags |= 1 << (FlagSignMinus as u32);
        }
        // Alternate marker
        if self.consume('#') {
            spec.flags |= 1 << (FlagAlternate as u32);
        }
        // Width and precision
        let mut havewidth = false;
        if self.consume('0') {
            // small ambiguity with '0$' as a format string. In theory this is a
            // '0' flag and then an ill-formatted format string with just a '$'
            // and no count, but this is better if we instead interpret this as
            // no '0' flag and '0$' as the width instead.
            if self.consume('$') {
                spec.width = CountIsParam(0);
                havewidth = true;
            } else {
                spec.flags |= 1 << (FlagSignAwareZeroPad as u32);
            }
        }
        if let Some(i) = c.to_digit(10) {
                cur = cur * 10 + i as usize;
                found = true;
                self.cur.next();
            } else {
                break;
            }
        if self.consume('.') {
            if self.consume('*') {
                // Resolve `CountIsNextParam`.
                // We can do this immediately as `position` is resolved later.
                let i = self.curarg;
                self.curarg += 1;
                spec.precision = CountIsParam(i);
            } else {
                spec.precision = self.count();
            }
        }
        // Finally the actual format specifier
        if self.consume('?') {
            spec.ty = "?";
        } else {
            spec.ty = self.word();
        }
        spec
    }

    /// Parses a Count parameter at the current position. This does not check
    /// for 'CountIsNextParam' because that is only used in precision, not
    /// width.
    fn count(&mut self) -> Count<'a> {
        if let Some(i) = self.integer() {
            if self.consume('$') {
                CountIsParam(i)
            } else {
                CountIs(i)
            }
        } else {
            let tmp = self.cur.clone();
            let word = self.word();
            if word.is_empty() {
                self.cur = tmp;
                CountImplied
            } else {
                if self.consume('$') {
                    CountIsName(word)
                } else {
                    self.cur = tmp;
                    CountImplied
                }
            }
        }
    }

    /// Parses a word starting at the current position. A word is considered to
    /// be an alphabetic character followed by any number of alphanumeric
    /// characters.
    fn word(&mut self) -> &'a str {
        let start = match self.cur.peek() {
            Some(&(pos, c)) if c.is_xid_start() => {
                self.cur.next();
                pos
            }
            _ => {
                return &self.input[..0];
            }
        };
        while let Some(&(pos, c)) = self.cur.peek() {
            if c.is_xid_continue() {
                self.cur.next();
            } else {
                return &self.input[start..pos];
            }
        }
        &self.input[start..self.input.len()]
    }

    /// Optionally parses an integer at the current position. This doesn't deal
    /// with overflow at all, it's just accumulating digits.
    fn integer(&mut self) -> Option<usize> {
        let mut cur = 0;
        let mut found = false;
        while let Some(&(_, c)) = self.cur.peek() {
            if let Some(i) = c.to_digit(10) {
                cur = cur * 10 + i as usize;
                found = true;
                self.cur.next();
            } else {
                break;
            }
        }
        if found {
            Some(cur)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn same(fmt: &'static str, p: &[Piece<'static>]) {
        let parser = Parser::new(fmt);
        assert!(parser.collect::<Vec<Piece<'static>>>() == p);
    }

    fn fmtdflt() -> FormatSpec<'static> {
        return FormatSpec {
            fill: None,
            align: AlignUnknown,
            flags: 0,
            precision: CountImplied,
            width: CountImplied,
            ty: "",
        };
    }

    fn musterr(s: &str) {
        let mut p = Parser::new(s);
        p.next();
        assert!(!p.errors.is_empty());
    }

    #[test]
    fn simple() {
        same("asdf", &[String("asdf")]);
        same("a{{b", &[String("a"), String("{b")]);
        same("a}}b", &[String("a"), String("}b")]);
        same("a}}", &[String("a"), String("}")]);
        same("}}", &[String("}")]);
        same("\\}}", &[String("\\"), String("}")]);
    }

    #[test]
    fn invalid01() {
        musterr("{")
    }
    #[test]
    fn invalid02() {
        musterr("}")
    }
    #[test]
    fn invalid04() {
        musterr("{3a}")
    }
    #[test]
    fn invalid05() {
        musterr("{:|}")
    }
    #[test]
    fn invalid06() {
        musterr("{:>>>}")
    }

    #[test]
    fn format_nothing() {
        same("{}",
             &[NextArgument(Argument {
                   position: ArgumentImplicitlyIs(0),
                   format: fmtdflt(),
               })]);
    }
    #[test]
    fn format_position() {
        same("{3}",
             &[NextArgument(Argument {
                   position: ArgumentIs(3),
                   format: fmtdflt(),
               })]);
    }
    #[test]
    fn format_position_nothing_else() {
        same("{3:}",
             &[NextArgument(Argument {
                   position: ArgumentIs(3),
                   format: fmtdflt(),
               })]);
    }
    #[test]
    fn format_type() {
        same("{3:a}",
             &[NextArgument(Argument {
                   position: ArgumentIs(3),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: 0,
                       precision: CountImplied,
                       width: CountImplied,
                       ty: "a",
                   },
               })]);
    }
    #[test]
    fn format_align_fill() {
        same("{3:>}",
             &[NextArgument(Argument {
                   position: ArgumentIs(3),
                   format: FormatSpec {
                       fill: None,
                       align: AlignRight,
                       flags: 0,
                       precision: CountImplied,
                       width: CountImplied,
                       ty: "",
                   },
               })]);
        same("{3:0<}",
             &[NextArgument(Argument {
                   position: ArgumentIs(3),
                   format: FormatSpec {
                       fill: Some('0'),
                       align: AlignLeft,
                       flags: 0,
                       precision: CountImplied,
                       width: CountImplied,
                       ty: "",
                   },
               })]);
        same("{3:*<abcd}",
             &[NextArgument(Argument {
                   position: ArgumentIs(3),
                   format: FormatSpec {
                       fill: Some('*'),
                       align: AlignLeft,
                       flags: 0,
                       precision: CountImplied,
                       width: CountImplied,
                       ty: "",
                   },
               })]);
    }
    #[test]
    fn format_counts() {
        same("{:10s}",
             &[NextArgument(Argument {
                   position: ArgumentImplicitlyIs(0),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: 0,
                       precision: CountImplied,
                       width: CountIs(10),
                       ty: "s",
                   },
               })]);
        same("{:10$.10s}",
             &[NextArgument(Argument {
                   position: ArgumentImplicitlyIs(0),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: 0,
                       precision: CountIs(10),
                       width: CountIsParam(10),
                       ty: "s",
                   },
               })]);
        same("{:.*s}",
             &[NextArgument(Argument {
                   position: ArgumentImplicitlyIs(1),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: 0,
                       precision: CountIsParam(0),
                       width: CountImplied,
                       ty: "s",
                   },
               })]);
        same("{:.10$s}",
             &[NextArgument(Argument {
                   position: ArgumentImplicitlyIs(0),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: 0,
                       precision: CountIsParam(10),
                       width: CountImplied,
                       ty: "s",
                   },
               })]);
        same("{:a$.b$s}",
             &[NextArgument(Argument {
                   position: ArgumentImplicitlyIs(0),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: 0,
                       precision: CountIsName("b"),
                       width: CountIsName("a"),
                       ty: "s",
                   },
               })]);
    }
    #[test]
    fn format_flags() {
        same("{:-}",
             &[NextArgument(Argument {
                   position: ArgumentImplicitlyIs(0),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: (1 << FlagSignMinus as u32),
                       precision: CountImplied,
                       width: CountImplied,
                       ty: "",
                   },
               })]);
        same("{:+#}",
             &[NextArgument(Argument {
                   position: ArgumentImplicitlyIs(0),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: (1 << FlagSignPlus as u32) | (1 << FlagAlternate as u32),
                       precision: CountImplied,
                       width: CountImplied,
                       ty: "",
                   },
               })]);
    }
    #[test]
    fn format_mixture() {
        same("abcd {3:a} efg",
             &[String("abcd "),
               NextArgument(Argument {
                   position: ArgumentIs(3),
                   format: FormatSpec {
                       fill: None,
                       align: AlignUnknown,
                       flags: 0,
                       precision: CountImplied,
                       width: CountImplied,
                       ty: "a",
                   },
               }),
               String(" efg")]);
    }
}
