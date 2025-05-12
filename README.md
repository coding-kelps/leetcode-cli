# Leetcode Command Line Interface

Interact with LeetCode in your development environment.

## Example

```sh
leetcode_cli info --id 32
```

gives:

```
32 Hard: Longest Valid Parentheses

Given a string containing just the characters '(' and ')',
return the length of the longest valid (well-formed) parentheses substring.


Example 1:
Input: s = "(()" Output: 2 Explanation: The longest valid parentheses substring is "()".

Example 2:
Input: s = ")()())" Output: 4 Explanation: The longest valid parentheses substring is "()()".

Example 3:
Input: s = "" Output: 0

Constraints:
0 <= s.length <= 3 * 104
s[i] is '(', or ')'.
```

## Features

- [ ] Automatic token generation / refresh
- [x] Search problems
- [ ] Automatically create test cases based on the problem description
- [ ] Download problems in any programming language
- [ ] Submit solution(s) for any given problem

## Installation

run the following command to install `leetcode_cli`:

```sh
git clone https://github.com/dfayd0/leetcode-cli
cd leetcode_cli
cargo install --path .
```

## Configuration

The `leetcode_cli` tool uses a configuration file to store necessary settings, such as your LeetCode token or your desired language.  
The configuration file is located at:

```rust
~/.config/leetcode-cli/config.toml
```

## Cookie

To obtain your LeetCode token, follow these steps:  
Login to LeetCode and obtain the csrftoken and LEETCODE_SESSION cookie values.

- After logging in, right-click on the page and press Inspect.
- Refresh the page.
- Look for a network request to https://leetcode.com and select it.
- Look under Request Headers for the cookie: attribute to find the values.
- right-click on the cookie value and select Copy Value.
- Paste the values into the config.toml file, in the quotes of leetcode_token entry

## Usage

For more details on available commands, run:

```sh
leetcode_cli --help
```

For a specific command, run:

```sh
leetcode_cli <command> --help
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the tool.
