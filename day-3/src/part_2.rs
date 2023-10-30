use std::fs;
use std::time::Instant;

pub fn part_2() {
  let file_path = r"src/input.txt";

  let input = fs::read_to_string(file_path).unwrap().lines().map(String::from).collect::<Vec<String>>();

  let start = Instant::now();

  let mut badges = Vec::new();
  let mut x = 0;
  while &x < &input.len() {
    let (elf_1, elf_2, elf_3) = (&input[x], &input[x+1], &input[x+2]);
    let char = compare(&elf_1, &elf_2, &elf_3);
    let c = match char {
      Some(x) => x,
      None => '-',
    };
    badges.push(c.value());
    x += 3;
  }

  let duration = start.elapsed();

  println!("Total: {:?} in {:?}", badges.into_iter().sum::<u32>(), duration);
}

fn compare(str_1: &str, str_2: &str, str_3: &str) -> Option<char> {
  for c in str_1.chars() {
    for d in str_2.chars() {
      for e in str_3.chars() {
        if c == d && d == e {
          return Some(c);
        }
      }
    }
  }
  None
}


trait Priority {
    fn value(self) -> u32;
}

impl Priority for char {
    fn value(self) -> u32 {
        match self {
            'a'..='z' => self as u32 - 'a' as u32 + 1,
            'A'..='Z' => self as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid Character"),
        }
    }
}
