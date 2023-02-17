[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/UrlRedirect-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/UrlRedirect-rust)


# `url-redirect`
Welcome to `url-redirect` 🎉

`url-redirect` is a trivial, headless URL redirection service.


## Quickstart
To start the server, simply run `url-redirect`. By default, `url-redirect` expects `config.toml` and `db.toml`. You can
override this behavior by exporting `CONFIG_FILE=/path/to/config.toml` respective `DB_FILE=/path/to/db.toml` into the
environment.

### `config.toml`
```toml
[server]
# The address to listen on
address = "0.0.0.0:8088"

# The connection hart limit; i.e. the amount of threads to spawn at max to process incoming connections
# This field is optional, the default value is 2048
#connection_limit = 2048
```

### `db.toml`
```toml
# The redirect database
# Please note: All redirect paths must start with a leading `/`, like in the example

[redirects]
"/testolope" = "https://example.com"
```


## TODO
 - [ ] Admin-API (always returns 403 for now)
