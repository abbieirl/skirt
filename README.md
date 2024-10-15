# Skirt
<picture><img align="right" width="25%" src="./.github/assets/margot.png" /></picture>

**Synchronization primitives that rely on spin-locking mechanisms.**

> [!WARNING]\
> This project is currently under development, and things might change rapidly.

The behavior of these locks is similar to their counterparts in [`std::sync`][STD_SYNC], but with a few key differences.
- Locks are not poisoned if a thread panics while holding them.
- Threads encountering an unavailable lock will busy-wait in a loop until it's available, without yielding.

## Usage
```console
cargo add skirt
```

| Primitive                                  | Description                                                     |
|:-------------------------------------------|:----------------------------------------------------------------|
| [`skirt::sync::Once`][SKIRT_SYNC_ONCE]     | A primitive that provides lazy one-time initialization.         |
| [`skirt::sync::Mutex`][SKIRT_SYNC_MUTEX]   | A mutual exclusion primitive useful for protecting shared data. |
| [`skirt::sync::RwLock`][SKIRT_SYNC_RWLOCK] | A reader write lock.                                            |

## Contributing
> [!NOTE]\
> All contributions to this project must comply with the Rust standard licensing model `MIT OR Apache 2.0` and will be dual-licensed accordingly, without additional terms or conditions.

## License
This project is dual licensed and distributed under the terms of `MIT OR Apache 2.0`.
- `Apache 2.0` ─ [`LICENSE-APACHE`][LICENSE_APACHE] ─  https://www.apache.org/licenses/LICENSE-2.0
- `MIT` ─ [`LICENSE-MIT`][LICENSE_MIT] ─ https://opensource.org/license/mit

[STD_SYNC]: https://doc.rust-lang.org/std/sync/
[SKIRT_SYNC_ONCE]: #
[SKIRT_SYNC_MUTEX]: #
[SKIRT_SYNC_RWLOCK]: #
[LICENSE_APACHE]: ./LICENSE-APACHE
[LICENSE_MIT]: ./LICENSE-MIT
