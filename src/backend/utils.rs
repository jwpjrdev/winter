use sudo::{check, RunningAs};

use crate::error::Error;

pub fn ensure_running_as_root() -> Result<(), Error> {
    if check() == RunningAs::Root {
        return Ok(());
    } else {
        return Err(Error::NotEnoughPrivileges);
    }
}
