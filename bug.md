# Known bugs

Ordered from easiest to hardest fix.

- [x] `test` ignores the result of `run_local_check` (`let _ = ...`), so a failed local compilation is never shown and the code is still sent to leetcode
- [x] `get_problem_name` uses `unwrap`, so it panics instead of returning an error on a bad id or api failure
- [x] `submit` does not call `preprocess_code`, so rust submissions still contain `pub struct Solution;` and `fn main() {}`
- [x] `toml_edit` dependency is unused, `write_token_to_file` edits the config with a regex instead
- [x] `parse_code_signature` only matches `Python`, not `Python3`, so test generation (and thus `start`) fails for python3
- [x] `language_to_string` and `get_extension_from_language` panic on racket, erlang, elixir, dart, pandas and react even though `parse_programming_language` accepts them
- [x] `LocalConfig::get_main_file` matches `c++` while `language_to_string` writes `cpp`, and has no arm for csharp, ruby, swift, kotlin, scala, bash, php..., so those fall back to `main.txt` and `test` / `submit` without `-p` point to a missing file
- [x] `TestGenerator::run` returns an error for any language other than rust / python, and `start_problem` propagates it with `?`, so `start` fails for every other language
- [x] test generators index `inputs[i]` / `outputs[i]` up to `example_count`, which is counted separately from the `**Example` lines, so a readme with a missing Input / Output line panics with index out of bounds

## Rust test generation

Found by generating the file `start` writes for real leetcode rust starter codes and running `cargo test` on it.

- [x] generated tests mix tabs and spaces for indentation
- [x] `inject_default_return_value` is a no-op, the starter code keeps an empty body so nothing compiles (`expected i32, found ()`) and the tests cannot even run
- [x] `parse_rust_signature` / `parse_python_signature` slice the parameters with `end + p` instead of `end + 1 + p`: the last char of the parameters is dropped and a function without parameters (`fn new()`) panics with `byte range starts at .. but ends at ..`
- [x] `parse_rust_signature` takes the first `fn ` of the file, which is the commented `fn new` of the `ListNode` / `TreeNode` definition, so tests call `Solution::new(..)`; design problems (no `impl Solution`, eg `MinStack`) also get `Solution::..` calls
- [x] literals ignore the parameter / return types: `"1"` becomes `"1".to_string()` for a `char` (`Vec<Vec<char>>`, `&mut Vec<char>`), and an output `2` for a `f64` return stays an integer
- [x] in-place problems (`&mut` parameter, no return value, eg reverse_string) pass the value instead of `&mut` and compare `()` with the expected output instead of the mutated parameter
- [x] the readme parser splits the input on every `,` before looking for `=`, so a string containing `,` or `=` is mangled (`"t,a=n"` becomes `"t,n"`)
- [x] `ListNode` / `TreeNode` problems: the type definitions stay commented so the file does not compile, and list / tree inputs are generated as `vec![..]`
- [x] the readme parser only reads the first line of `Input:`, multi-line inputs (eg the `grid` of number of islands) become `[`
- [x] zero width spaces from the leetcode html end up in the generated literals and break the file (greatest common divisor of strings)
- [x] empty expected outputs (`vec![]`) do not type check, the `expected` binding has no type annotation (3sum, combination sum)
- [x] interactive problems (guess number) have hidden inputs, the generated calls pass more arguments than the function takes
- [x] custom judge outputs (remove duplicates / remove element: `2, nums = [1,2,_]`) are pasted as is and do not compile
- [x] float outputs are compared with `assert_eq!`, a correct `my_pow` fails on `9.261000000000001 != 9.261`, leetcode accepts a 1e-5 difference
- [x] "return the answer in any order" problems (two sum, 3sum, combination sum) fail when the order differs from the example, vectors are now sorted (inner vectors too) before being compared
- [x] starters with both a list and a tree definition (109) keep the second `Definition for` title uncommented, the file does not compile
