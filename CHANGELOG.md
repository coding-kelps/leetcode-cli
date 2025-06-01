# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### ⛰️  Features

- *(api)* Support package manager - ([ff7f4a7](https://github.com/coding-kelps/leetcode-cli/commit/ff7f4a7aad8c2b8dc0e6445f2c41899d9ecf37cb))
- *(api)* Snippet language support - ([36b78a0](https://github.com/coding-kelps/leetcode-cli/commit/36b78a0f14ea0adb6ac0bcd6eafb3442b6e6e207))
- *(api_runner)* Readme and fs for start cli command - ([32c02a9](https://github.com/coding-kelps/leetcode-cli/commit/32c02a934d117cfdbe467ad3b289d4b482c09269))
- *(cli)* Support for test and submit - ([7062b7b](https://github.com/coding-kelps/leetcode-cli/commit/7062b7b6c1943ed0774c52c45a201d7a1a20ffc3))
- *(config)* Supporting default and custom dir path for leetcode folder - ([8775017](https://github.com/coding-kelps/leetcode-cli/commit/8775017bdbf0cc752ed941cd032c4d58b4d29397))
- *(config.rs)* Support for default language in the config.toml - ([a5242c9](https://github.com/coding-kelps/leetcode-cli/commit/a5242c99ebeef25e7d248be5c533b1a2ac30b97e))
- *(get_problem_info)* Cli prints some info about a given problem - ([e5246f3](https://github.com/coding-kelps/leetcode-cli/commit/e5246f31baa5f1f6fb5f80088f49404ab8ef190a))
- *(leetcode_api_runner.rs)* Introducing the struct running the api with the user config - ([6bd8129](https://github.com/coding-kelps/leetcode-cli/commit/6bd8129afe299ca3f3416d7e3b23713346e0cf65))
- *(main)* Handling language input in cli - ([984aae4](https://github.com/coding-kelps/leetcode-cli/commit/984aae49fb12d61e447007579b1837d6795b19b4))
- *(main)* First use of the runner - ([373ceb2](https://github.com/coding-kelps/leetcode-cli/commit/373ceb2b71c8651a202470f0e4150ac14bd2c883))
- A long cli cmd - ([e17713c](https://github.com/coding-kelps/leetcode-cli/commit/e17713c730c9f6419fd6a70f08eaaffe2cd3aef2))
- Cli structure - ([936bc47](https://github.com/coding-kelps/leetcode-cli/commit/936bc477d0cbc3a82b65ba2cf1af3a8f812981cf))
- [**breaking**] Adding the tests to the template - ([ec8cde0](https://github.com/coding-kelps/leetcode-cli/commit/ec8cde0ef4ddb85c481566717e9841b8bb6b201f))
- [**breaking**] Handling the tests cases from md - ([187e966](https://github.com/coding-kelps/leetcode-cli/commit/187e966cf594773c7e02553734ec037af2842b79))
- [**breaking**] Major changes: - ([aae8197](https://github.com/coding-kelps/leetcode-cli/commit/aae8197f708fc16b7eb4f454efc513f7e7770f25))
- [**breaking**] Files are created correctly using the api - ([447e451](https://github.com/coding-kelps/leetcode-cli/commit/447e451e974e08053340c1803329d60fbebfea54))

### 🐛 Bug Fixes

- *(api)* Using file name instead of file content to get the file extension - ([9b21dbe](https://github.com/coding-kelps/leetcode-cli/commit/9b21dbecb43338110ef3b4aab5e55cfe3a941c23))
- *(config)* Addiing config_file values to the rcs config_file - ([5f3b3f1](https://github.com/coding-kelps/leetcode-cli/commit/5f3b3f1c20978388c50720ab15fce263f6687b47))
- *(config)* Bug and fix discovery thanks to first test draft - ([b14ea88](https://github.com/coding-kelps/leetcode-cli/commit/b14ea886430d1270d8ac3edf7f532a22adc185db))
- *(config)* Leetcode_token needs to be public in order to be used by the runner - ([3d04adc](https://github.com/coding-kelps/leetcode-cli/commit/3d04adc8272e1ef6f0c2bd0aade3f0787948687c))
- *(rustfmt)* Warning: Unknown configuration option `trailling_semicolons` - ([94aae3c](https://github.com/coding-kelps/leetcode-cli/commit/94aae3ca6f372847ff25aad72f49d5f95c1f35f1))
- Some problem using tild for home path, cleaning the code from println. - ([2d8b556](https://github.com/coding-kelps/leetcode-cli/commit/2d8b556bac2c6e0ab1d6a22f01fb477741b43f61))
- Dirs and making async chain - ([624e903](https://github.com/coding-kelps/leetcode-cli/commit/624e90380a32a8e4a6ea5de63be80a7eff986d0b))
- [**breaking**] Quick crash fix due to the extraction of the variables for the tests - ([cf67a46](https://github.com/coding-kelps/leetcode-cli/commit/cf67a46e5a2290017add9b7501dd93f94e8f1a0e))
- Removing trailing spaces & tab from template - ([ba0725e](https://github.com/coding-kelps/leetcode-cli/commit/ba0725eb6ef7671f8685e81c279f3b55255d3443))
- Renaming leetcode_problem struct - ([6c68eb5](https://github.com/coding-kelps/leetcode-cli/commit/6c68eb5348ee8410ab8cf829b8d68b49eca02a79))
- Changing author in the initial script - ([3bb7479](https://github.com/coding-kelps/leetcode-cli/commit/3bb74795d8ab60ff6ed1104e4e4dd38abcce5303))

### 🚜 Refactor

- *(*)* The config is getting it's own struct, adapting the rest of the code to that - ([e120d74](https://github.com/coding-kelps/leetcode-cli/commit/e120d74b589303f0c38732bafbfbec3c815391ae))
- *(api)* Just using mut config in api - ([3cca6d4](https://github.com/coding-kelps/leetcode-cli/commit/3cca6d4f972a071195168f5d8271a5a3e0eafd76))
- *(api)* Removing duplicate - ([aabb98b](https://github.com/coding-kelps/leetcode-cli/commit/aabb98b5e375e20dd48e1626cd30c64a658c9c6b))
- *(api)* Moving some logic to utils - ([06ef819](https://github.com/coding-kelps/leetcode-cli/commit/06ef819c20799370317771cf16b0c881732c16ed))
- *(cli)* Adding debug command so user can debug their own config - ([8be76d5](https://github.com/coding-kelps/leetcode-cli/commit/8be76d5b2f4f53844fb5d174a30703086d4a7f3d))
- *(cli)* Moving cli logic to it's own file - ([8cf105e](https://github.com/coding-kelps/leetcode-cli/commit/8cf105e121e950f8254c035ac39e8aa91b21f082))
- *(cli)* Renaming Get command in Start - ([558d9be](https://github.com/coding-kelps/leetcode-cli/commit/558d9be2fbe854cdd1a350eb19277557235adda2))
- *(config)* Removing useless legacy fn - ([1e81974](https://github.com/coding-kelps/leetcode-cli/commit/1e8197422e38cdc47dae469188a374a3d8c0e8bb))
- *(config)* Using all preset variables, removing debug - ([f9db5f6](https://github.com/coding-kelps/leetcode-cli/commit/f9db5f678e8a2f26665553368905fceb5caee9e6))
- *(lib)* Passing from only main.r to main consuming its lib - ([3554308](https://github.com/coding-kelps/leetcode-cli/commit/3554308fb80776078657328c28288e3c6e50cce0))
- *(main)* Using ? operator and printing problem creation output - ([fa28c0f](https://github.com/coding-kelps/leetcode-cli/commit/fa28c0fd2b8b60b84d8277d0b92d900dac01b82e))
- *(readme)* Shortening directory to dir - ([fd6203a](https://github.com/coding-kelps/leetcode-cli/commit/fd6203af5b2645b84a91e822387dc3ea413ca9b4))
- *(readme)* Rearranging problem output - ([4801be5](https://github.com/coding-kelps/leetcode-cli/commit/4801be547422777fbf70b3938053f799d63f591f))
- *(utils)* Return value for write to file, removing unrreachable pattern - ([1b97872](https://github.com/coding-kelps/leetcode-cli/commit/1b978721a204075f60808ac5b51254bfe534dc17))
- *(utils)* Removing wrong extensions name - ([abd81a6](https://github.com/coding-kelps/leetcode-cli/commit/abd81a60a808cbeeb4aa0336cd0a39c2a4c54a65))
- *(utils)* Moving some general logic to a new utils.rs - ([6ba31e0](https://github.com/coding-kelps/leetcode-cli/commit/6ba31e00fa806dbcdd74967b886c9af14dd6bf12))

### 📚 Documentation

- *(README.md)* Adding a step by step guide to get and use the leetcode cookie - ([215e737](https://github.com/coding-kelps/leetcode-cli/commit/215e737efc7895fc46fc3bf85da2659c2161afa3))
- *(api)* Wishing differently - ([f274b92](https://github.com/coding-kelps/leetcode-cli/commit/f274b92abeee7bdff4f715a58c2e74d122da20c4))
- *(config)* Adding comment, removing allows - ([7140f9f](https://github.com/coding-kelps/leetcode-cli/commit/7140f9fd147237c4baf55eef2984fbebcc16197e))
- *(config.sample.toml)* Adding a sample config file - ([ac6b863](https://github.com/coding-kelps/leetcode-cli/commit/ac6b863bb20cdbfbb5d9c276210a62d49b173665))
- *(readme)* Adding test / submit feature to readme - ([7ce65c8](https://github.com/coding-kelps/leetcode-cli/commit/7ce65c868a31227a90a79a69293137fce8e4c9ed))
- *(readme)* Check for any language support - ([63fc889](https://github.com/coding-kelps/leetcode-cli/commit/63fc8894aeedf611c75bc299b94f6cd667a58853))
- *(readme)* Single quote for the token, rust as default language - ([5fbca45](https://github.com/coding-kelps/leetcode-cli/commit/5fbca45541d350def29ea11a28b9c5120d949cac))
- *(readme)* Correcting the cookie guide, adding default path for leetcode config - ([8744827](https://github.com/coding-kelps/leetcode-cli/commit/8744827d326e9fc417036c1e554253687be76ee7))
- Add missing url for CI status badge - ([ee45bc6](https://github.com/coding-kelps/leetcode-cli/commit/ee45bc61a1f0835b0bbe17e846e59a7ed9fab6f6))
- Add crate metadata in cargo.toml (+standardize embryo.md) - ([246f877](https://github.com/coding-kelps/leetcode-cli/commit/246f8777606c48b44775d9d02ea5177b8b55f3c0))
- Embryon - ([cf2e9ea](https://github.com/coding-kelps/leetcode-cli/commit/cf2e9ea01bf0025d4957cbbdf48ef92c05fc7876))

### 🎨 Styling

- *(README.md)* Typo - ([e0b727e](https://github.com/coding-kelps/leetcode-cli/commit/e0b727e4579fb080164e8ab994050b68e6c37c2c))
- *(cli_tests)* Fmt - ([cb36869](https://github.com/coding-kelps/leetcode-cli/commit/cb368696b3c3d1ad65ef45ef360497cf46938f35))

### 🧪 Testing

- *(*)* Modifying tests accordingly to breaking changes - ([5a7750f](https://github.com/coding-kelps/leetcode-cli/commit/5a7750f734e65d863734d13e87bccff215b9aee0))
- *(cli-&-config-&-utils)* Basic testing for ci - ([fbbc13b](https://github.com/coding-kelps/leetcode-cli/commit/fbbc13b83d4d0a538387db46be6b7ee66f76e493))

### 🔨 Build

- *(cargo)* Adding html2md as a dep - ([944749f](https://github.com/coding-kelps/leetcode-cli/commit/944749fbe48e0d32745c595b6da1f1e2618c981e))

### ⚙️ Miscellaneous Tasks

- *(release)* Add github workflow to automatically update changelog and publish release - ([28b72e9](https://github.com/coding-kelps/leetcode-cli/commit/28b72e975ee3d57af66ccc88f163be95dbd2756a))
- *(rustfmt)* Sharing rustfmt - ([72558d1](https://github.com/coding-kelps/leetcode-cli/commit/72558d1ba18ea2856541987e42f2e0956dfdb0d1))
- Add github actions for pull requests automated checks - ([c63b434](https://github.com/coding-kelps/leetcode-cli/commit/c63b434bc3e0fda12d951e515f71a94cc6e599b0))

### ◀️ Revert

- *(changelog)* Remove changelog auto-update through GitHub Action - ([ba2eb64](https://github.com/coding-kelps/leetcode-cli/commit/ba2eb642d9bae6b7d76c5de403aed529460be4d6))

### Add

- *(dependencies)* Colored for colored output in terminal and nanohtml2text, for a quick convert of the description - ([d88779c](https://github.com/coding-kelps/leetcode-cli/commit/d88779c77fafb51311e7e4dd7bab6240b7202ae5))
- Rustfmt - ([731ca5d](https://github.com/coding-kelps/leetcode-cli/commit/731ca5d70375a9a14a7ebf99782ab68208a5c99f))
- .gitignore - ([1e9bea9](https://github.com/coding-kelps/leetcode-cli/commit/1e9bea92fe22d51244fb986c3735fd3c338b851a))
- Leetcode_init.sh - ([2a58308](https://github.com/coding-kelps/leetcode-cli/commit/2a583083a66ff3339b39d3247ed781449633214b))

### Chore

- *(readme)* Trying to provide a decent readme - ([9f3c9d5](https://github.com/coding-kelps/leetcode-cli/commit/9f3c9d555de9831e56e390cafa5351784450bca4))
- Ignoring .vscode - ([f1a62b0](https://github.com/coding-kelps/leetcode-cli/commit/f1a62b00b86f1e84e269d6f17510b6f084ee66b8))

### Del

- Old unused dependencies - ([2d15b37](https://github.com/coding-kelps/leetcode-cli/commit/2d15b37bff8357c529c52f0ae0a47ae377e6d6c1))

## New Contributors ❤️

* @guilhem-sante made their first contribution in [#6](https://github.com/coding-kelps/leetcode-cli/pull/6)
* @dfayd0 made their first contribution

<!-- generated by git-cliff -->
