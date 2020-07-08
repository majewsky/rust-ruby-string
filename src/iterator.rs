use crate::*;

///A part of a [`RubyString`](struct.RubyString.html) that either has no ruby glosses or exactly
///one ruby gloss attached to it. This type appears:
///
///- when iterating over the segments of a `RubyStrings` using its `segments` method, or
///- when `extend()`ing a `RubyString` using an `impl Iterator<Item = Segment>`,
///- when building a `RubyString` through `collect()` on an `impl Iterator<Item = Segment>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Segment<'a> {
    ///A piece of text that does not have any ruby glosses attached to it.
    Plain { text: &'a str },
    ///A piece of text that has exactly one ruby gloss attached to its entirety.
    Rubied { text: &'a str, ruby: &'a str },
}

impl<'a> Segment<'a> {
    ///Returns only the plain text in this segment, ignoring any ruby glosses attached to it.
    pub fn plain_text(&self) -> &'a str {
        match *self {
            Segment::Plain { text } => text,
            Segment::Rubied { text, .. } => text,
        }
    }

    ///Returns an encoding of this segment as a plain String using interlinear annotation
    ///characters.
    ///
    ///```
    ///# use ruby_string::Segment;
    ///let s = Segment::Plain { text: "です" };
    ///assert_eq!(s.to_interlinear_encoding(), "です");
    ///let s = Segment::Rubied { text: "東京", ruby: "とうきょう" };
    ///assert_eq!(s.to_interlinear_encoding(), "\u{FFF9}東京\u{FFFA}とうきょう\u{FFFB}");
    ///```
    pub fn to_interlinear_encoding(&self) -> String {
        match *self {
            Segment::Plain { text } => text.into(),
            Segment::Rubied { text, ruby } => format!("\u{FFF9}{}\u{FFFA}{}\u{FFFB}", text, ruby),
        }
    }
}

///A segment iterator for [`RubyString`](struct.RubyString.html).
///
///This struct is created by the `segments` method on `RubyString`. See its documentation for more.
#[derive(Clone)]
pub struct SegmentIterator<'a> {
    pub(crate) string: &'a RubyString,
    ///Start of the next segment.
    pub(crate) next_text_start: usize,
    ///The index of the placement that starts directly at `.next_text_start` or as close as
    ///possible after it.
    pub(crate) next_placement_idx: usize,
}

impl<'a> Iterator for SegmentIterator<'a> {
    type Item = Segment<'a>;

    fn next(&mut self) -> Option<Segment<'a>> {
        if self.next_text_start >= self.string.packed_text.len() {
            //nothing left at all in the RubyString
            return None;
        }
        if self.next_placement_idx >= self.string.placements.len() {
            //only unrubied text left in the RubyString
            let text_end = self.string.packed_text.len();
            let text = &self.string.packed_text[self.next_text_start..text_end];
            self.next_text_start = text_end;
            return Some(Segment::Plain { text });
        }
        let placement = self.string.placements[self.next_placement_idx];
        if self.next_text_start < placement.text_start {
            //we have not reached the next rubied part yet - yield the plain text until there
            let text = &self.string.packed_text[self.next_text_start..placement.text_start];
            self.next_text_start = placement.text_start;
            Some(Segment::Plain { text })
        } else {
            //yield a rubied part
            let text = &self.string.packed_text[placement.text_start..placement.text_end];
            let ruby = &self.string.packed_ruby[placement.ruby_start..placement.ruby_end];
            self.next_text_start = placement.text_end;
            self.next_placement_idx += 1;
            Some(Segment::Rubied { text, ruby })
        }
    }
}
