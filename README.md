# Leetcode Command Line Interface

[![Tests Status][actions-badge]][actions-url]

[actions-url]: https://github.com/coding-kelps/leetcode-cli/actions?query=workflow%3ACI+branch%3Amain
[actions-badge]: https://github.com/coding-kelps/leetcode-cli/actions/workflows/ci.yaml/badge.svg?branch=main

Interact with LeetCode in your development environment.

## Example

```sh
leetcode-cli info --id 42
```
```leetcode problem
#42  -  Hard  -  Trapping Rain Water

Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining. 


Example 1:   
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1] 
Output: 6 
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.  

Example 2:  
Input: height = [4,2,0,3,2,5] 
Output: 9  


Constraints:  
n == height.length 
1 <= n <= 2 * 104 
0 <= height[i] <= 105 
```
---
```sh
leetcode-cli start --id 42 --lang rust
```
```rust
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_0() {
        let expected = 6;
        let result = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_1() {
        let expected = 9;
        let result = Solution::trap(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(result, expected);
    }
}

fn main() {}
```

## Features

- [ ] Automatic token generation / refresh
- [x] Search problems
- [x] Automatically create test cases based on the problem description
- [x] Download problems in any programming language
- [x] Submit solution(s) for any given problem

## Installation

run the following command to install `leetcode_cli`:

```sh
git clone https://github.com/dfayd0/leetcode-cli
cd leetcode_cli
cargo install --path .
```

## Configuration

The `leetcode_cli` tool uses a configuration file to store necessary settings.
The configuration file is located at:

```sh
~/.config/leetcode-cli/config.toml
```

and should look like this:

```toml
leetcode_token='YOUR_TOKEN_HERE' # (In single quote!) obtained from the cookie section below
default_language="Rust" # could be any language supported by leetcode, re-prompted if not found for a given problem
leetcode_dir_path="~/leetcode" # where to store the downloaded problems
```

## Cookie

To obtain your LeetCode token, follow these steps:
Login to LeetCode and obtain the csrftoken from the cookie value.

- After logging in, right-click on the page and press Inspect.
- Refresh the page.
- Look for a network request to https://leetcode.com and select it.
- Look under Request Headers for the "Cookie:" attribute.
- right-click on the cookie value and select Copy Value.
- Paste the values into the config.toml file, in the quotes of leetcode_token entry

## Usage

### Global Commands

For more details on available commands, run:

```sh
leetcode_cli --help
```

For a specific command, run:

```sh
leetcode_cli <command> --help
```

### Local Configuration

When you start a problem using `leetcode_cli start --id <problem_id>`, the tool automatically creates a `.leetcode-cli` file in the problem directory. This file contains:

```toml
problem_id = <id>
problem_name = <"problem_name">
language = <"language_chosen">
```

### Working with Problems

Once you're in a problem directory (one that contains a `.leetcode-cli` file), you can run commands without specifying the problem ID nor the language:

```sh
# Start a problem (creates the local config)
leetcode_cli start --id 42 --lang rust

# Navigate to the problem directory
cd ~/leetcode/42_trapping_rain_water

# Test your solution (automatically detects problem ID and main file)
leetcode_cli test

# Submit your solution (automatically detects problem ID and main file)
leetcode_cli submit

# You can still override the defaults if needed
leetcode_cli test --id 42 --file src/custom_solution.rs
```

#### Supported Commands

- `info --id <id>`: Get problem information (ID required)
- `start --id <id> [--lang <language>]`: Start working on a problem (creates local config)
- `test [--id <id>] [--file <path>]`: Test your solution (uses local config if available)
- `submit [--id <id>] [--file <path>]`: Submit your solution (uses local config if available)

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the tool.
