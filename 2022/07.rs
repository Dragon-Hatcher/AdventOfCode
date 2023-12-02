use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 07)
}

#[derive(Debug, Clone)]
enum File {
    Size(i64),
    Dir(String, Option<Vec<File>>),
}

impl File {
    fn size(&self) -> i64 {
        match self {
            File::Size(x) => *x,
            File::Dir(_, children) => children
                .as_ref()
                .map(|c| c.iter().map(File::size).sum())
                .unwrap_or_default(),
        }
    }

    fn sum_less_than(&self, max: i64) -> i64 {
        let mut size = self.size();
        if size > max {
            size = 0;
        }
        match self {
            File::Size(_) => 0,
            File::Dir(_, children) => {
                size + children
                    .as_ref()
                    .map(|c| c.iter().map(|c| c.sum_less_than(max)).sum::<i64>())
                    .unwrap_or_default()
            }
        }
    }

    fn smallest_of_size(&self, size: i64) -> Option<i64> {
        match self {
            File::Size(_) => None,
            File::Dir(_, children) => {
                let child_smallest = children
                    .as_ref()
                    .and_then(|c| c.iter().filter_map(|c| c.smallest_of_size(size)).min());

                let me = if self.size() < size {
                    None
                } else {
                    Some(self.size())
                };

                match (child_smallest, me) {
                    (None, None) => None,
                    (None, Some(a)) => Some(a),
                    (Some(a), None) => Some(a),
                    (Some(a), Some(b)) => Some(a.min(b)),
                }
            }
        }
    }

    fn children(&mut self) -> &mut Option<Vec<File>> {
        match self {
            File::Size(_) => panic!(),
            File::Dir(_, c) => c,
        }
    }

    fn add_contents(&mut self, names: Vec<String>, files: Vec<File>) {
        let mut children: &mut Option<Vec<File>> = self.children();
        for name in names {
            children = children
                .as_mut()
                .unwrap()
                .iter_mut()
                .find(|c| match c {
                    File::Size(_) => false,
                    File::Dir(n, _) => &name == n,
                })
                .unwrap()
                .children();
        }

        _ = std::mem::replace(children, Some(files))
    }
}

fn part1(input: &str) -> i64 {
    let root_file: File = File::Dir("/".to_owned(), None);
    let mut sup_root = File::Dir("".to_owned(), Some(vec![root_file]));
    let mut cur_path = vec![];
    let mut cur_files = Vec::new();

    for l in input.lines() {
        if let Some(file_name) = l.strip_prefix("$ cd ") {
            if !cur_files.is_empty() {
                sup_root.add_contents(cur_path.clone(), cur_files.clone());
            }
            if file_name == ".." {
                cur_path.pop();
            } else {
                cur_path.push(file_name.to_owned());
            }
            cur_files = Vec::new();
        } else if l.starts_with("$ ls") {
            cur_files = Vec::new();
        } else if let Some(l) = l.strip_prefix("dir ") {
            cur_files.push(File::Dir(l.to_owned(), None));
        } else {
            cur_files.push(File::Size(l.nums().nu()))
        }
    }

    sup_root.sum_less_than(100000)
}

fn part2(input: &str) -> i64 {
    let root_file: File = File::Dir("/".to_owned(), None);
    let mut sup_root = File::Dir("".to_owned(), Some(vec![root_file]));
    let mut cur_path = vec![];
    let mut cur_files = Vec::new();

    for l in input.lines() {
        if let Some(file_name) = l.strip_prefix("$ cd ") {
            if !cur_files.is_empty() {
                sup_root.add_contents(cur_path.clone(), cur_files.clone());
            }
            if file_name == ".." {
                cur_path.pop();
            } else {
                cur_path.push(file_name.to_owned());
            }
            cur_files = Vec::new();
        } else if l.starts_with("$ ls") {
            cur_files = Vec::new();
        } else if let Some(dir_name) = l.strip_prefix("dir ") {
            cur_files.push(File::Dir(dir_name.to_owned(), None));
        } else {
            cur_files.push(File::Size(l.nums().nu()))
        }
    }
    if !cur_files.is_empty() {
        sup_root.add_contents(cur_path.clone(), cur_files.clone());
    }

    let unused = 70000000 - sup_root.size();
    sup_root
        .smallest_of_size(30000000 - unused)
        .unwrap_or_default()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli();
}

#[test]
fn example() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    assert_eq!(part1(input), 95437);
    assert_eq!(part2(input), 24933642);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1543140);
    assert_eq!(part2(input), 1117448);
}
