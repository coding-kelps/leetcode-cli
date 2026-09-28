# Cookie (manual fallback)

To obtain your LeetCode token, follow these steps:
Login to LeetCode and obtain the csrftoken from the cookie value.

- After logging in, right-click on the page and press Inspect.
- Refresh the page.
- Look for a network request to https://leetcode.com and select it.
- Look under Request Headers for the "Cookie:" attribute.
- right-click on the cookie value and select Copy Value.
- Paste the values into the config.toml file, in the quotes of leetcode_token entry
