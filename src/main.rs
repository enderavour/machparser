mod macho;
mod parser;
use std::fs::File;
use std::error::Error;
use std::ptr;
use std::env;
use macho::MachOHdr;
use std::sync::{Once, OnceLock};

static MACHO_HDR: OnceLock<Box<dyn MachOHdr>> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>>
{
    let fname = env::args().nth(1).unwrap();
    let mut f = File::open(fname)?;
    let f_contents = parser::read_entire_file(&mut f)?;
    #[cfg(target_arch = "x86_64")]
    {
        use crate::macho::MachOHeader64;
        // Safety Kaboom💥💥💥
        MACHO_HDR.set(MachOHeader64::from_raw(&f_contents));
        println!("Executable type: {}", MACHO_HDR.get().unwrap().get_file_type());
        let load_comms = parser::read_load_commands(&f_contents, MACHO_HDR.get().unwrap().get_load_commands_number());
        println!("First segment name: {}", String::from_utf8_lossy(&load_comms[0].segment_name));
    }
    Ok(())
}
