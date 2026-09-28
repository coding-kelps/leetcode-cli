# Configuration

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
