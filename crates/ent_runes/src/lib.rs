
//! ent_runes — движок скриптов автоматизации на Rune.
//!
//! Stateless: Vm создаётся на событие, вызывается then, уничтожается.


use rune::{Unit, Source, Sources};
use rune_macros::{FromValue, Any};
use std::path::{Path, PathBuf};

// собираем скрипты 