use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek};

/**
 We have to wrap the Option type in a Box because this is a recursive structure, which can theoretically
    go on infinitely, which poses a problem for Rust (it needs to know the size of values at compile time).

 The Box type allows us to work around this because a Box is a smart pointer (with a fixed value)
    to another value on the heap (values are normally put on the stack by default)

 For more info:
    https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
*/
struct TreeNode {
    val: i32,
    left: Box<Option<TreeNode>>,
    right: Box<Option<TreeNode>>,
}

fn main() -> io::Result<()> {
    let test_file = File::open("small-input.txt")?;
    let mut test_reader = BufReader::new(test_file);
    let tree = build_tree(&mut test_reader);

    println!("Hello, world!");
    Ok(())
}

// parse the input, return the root node of the tree
fn build_tree<R: Read + Seek>(reader: &mut BufReader<R>) -> TreeNode {
    let lines: Vec<Vec<String>> =
        reader
            .lines()
            .map(
                |line| line
                    .unwrap()
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect()
            )
            .collect();
    println!("{:?}", lines);

    for line in lines {
        /*
        first element can be:
            $
            dir
            number
         */

    }
    TreeNode {
        val: 0,
        left: Box::new(None),
        right: Box::new(None),
    }
}

fn get_total_sizes(tree: TreeNode) -> i32 {
    0
}
