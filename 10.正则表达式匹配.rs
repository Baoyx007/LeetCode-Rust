impl Solution {
  pub fn is_match(s: String, p: String) -> bool {
    Self::bt(&s, &p)
  }

  fn bt(rest: &str, p: &str) -> bool {
    // println!("{} | {}", rest, p);
    match (rest.is_empty(), p.is_empty()) {
      (true, true) => return true,
      // (true, false) => return false,
      (false, true) => return false,
      _ => {}
    };

    // both not empty
    let mut chars = p.chars();
    match chars.next() {
      Some('.') => match chars.next() {
        Some('*') => {
          if rest.is_empty() {
            Self::bt(rest, &p[2..])
          } else {
            Self::bt(&rest[1..], p) || Self::bt(rest, &p[2..])
          }
        }

        _ => !rest.is_empty() && Self::bt(&rest[1..], &p[1..]),
      },
      Some(c) => {
        if rest.is_empty() {
          chars.next() == Some('*') && Self::bt(rest, &p[2..])
        } else if c != rest.chars().nth(0).unwrap() {
          match chars.next() {
            Some('*') => Self::bt(rest, &p[2..]),
            _ => false,
          }
        } else {
          match chars.next() {
            Some('*') => Self::bt(&rest[1..], p) || Self::bt(rest, &p[2..]),
            _ => Self::bt(&rest[1..], &p[1..]),
          }
        }
      }
      None => false,
    }
  }
}


