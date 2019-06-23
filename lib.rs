#[macro_export]
macro_rules! vec2d {

    ($($arr:tt),*) => { // handle sets
        {
            let mut ret= Vec::new();
            $(ret.push(vec!$arr);)*
            ret
        }
    };
}

/// 为了 dfs 中,数组的 index 加上下左右
pub fn u_add_i(u: usize, i: isize, max: usize) -> Option<usize> {
  if i < 0 {
    let ret = u.checked_sub(-i as usize);
    ret.and_then(|v| if v < max { Some(v) } else { None })
  } else {
    let ret = u + i as usize;
    if ret < max {
      Some(ret)
    } else {
      None
    }
  }
}

pub fn add(u: usize, i: i32) -> usize {
  if i.is_negative() {
    u - i.wrapping_abs() as u32 as usize
  } else {
    u + i as usize
  }
}
