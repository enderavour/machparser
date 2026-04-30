
// Unifying trait for Mach-O 32 bit and 64 bit headers.
pub trait MachOHdr: Send + Sync
{
    fn from_raw(data: &[u8]) -> Box<dyn MachOHdr>
        where Self: Sized;
    fn get_magic(&self) -> [u8; 4];
    fn get_cpu_type(&self) -> u32;
    fn get_file_type(&self) -> u32;
    fn get_load_commands_number(&self) -> u32;
    fn get_load_commands_size(&self) -> u32;
    fn get_flags(&self) -> u32;
}


#[repr(packed)]
#[derive(Default)]
pub struct MachOHeader64
{
    pub magic: [u8; 4],
    pub cpu_type: u32,
    pub cpu_subtype: u32,
    pub file_type: u32,
    pub load_commands_n: u32,
    pub load_commands_size: u32,
    pub flags: u32,
    pub reserved: [u8; 4],
}

impl MachOHdr for MachOHeader64
{
    fn from_raw(data: &[u8]) -> Box<dyn MachOHdr>
        where Self: Sized
    {
        let mut hdr = MachOHeader64::default();

        let mut ptr = 0;
        let mut buf = [0u8; 4];

        // Magic number
        hdr.magic[..].copy_from_slice(&data[ptr..ptr + 4]);
        ptr += 4;

        // CPU Type
        buf[..].copy_from_slice(&data[ptr..ptr + 4]);
        hdr.cpu_type = u32::from_le_bytes(buf);
        ptr += 4;

        // CPU Subtype
        buf[..].copy_from_slice(&data[ptr..ptr + 4]);
        hdr.cpu_subtype = u32::from_le_bytes(buf);
        ptr += 4;

        // File type
        buf[..].copy_from_slice(&data[ptr..ptr + 4]);
        hdr.file_type = u32::from_le_bytes(buf);
        ptr += 4;

        // Number of Load Commands
        buf[..].copy_from_slice(&data[ptr..ptr + 4]);
        hdr.load_commands_n = u32::from_le_bytes(buf);
        ptr += 4;

        // Size of Load Commands
        buf[..].copy_from_slice(&data[ptr..ptr + 4]);
        hdr.load_commands_size = u32::from_le_bytes(buf);
        ptr += 4;

        // Flags
        buf[..].copy_from_slice(&data[ptr..ptr + 4]);
        hdr.flags = u32::from_le_bytes(buf);
        ptr += 4;

        buf[..].copy_from_slice(&data[ptr..ptr + 4]);
        hdr.reserved = buf;

        Box::new(hdr)
    }

    fn get_magic(&self) -> [u8; 4]
    {
        self.magic
    }

    fn get_cpu_type(&self) -> u32
    {
        self.cpu_type
    }

    fn get_file_type(&self) -> u32
    {
        self.file_type
    }

    fn get_load_commands_number(&self) -> u32
    {
        self.load_commands_n
    }

    fn get_load_commands_size(&self) -> u32
    {
        self.load_commands_size
    }

    fn get_flags(&self) -> u32
    {
        self.flags
    }
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
