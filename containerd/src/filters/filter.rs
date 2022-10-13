use log::error;

use super::adaptor::Adaptor;
use std::ops::{Deref, DerefMut};

// Package filters defines a syntax and parser that can be used for the
// filtration of items across the containerd API. The core is built on the
// concept of protobuf field paths, with quoting.  Several operators allow the
// user to flexibly select items based on field presence, equality, inequality
// and regular expressions. Flexible adaptors support working with any type.
//
// The syntax is fairly familiar, if you've used container ecosystem
// projects.  At the core, we base it on the concept of protobuf field
// paths, augmenting with the ability to quote portions of the field path
// to match arbitrary labels. These "selectors" come in the following
// syntax:
//
// ```
// <fieldpath>[<operator><value>]
// ```
//
// A basic example is as follows:
//
// ```
// name==foo
// ```
//
// This would match all objects that have a field `name` with the value
// `foo`. If we only want to test if the field is present, we can omit the
// operator. This is most useful for matching labels in containerd. The
// following will match objects that have the field "labels" and have the
// label "foo" defined:
//
// ```
// labels.foo
// ```
//
// We also allow for quoting of parts of the field path to allow matching
// of arbitrary items:
//
// ```
// labels."very complex label"==something
// ```
//
// We also define `!=` and `~=` as operators. The `!=` will match all
// objects that don't match the value for a field and `~=` will compile the
// target value as a regular expression and match the field value against that.
//
// Selectors can be combined using a comma, such that the resulting
// selector will require all selectors are matched for the object to match.
// The following example will match objects that are named `foo` and have
// the label `bar`:
//
// ```
// name==foo,labels.bar
// ```

/// Filter matches specific resources based the provided filter
pub trait Filter {
    fn is_match(&self, adaptor: &Box<dyn Adaptor>) -> bool;
}

/// AdapterFunc allows implementation specific matching of fieldpaths
pub type FilterFunc = fn(adaptor: &Box<dyn Adaptor>) -> bool;

impl Filter for FilterFunc {
    fn is_match(&self, adaptor: &Box<dyn Adaptor>) -> bool {
        return self(adaptor);
    }
}

fn always_fn(adaptor: &Box<dyn Adaptor>) -> bool {
    true
}

/// ALWAYS is a filter that always returns true for any type of object
const ALWAYS: FilterFunc = always_fn;

/// Any allows multiple filters to be matched against the object
pub struct Any(Vec<Box<dyn Filter>>);

impl Filter for Any {
    /// is_match returns true if any of the provided filters are true
    fn is_match(&self, adaptor: &Box<dyn Adaptor>) -> bool {
        for filter in self.iter() {
            if filter.is_match(adaptor) {
                return true;
            }
        }
        return false;
    }
}

impl Deref for Any {
    type Target = Vec<Box<dyn Filter>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Any {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// All allows multiple filters to be matched against the object
pub struct All(Vec<Box<dyn Filter>>);

impl Filter for All {
    /// is_match only returns true if all filters match the object
    fn is_match(&self, adaptor: &Box<dyn Adaptor>) -> bool {
        for filter in self.iter() {
            if !filter.is_match(adaptor) {
                return false;
            }
        }
        return false;
    }
}

impl Deref for All {
    type Target = Vec<Box<dyn Filter>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for All {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum Operator {
    Present,
    Equal,
    NotEqual,
    Matches,
}

impl Operator {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Operator::Present => return "?",
            Operator::Equal => return "==",
            Operator::NotEqual => return "!=",
            Operator::Matches => return "~=",
        }
    }
}

pub struct Select {
    field_path: Vec<String>,
    operator: Operator,
    value: String,
    re: Option<regex::Regex>,
}

impl Select {
    fn is_match(mut self, adaptor: Box<dyn Adaptor>) -> bool {
        let (value, present) = adaptor.field(self.field_path);
        match self.operator {
            Operator::Present => return present,
            Operator::Equal => return present && value == self.value,
            Operator::NotEqual => return value != self.value,
            Operator::Matches => {
                let r = match self.re {
                    Some(r) => r,
                    None => match regex::Regex::new(self.value.as_str()) {
                        Ok(a) => a,
                        Err(_e) => {
                            error!("error compiling regex {}", self.value);
                            return false;
                        }
                    },
                };
                self.re = Some(r.clone());
                return r.is_match(self.value.as_str());
            }
        };
    }
}
