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
address = "0.0.0.0:6666"
```

### `db.toml`
```toml
[redirects]
"/testolope" = "https://example.com"
```


## TODO
 - [ ] Admin-API (always returns 401 for now)
