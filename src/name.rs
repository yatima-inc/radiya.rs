use num_bigint::BigUint;
use sp_im::Vector;
use sp_std::{borrow::Borrow, fmt, ops::Deref, rc::Rc, vec::Vec};

use alloc::string::{String, ToString};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum NamePart {
  Str(String),
  Num(BigUint),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Name {
  pub system: bool,
  pub parts: Vector<NamePart>,
}

impl Name {
  pub fn empty() -> Self {
    Name {
      system: false,
      parts: Vector::new(),
    }
  }
  /// Simple shorthand for creating a Name
  pub fn simple(s: &[&str]) -> Self {
    Name {
      system: false,
      parts: s.iter().map(|s| NamePart::Str(s.to_string())).collect(),
    }
  }

  pub fn print(&self) -> String {
    let mut res = String::new();
    let mut iter = self.parts.iter().peekable();
    if let Some(_) = iter.peek() {
      res.push('.');
    }
    while let Some(p) = iter.next() {
      if let Some(_) = iter.peek() {
        res.push('.');
      }
      match p {
        NamePart::Str(s) => res.push_str(s),
        NamePart::Num(n) => res.push_str(&format!("{}", n)),
      }
    }
    res
  }
}

impl fmt::Debug for Name {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.print())
  }
}

impl fmt::Display for Name {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.print())
  }
}

/// Generates unique names
pub struct NameGenerator {
  prefix: Name,
  next_index: u32,
}

impl NameGenerator {
  pub fn new(prefix: Name) -> Self {
    NameGenerator {
      prefix,
      next_index: 0
    }
  }
}

// impl AsRef<str> for Name {
//  fn as_ref(&self) -> &str { self.inner.as_ref() }
//}
// impl<'a> From<&'a str> for Name {
//  fn from(v: &str) -> Name { Self { inner: Rc::from(v) } }
//}
// impl From<String> for Name {
//  fn from(v: String) -> Name { Self { inner: Rc::from(v) } }
//}

// impl Borrow<str> for Name {
//  fn borrow(&self) -> &str { self.inner.borrow() }
//}
// impl Deref for Name {
//  type Target = str;
//
//  fn deref(&self) -> &str { self.inner.deref() }
//}
