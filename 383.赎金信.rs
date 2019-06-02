
impl Solution {
  pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() > magazine.len() {
      return false;
    }
    let mut map_r: [usize; 26] = [0; 26];
    let mut map_m: [usize; 26] = [0; 26];

    ransom_note.as_bytes().iter().for_each(|&x| {
      map_r[(x - 97) as usize] += 1;
    });

    magazine.as_bytes().iter().for_each(|&x| {
      map_m[(x - 97) as usize] += 1;
    });

    println!("{:?}", map_r);
    println!("{:?}", map_m);

    map_r.iter().enumerate().all(|(idx, &v)| {
      println!("{} {}", v, map_m[idx]);
      v <= map_m[idx]
    })

  }

}