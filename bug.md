# Known bugs

Ordered from easiest to hardest fix.

- [x] `test` ignores the result of `run_local_check` (`let _ = ...`), so a failed local compilation is never shown and the code is still sent to leetcode
- [x] `get_problem_name` uses `unwrap`, so it panics instead of returning an error on a bad id or api failure
- [x] `submit` does not call `preprocess_code`, so rust submissions still contain `pub struct Solution;` and `fn main() {}`
- [x] `toml_edit` dependency is unused, `write_token_to_file` edits the config with a regex instead
- [x] `parse_code_signature` only matches `Python`, not `Python3`, so test generation (and thus `start`) fails for python3
- [x] `language_to_string` and `get_extension_from_language` panic on racket, erlang, elixir, dart, pandas and react even though `parse_programming_language` accepts them
- [ ] `LocalConfig::get_main_file` matches `c++` while `language_to_string` writes `cpp`, and has no arm for csharp, ruby, swift, kotlin, scala, bash, php..., so those fall back to `main.txt` and `test` / `submit` without `-p` point to a missing file
- [ ] `TestGenerator::run` returns an error for any language other than rust / python, and `start_problem` propagates it with `?`, so `start` fails for every other language
