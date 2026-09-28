# Installation

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
[github release](https://github.com/dfayd0/leetcode-cli/releases), along
with a sha512 checksum:

```sh
LEETCODE_CLI_VERSION=0.1.0
RELEASE_ASSET="leetcode-cli-${LEETCODE_CLI_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
curl -fLO "https://github.com/dfayd0/leetcode-cli/releases/download/v${LEETCODE_CLI_VERSION}/${RELEASE_ASSET}" \
     -fLO "https://github.com/dfayd0/leetcode-cli/releases/download/v${LEETCODE_CLI_VERSION}/${RELEASE_ASSET}.sha512"
sha512sum -c "${RELEASE_ASSET}.sha512"
tar -xzf "${RELEASE_ASSET}"
sudo mv "leetcode-cli-${LEETCODE_CLI_VERSION}/leetcode-cli" /usr/local/bin/
```

then run `leetcode-cli login` (or any command, it walks you through the login
if no token is saved yet, see [login](#login)) and start solving problems.
