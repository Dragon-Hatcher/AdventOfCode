use crate::{helpers::IterExtension, standard_parsers::AocParsed};

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

///
/// --- Day 7: No Space Left On Device ---
///
/// You can hear birds chirping and raindrops hitting leaves as the expedition proceeds.
/// Occasionally, you can even hear much louder sounds in the distance; how big do
/// the animals get out here, anyway?
///
/// The device the Elves gave you has problems with more than just its communication
/// system. You try to run a system update:
///
/// ```
/// $ system-update --please --pretty-please-with-sugar-on-top
/// Error: No space left on device
///
/// ```
///
/// Perhaps you can delete some files to make space for the update?
///
/// You browse around the filesystem to assess the situation and save the resulting
/// terminal output (your puzzle input). For example:
///
/// ```
/// $ cd /
/// $ ls
/// dir a
/// 14848514 b.txt
/// 8504156 c.dat
/// dir d
/// $ cd a
/// $ ls
/// dir e
/// 29116 f
/// 2557 g
/// 62596 h.lst
/// $ cd e
/// $ ls
/// 584 i
/// $ cd ..
/// $ cd ..
/// $ cd d
/// $ ls
/// 4060174 j
/// 8033020 d.log
/// 5626152 d.ext
/// 7214296 k
///
/// ```
///
/// The filesystem consists of a tree of files (plain data) and directories (which
/// can contain other directories or files). The outermost directory is called `/`.
/// You can navigate around the filesystem, moving into or out of directories and
/// listing the contents of the directory you're currently in.
///
/// Within the terminal output, lines that begin with `$` are *commands you executed*,
/// very much like some modern computers:
///
/// * `cd` means *change directory*. This changes which directory is the current
/// directory, but the specific result depends on the argument:
///   * `cd x` moves *in* one level: it looks in the current directory for the directory
/// named `x` and makes it the current directory.
///   * `cd ..` moves *out* one level: it finds the directory that contains the current
/// directory, then makes that directory the current directory.
///   * `cd /` switches the current directory to the outermost directory, `/`.
///
/// * `ls` means *list*. It prints out all of the files and directories immediately
/// contained by the current directory:
///   * `123 abc` means that the current directory contains a file named `abc` with
/// size `123`.
///   * `dir xyz` means that the current directory contains a directory named `xyz`.
///
/// Given the commands and output in the example above, you can determine that the
/// filesystem looks visually like this:
///
/// ```
/// - / (dir)
///   - a (dir)
///     - e (dir)
///       - i (file, size=584)
///     - f (file, size=29116)
///     - g (file, size=2557)
///     - h.lst (file, size=62596)
///   - b.txt (file, size=14848514)
///   - c.dat (file, size=8504156)
///   - d (dir)
///     - j (file, size=4060174)
///     - d.log (file, size=8033020)
///     - d.ext (file, size=5626152)
///     - k (file, size=7214296)
///
/// ```
///
/// Here, there are four directories: `/` (the outermost directory), `a` and `d`
/// (which are in `/`), and `e` (which is in `a`). These directories also contain
/// files of various sizes.
///
/// Since the disk is full, your first step should probably be to find directories
/// that are good candidates for deletion. To do this, you need to determine the
/// *total size* of each directory. The total size of a directory is the sum of the
/// sizes of the files it contains, directly or indirectly. (Directories themselves
/// do not count as having any intrinsic size.)
///
/// The total sizes of the directories above can be found as follows:
///
/// * The total size of directory `e` is *584* because it contains a single file
/// `i` of size 584 and no other directories.
/// * The directory `a` has total size *94853* because it contains files `f` (size
/// 29116), `g` (size 2557), and `h.lst` (size 62596), plus file `i` indirectly (`a`
/// contains `e` which contains `i`).
/// * Directory `d` has total size *24933642*.
/// * As the outermost directory, `/` contains every file. Its total size is *48381165*,
/// the sum of the size of every file.
///
/// To begin, find all of the directories with a total size of *at most 100000*,
/// then calculate the sum of their total sizes. In the example above, these directories
/// are `a` and `e`; the sum of their total sizes is `*95437*` (94853 + 584). (As
/// in this example, this process can count files more than once!)
///
/// Find all of the directories with a total size of at most 100000. *What is the
/// sum of the total sizes of those directories?*
///
pub fn part1(input: &str) -> i64 {
    let root_file: File = File::Dir("/".to_owned(), None);
    let mut sup_root = File::Dir("".to_owned(), Some(vec![root_file]));
    let mut cur_path = vec![];
    let mut cur_files = Vec::new();

    for l in input.non_empty() {
        if l.starts_with("$ cd ") {
            if !cur_files.is_empty() {
                sup_root.add_contents(cur_path.clone(), cur_files.clone());
            }
            let file_name = &l[5..];
            if file_name == ".." {
                cur_path.pop();
            } else {
                cur_path.push(file_name.to_owned());
            }
            cur_files = Vec::new();
        } else if l.starts_with("$ ls") {
            cur_files = Vec::new();
        } else if l.starts_with("dir ") {
            cur_files.push(File::Dir(l[4..].to_owned(), None));
        } else {
            cur_files.push(File::Size(l.nums().nu()))
        }
    }

    sup_root.sum_less_than(100000)
}

///
/// --- Part Two ---
///
/// Now, you're ready to choose a directory to delete.
///
/// The total disk space available to the filesystem is `*70000000*`. To run the
/// update, you need unused space of at least `*30000000*`. You need to find a directory
/// you can delete that will *free up enough space* to run the update.
///
/// In the example above, the total size of the outermost directory (and thus the
/// total amount of used space) is `48381165`; this means that the size of the *unused*
/// space must currently be `21618835`, which isn't quite the `30000000` required
/// by the update. Therefore, the update still requires a directory with total size
/// of at least `8381165` to be deleted before it can run.
///
/// To achieve this, you have the following options:
///
/// * Delete directory `e`, which would increase unused space by `584`.
/// * Delete directory `a`, which would increase unused space by `94853`.
/// * Delete directory `d`, which would increase unused space by `24933642`.
/// * Delete directory `/`, which would increase unused space by `48381165`.
///
/// Directories `e` and `a` are both too small; deleting them would not free up enough
/// space. However, directories `d` and `/` are both big enough! Between these, choose
/// the *smallest*: `d`, increasing unused space by `*24933642*`.
///
/// Find the smallest directory that, if deleted, would free up enough space on the
/// filesystem to run the update. *What is the total size of that directory?*
///
pub fn part2(input: &str) -> i64 {
    let root_file: File = File::Dir("/".to_owned(), None);
    let mut sup_root = File::Dir("".to_owned(), Some(vec![root_file]));
    let mut cur_path = vec![];
    let mut cur_files = Vec::new();

    for l in input.non_empty() {
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

const PART1_EX_ANSWER: &str = "95437";
const PART1_ANSWER: &str = "1543140";
const PART2_EX_ANSWER: &str = "24933642";
const PART2_ANSWER: &str = "1117448";
pub const ANSWERS: (&str, &str, &str, &str) =
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);
