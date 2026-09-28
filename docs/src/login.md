# Login

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
