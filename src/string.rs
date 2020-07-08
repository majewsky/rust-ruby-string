use crate::*;
use std::iter::FromIterator;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Placement {
    //NOTE: all indices are in bytes
    pub(crate) text_start: usize,
    pub(crate) text_end: usize,
    pub(crate) ruby_start: usize,
    //TODO: this field is redundant, we could remove it and use the ruby_start field of the next
    //Placement in order instead
    pub(crate) ruby_end: usize,
}

///A string type that can have [ruby glosses](https://en.wikipedia.org/wiki/Ruby_character)
///attached to parts of it.
///
///## Memory layout
///
///The text content is stored in two String instances, one being the main text and one being a
///concatenation of all the rubies. Placement of the rubies is stored as a list of indices into
///both strings. Compared to the trivial structure (where each rubied substring is held as a
///separate String), this layout reduces memory usage and the number of separate allocations at the
///expense of slightly more complicated indexing logic.
#[derive(Clone, PartialEq, Eq)]
pub struct RubyString {
    pub(crate) packed_text: String,
    pub(crate) packed_ruby: String,
    pub(crate) placements: Vec<Placement>,
}

impl RubyString {
    ///Creates a new empty `RubyString`.
    pub fn new() -> RubyString {
        RubyString {
            packed_text: String::new(),
            packed_ruby: String::new(),
            placements: Vec::new(),
        }
    }

    ///Appends plain text (that does not have a ruby gloss attached to it) to this `RubyString`.
    pub fn push_str(&mut self, string: &str) {
        self.packed_text.push_str(string);
    }

    ///Appends text to this `RubyString` and attaches a ruby gloss to it.
    pub fn push_segment<'a>(&mut self, segment: Segment<'a>) {
        match segment {
            Segment::Plain { text } => {
                self.packed_text.push_str(text);
            }
            Segment::Rubied { text, ruby } => {
                let text_start = self.packed_text.len();
                let ruby_start = self.packed_ruby.len();
                self.packed_text.push_str(text);
                self.packed_ruby.push_str(ruby);
                self.placements.push(Placement {
                    text_start,
                    text_end: text_start + text.len(),
                    ruby_start,
                    ruby_end: ruby_start + ruby.len(),
                });
            }
        }
    }

    ///Returns the plain text stored in this `RubyString`. The result has no ruby glosses attached
    ///to it anymore.
    ///
    ///```
    ///# use ruby_string::{RubyString, Segment};
    ///let mut rs = RubyString::new();
    ///rs.push_str("ここは");
    ///rs.push_segment(Segment::Rubied { text: "東", ruby: "とう" });
    ///rs.push_segment(Segment::Rubied { text: "京", ruby: "きょう" });
    ///rs.push_str("です");
    ///assert_eq!(rs.to_plain_text(), "ここは東京です");
    ///```
    pub fn to_plain_text(&self) -> String {
        self.segments().map(|s| s.plain_text()).collect()
    }

    ///Returns an encoding of this `RubyString` as a plain String using interlinear annotation
    ///characters.
    ///
    ///```
    ///# use ruby_string::{RubyString, Segment};
    ///let mut rs = RubyString::new();
    ///rs.push_str("ここは");
    ///rs.push_segment(Segment::Rubied { text: "東", ruby: "とう" });
    ///rs.push_segment(Segment::Rubied { text: "京", ruby: "きょう" });
    ///rs.push_str("です");
    ///let encoded = rs.to_interlinear_encoding();
    ///assert_eq!(encoded, "ここは\u{FFF9}東\u{FFFA}とう\u{FFFB}\u{FFF9}京\u{FFFA}きょう\u{FFFB}です");
    ///```
    pub fn to_interlinear_encoding(&self) -> String {
        self.segments()
            .map(|s| s.to_interlinear_encoding())
            .collect()
    }

    ///An iterator over the segments in this `RubyString`.
    pub fn segments(&self) -> SegmentIterator<'_> {
        SegmentIterator {
            string: self,
            next_text_start: 0,
            next_placement_idx: 0,
        }
    }
}

impl Default for RubyString {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Into<String>> From<T> for RubyString {
    fn from(val: T) -> RubyString {
        RubyString {
            packed_text: val.into(),
            packed_ruby: String::new(),
            placements: Vec::new(),
        }
    }
}

impl<'a> FromIterator<Segment<'a>> for RubyString {
    fn from_iter<I: IntoIterator<Item = Segment<'a>>>(iter: I) -> RubyString {
        let mut s = RubyString::new();
        s.extend(iter);
        s
    }
}

impl<'a> Extend<Segment<'a>> for RubyString {
    fn extend<I: IntoIterator<Item = Segment<'a>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |s| self.push_segment(s));
    }
}
