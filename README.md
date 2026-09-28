# Leetcode Command Line Interface

[![Tests Status][actions-badge]][actions-url]

[actions-url]: https://github.com/coding-kelps/leetcode-cli/actions?query=workflow%3ACI+branch%3Amain
[actions-badge]: https://github.com/coding-kelps/leetcode-cli/actions/workflows/ci.yaml/badge.svg?branch=main

Interact with LeetCode in your development environment.

## Example

```sh
leetcode-cli info 42
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
leetcode-cli start 42 --lang rust
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

- [x] One time login that captures and saves the token automatically (see [#14](https://github.com/coding-kelps/leetcode-cli/issues/14))
- [x] Automatic token generation / refresh (see [#14](https://github.com/coding-kelps/leetcode-cli/issues/14))
- [x] Search problems
- [x] Automatically create test cases based on the problem description
- [x] Download problems in any programming language
- [x] Submit solution(s) for any given problem

## Installation

### Requirements

- a Rust toolchain (1.83 or newer), e.g. through
  [rustup](https://rustup.rs/)
- a chromium based browser (chrome, chromium, brave, edge...) for the one time
  login flow

### From the source

run the following commands to build and install `leetcode-cli`:

```sh
git clone https://github.com/coding-kelps/leetcode-cli
cd leetcode-cli
cargo install --path .
```

`cargo install` puts the binary in `~/.cargo/bin`, which is usually already in
your `PATH`.

### From the releases

prebuilt binaries for linux x64 (glibc) are attached to each
[github release](https://github.com/coding-kelps/leetcode-cli/releases), along
with a sha512 checksum:

```sh
LEETCODE_CLI_VERSION=0.1.0
RELEASE_ASSET="leetcode-cli-${LEETCODE_CLI_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
curl -LO "https://github.com/coding-kelps/leetcode-cli/releases/download/v${LEETCODE_CLI_VERSION}/${RELEASE_ASSET}" \
     -LO "https://github.com/coding-kelps/leetcode-cli/releases/download/v${LEETCODE_CLI_VERSION}/${RELEASE_ASSET}.sha512"
sha512sum -c "${RELEASE_ASSET}.sha512"
tar -xzf "${RELEASE_ASSET}"
sudo mv "leetcode-cli-${LEETCODE_CLI_VERSION}/leetcode-cli" /usr/local/bin/
```

then run `leetcode-cli login` (or any command, it walks you through the login
if no token is saved yet, see [login](#login)) and start solving problems.

## Configuration

The `leetcode-cli` tool uses a configuration file to store necessary settings.
The configuration file is located at:

```sh
~/.config/leetcode-cli/config.toml
```

and should look like this:

```toml
leetcode_token='csrftoken=YOUR_CSRFTOKEN; LEETCODE_SESSION=YOUR_SESSION' # (In single quote!) obtained automatically by `leetcode-cli login` or from the cookie section below
default_language="Rust" # could be any language supported by leetcode, re-prompted if not found for a given problem
leetcode_dir_path="~/leetcode" # where to store the downloaded problems
```

## Login

Instead of copying cookies by hand, run:

```sh
leetcode-cli login
```

- a chromium based browser opens on the leetcode login page, using a dedicated
  profile stored in `~/.config/leetcode-cli/browser-profile`
- log into your leetcode account in that browser
- the cli polls the browser devtools protocol, grabs the `csrftoken` and
  `LEETCODE_SESSION` cookies, validates them against leetcode and saves them in
  the `leetcode_token` entry of your config file
- the browser is closed automatically once the token is saved

Because the browser profile persists, the next `leetcode-cli login` run will
succeed instantly if your session is still valid, without any manual action.

If no browser is detected, or you want to force one, use:

```sh
leetcode-cli login --browser google-chrome-stable
```

or set the `LEETCODE_CLI_BROWSER` environment variable.

## Automatic Token Refresh

The `LEETCODE_SESSION` cookie lasts roughly a week before LeetCode expires it.
You don't need to track that manually: whenever a command fails because the
saved token is expired or invalid, the cli automatically opens the browser to
grab a fresh one and then re-runs the original command logic.

- if the persisted browser profile still holds a valid leetcode session, the
  refresh is instant and requires no action
- if that session also expired, you just log in once in the browser that pops
  up and the new token is saved
- if the refresh cannot complete (no browser, timeout...), the command stops
  and asks you to run `leetcode-cli login` manually

This means a missing token is handled the same way: the first `leetcode-cli`
command after installation walks you through the login instead of erroring.

## Cookie (manual fallback)

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
leetcode-cli --help
```

For a specific command, run:

```sh
leetcode-cli <command> --help
```

### Local Configuration

When you start a problem using `leetcode-cli start <problem_id>`, the tool automatically creates a `.leetcode-cli` file in the problem directory. This file contains:

```toml
problem_id = <id>
problem_name = <"problem_name">
language = <"language_chosen">
```

### Working with Problems

Once you're in a problem directory (one that contains a `.leetcode-cli` file), you can run commands without specifying the problem ID nor the language:

```sh
# Start a problem (creates the local config)
leetcode-cli start 42 --lang rust

# Navigate to the problem directory
cd ~/leetcode/42_trapping_rain_water

# Test your solution (automatically detects problem ID and main file)
leetcode-cli test

# Submit your solution (automatically detects problem ID and main file)
leetcode-cli submit

# You can still override the defaults if needed
leetcode-cli test 42 --file src/custom_solution.rs
```

#### Supported Commands

- `info <id>`: Get problem information (ID required)
- `start <id> [--lang <language>]`: Start working on a problem (creates local config)
- `test [<id>] [--file <path>]`: Test your solution (uses local config if available)
- `submit [<id>] [--file <path>]`: Submit your solution (uses local config if available)

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the tool.

## Documentation

The full documentation is built with [mdBook](https://rust-lang.github.io/mdBook/).

- read it online: https://dfayd0.github.io/leetcode-cli/
- read it locally:

```sh
cargo install mdbook
mdbook serve docs
```
