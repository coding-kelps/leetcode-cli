# Introduction

`leetcode-cli` is a Rust command-line interface to interact with
[LeetCode](https://leetcode.com) directly from your development environment:
browse problems, start them locally in any language, generate the test cases
for you, then test and submit your solutions without leaving the terminal.

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
