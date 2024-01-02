#!/bin/sh

# This script is used to initialize a leetcode problem in Rust
# Usage: ./leetcode_init.sh <problem_name>

# Check if the problem name is given
# If not, exit
if [ $# -eq 0 ]; then
    echo "Usage: ./leetcode_init.sh <problem_name>"
    exit 1
fi

# Create a directory for the problem
# If the directory already exists, exit
if [ -d "$1" ]; then
    echo "Directory $1 already exists"
    exit 1
else
    mkdir "$1"
fi

# create a problem.md file
touch "$1/problem.md"

# create a src directory with a main.rs file
mkdir "$1/src"
touch "$1/src/main.rs"

content_cargo="[package]\nname = \"$1\"\nversion = \"0.1.0\"\nauthors = [\"Coton\"]\nedition = \"2018\"\n[dependencies]\n"

echo -e "$content_cargo" > "$1/Cargo.toml"

content_main=$(cat <<EOF
pub struct Solution {}

impl Solution {
    pub fn $1() -> i32 {
        // TODO: Implement the function
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn $1_test1() {
	let x = 0; //TODO
        assert_eq!(Solution::$1(x), 0);
    }

    #[test]
    fn $1_test2() {
	let x = 0; //TODO
        assert_eq!(Solution::$1(x), 0);
    }

    #[test]
    fn $1_test3() {
	let x = 0; //TODO
        assert_eq!(Solution::$1(x), 0);
    }
}

fn main() {
}
EOF
)

echo -e "$content_main" > "$1/src/main.rs"

echo "Successfully initialized leetcode problem $1"
