use thiserror::Error;
use anyhow::Context;

mod ws;

#[derive(Error, Debug)]
pub enum AppError {
    /// Represents a failure to register to IPC more one time.
    #[error("Error IPC")]
    IPCError { source: ws::IPCError },

}

fn test_main() -> anyhow::Result<(), AppError> {
    ws::test().map_err(|source| { AppError::IPCError { source }})
}

fn main() -> anyhow::Result<()>{
    let r = test_main();
    let p = 67;
    //println!("{:?}", r.err().take().unwrap());

    //let pr = r.context("Error");
    println!("{:?}", r);

    //pr
    Ok(())
}
