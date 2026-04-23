use std::io::Error;
use std::{fs::File, io::Read};

use crate::macho::{LoadCommand, MachOHeader};

pub fn read_entire_file(fh: &mut File) -> Result<Vec<u8>, Error>
{
    let flen = fh.metadata()?.len();
    let mut buffer = Vec::with_capacity(flen as usize);
    unsafe {
        buffer.set_len(flen as usize);
    }

    fh.read_exact(&mut buffer)?;

    Ok(buffer)
}

pub fn parse_mach_header(fdata: &[u8]) -> MachOHeader
{
    let mut ptr = 0;
    let mut buf = [0u8; 4];

    let mut hdr = MachOHeader::default();

    // Magick number
    hdr.magick[..].copy_from_slice(&fdata[ptr..ptr + 4]);
    ptr += 4;

    // CPU Type
    buf[..].copy_from_slice(&fdata[ptr..ptr + 4]);
    hdr.cpu_type = u32::from_le_bytes(buf);
    ptr += 4;

    // CPU Subtype
    buf[..].copy_from_slice(&fdata[ptr..ptr + 4]);
    hdr.cpu_subtype = u32::from_le_bytes(buf);
    ptr += 4;

    // File type
    buf[..].copy_from_slice(&fdata[ptr..ptr + 4]);
    hdr.file_type = u32::from_le_bytes(buf);
    ptr += 4;

    // Number of Load Commands
    buf[..].copy_from_slice(&fdata[ptr..ptr + 4]);
    hdr.load_commands_n = u32::from_le_bytes(buf);
    ptr += 4;

    // Size of Load Commands
    buf[..].copy_from_slice(&fdata[ptr..ptr + 4]);
    hdr.load_commands_size = u32::from_le_bytes(buf);
    ptr += 4;

    // Flags
    buf[..].copy_from_slice(&fdata[ptr..ptr + 4]);
    hdr.flags = u32::from_le_bytes(buf);
    ptr += 4;

    buf[..].copy_from_slice(&fdata[ptr..ptr + 4]);
    hdr.reserved = buf;

    hdr
}

#[macro_export]
macro_rules! get_field {
    ($s:ident, $fld:ident) => {{
        let ftype_unal = &raw const $s.$fld;
        let ftype = unsafe { ptr::read_unaligned(ftype_unal) };
        ftype
    }};
}

pub fn read_load_commands(data: &[u8], commands_count: u32) -> Vec<LoadCommand>
{
    let commands_offset = &data[32..];
    let mut ptr = 0;
    let mut load_command = LoadCommand::default();
    let mut vec_commands = Vec::<LoadCommand>::new();
    let mut u32buf = [0u8; 4];
    let mut u64buf = [0u8; 8];

    for _ in 0..commands_count
    {
        // Command type
        u32buf[..].copy_from_slice(&commands_offset[ptr..ptr + 4]);
        load_command.command_type = u32::from_le_bytes(u32buf);
        ptr += 4;

        // Command Size
        u32buf[..].copy_from_slice(&commands_offset[ptr..ptr + 4]);
        load_command.command_size = u32::from_le_bytes(u32buf);
        ptr += 4;

        // Load Command
        load_command.segment_name[..].copy_from_slice(&commands_offset[ptr..ptr + 16]);
        ptr += 16;

        // Address
        u64buf[..].copy_from_slice(&commands_offset[ptr..ptr + 8]);
        load_command.address = u64::from_le_bytes(u64buf);
        ptr += 8;

        // Address size
        u64buf[..].copy_from_slice(&commands_offset[ptr..ptr + 8]);
        load_command.address_size = u64::from_le_bytes(u64buf);
        ptr += 8;

        // File offset
        u64buf[..].copy_from_slice(&commands_offset[ptr..ptr + 8]);
        load_command.file_offset = u64::from_le_bytes(u64buf);
        ptr += 8;

        // Size (bytes from file offset)
        u64buf[..].copy_from_slice(&commands_offset[ptr..ptr + 8]);
        load_command.size = u64::from_le_bytes(u64buf);
        ptr += 8;

        // Maximum virtual memory protections
        u32buf[..].copy_from_slice(&commands_offset[ptr..ptr + 4]);
        load_command.max_mprots = u32::from_le_bytes(u32buf);
        ptr += 4;

        // Initial virtual memory protections
        u32buf[..].copy_from_slice(&commands_offset[ptr..ptr + 4]);
        load_command.init_mprots = u32::from_le_bytes(u32buf);
        ptr += 4;

        // Number of sections
        u32buf[..].copy_from_slice(&commands_offset[ptr..ptr + 4]);
        load_command.sect_num = u32::from_le_bytes(u32buf);
        ptr += 4;

        // Flag32
        u32buf[..].copy_from_slice(&commands_offset[ptr..ptr + 4]);
        load_command.sect_num = u32::from_le_bytes(u32buf);
        ptr += 4;

        let lc_copy = load_command.clone();
        vec_commands.push(lc_copy);
    }

    vec_commands
}
