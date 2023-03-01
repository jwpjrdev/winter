#[derive(Debug)]
pub enum Error {
    NotEnoughPrivileges,
    FailedRequest(ureq::Error),
}
