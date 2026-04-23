#[repr(packed)]
#[derive(Default)]
pub struct MachOHeader
{
    pub magick: [u8; 4],
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_type: u32,
    pub load_commands_n: u32,
    pub load_commands_size: u32,
    pub flags: u32,
    pub reserved: [u8; 4],
}

#[repr(packed)]
#[derive(Default, Clone, Copy)]
pub struct LoadCommand
{
    pub command_type: u32,
    pub command_size: u32,
    pub segment_name: [u8; 16],
    pub address: u64,
    pub address_size: u64,
    pub file_offset: u64,
    pub size: u64,
    pub max_mprots: u32,
    pub init_mprots: u32,
    pub sect_num: u32,
    pub flag32: u32
}
