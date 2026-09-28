# Usage

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
