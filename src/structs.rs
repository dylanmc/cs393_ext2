use bitflags::bitflags;
use null_terminated::NulStr;

#[repr(C)]
#[derive(Debug)]
// https://wiki.osdev.org/Ext2
pub struct Superblock {
    // taken from https://wiki.osdev.org/Ext2
    /// Total number of inodes in file system
    pub inodes_count: u32,
    /// Total number of blocks in file system
    pub blocks_count: u32,
    /// Number of blocks reserved for superuser (see offset 80)
    pub r_blocks_count: u32,
    /// Total number of unallocated blocks
    pub free_blocks_count: u32,
    /// Total number of unallocated inodes
    pub free_inodes_count: u32,
    /// Block number of the block containing the superblock
    pub first_data_block: u32,
    /// log2 (block size) - 10. (In other words, the number to shift 1,024
    /// to the left by to obtain the block size)
    pub log_block_size: u32,
    /// log2 (fragment size) - 10. (In other words, the number to shift
    /// 1,024 to the left by to obtain the fragment size)
    pub log_frag_size: i32,
    /// Number of blocks in each block group
    pub blocks_per_group: u32,
    /// Number of fragments in each block group
    pub frags_per_group: u32,
    /// Number of inodes in each block group
    pub inodes_per_group: u32,
    /// Last mount time (in POSIX time)
    pub mtime: u32,
    /// Last written time (in POSIX time)
    pub wtime: u32,
    /// Number of times the volume has been mounted since its last
    /// consistency check (fsck)
    pub mnt_count: u16,
    /// Number of mounts allowed before a consistency check (fsck) must be
    /// done
    pub max_mnt_count: i16,
    /// Ext2 signature (0xef53), used to help confirm the presence of Ext2
    /// on a volume
    pub magic: u16,
    /// File system state (see `FS_CLEAN` and `FS_ERR`)
    pub state: u16,
    /// What to do when an error is detected (see `ERR_IGNORE`, `ERR_RONLY` and
    /// `ERR_PANIC`)
    pub errors: u16,
    /// Minor portion of version (combine with Major portion below to
    /// construct full version field)
    pub rev_minor: u16,
    /// POSIX time of last consistency check (fsck)
    pub lastcheck: u32,
    /// Interval (in POSIX time) between forced consistency checks (fsck)
    pub checkinterval: u32,
    /// Operating system ID from which the filesystem on this volume was
    /// created
    pub creator_os: u32,
    /// Major portion of version (combine with Minor portion above to
    /// construct full version field)
    pub rev_major: u32,
    /// User ID that can use reserved blocks
    pub block_uid: u16,
    /// Group ID that can use reserved blocks
    pub block_gid: u16,

    /// First non-reserved inode in file system.
    pub first_inode: u32,
    /// Size of each inode structure in bytes. - only 128 bytes seem used
    /// but modern EXT filesystems seem to use 256 bytes for each inode
    pub inode_size: u16,
    /// Block group that this superblock is part of (if backup copy)
    pub block_group: u16,
    /// Optional features present (features that are not required to read
    /// or write, but usually result in a performance increase)
    pub features_opt: u32,
    /// Required features present (features that are required to be
    /// supported to read or write)
    pub features_req: u32,
    /// Features that if not supported, the volume must be mounted
    /// read-only)
    pub features_ronly: u32,
    /// File system ID (what is output by blkid)
    pub fs_id: [u8; 16],
    /// Volume name (C-style string: characters terminated by a 0 byte)
    pub volume_name: [u8; 16],
    /// Path volume was last mounted to (C-style string: characters
    /// terminated by a 0 byte)
    pub last_mnt_path: [u8; 64],
    /// Compression algorithms used (see Required features above)
    pub compression: u32,
    /// Number of blocks to preallocate for files
    pub prealloc_blocks_files: u8,
    /// Number of blocks to preallocate for directories
    pub prealloc_blocks_dirs: u8,
    #[doc(hidden)]
    _unused: [u8; 2],
    /// Journal ID (same style as the File system ID above)
    pub journal_id: [u8; 16],
    /// Journal inode
    pub journal_inode: u32,
    /// Journal device
    pub journal_dev: u32,
    /// Head of orphan inode list
    pub journal_orphan_head: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BlockGroupDescriptor {
    /// Block address of block usage bitmap
    pub block_usage_addr: u32,
    /// Block address of inode usage bitmap
    pub inode_usage_addr: u32,
    /// Starting block address of inode table
    pub inode_table_block: u32,
    /// Number of unallocated blocks in group
    pub free_blocks_count: u16,
    /// Number of unallocated inodes in group
    pub free_inodes_count: u16,
    /// Number of directories in group
    pub dirs_count: u16,

    _reserved: [u8; 14],
}

#[repr(C)]
pub struct Inode {
    /// Type and Permissions (see below)
    pub type_perm: TypePerm,
    /// User ID
    pub uid: u16,
    /// Lower 32 bits of size in bytes
    pub size_low: u32,
    /// Last Access Time (in POSIX time)
    pub atime: u32,
    /// Creation Time (in POSIX time)
    pub ctime: u32,
    /// Last Modification time (in POSIX time)
    pub mtime: u32,
    /// Deletion time (in POSIX time)
    pub dtime: u32,
    /// Group ID
    pub gid: u16,
    /// Count of hard links (directory entries) to this inode. When this
    /// reaches 0, the data blocks are marked as unallocated.
    pub hard_links: u16,
    /// Count of disk sectors (not Ext2 blocks) in use by this inode, not
    /// counting the actual inode structure nor directory entries linking
    /// to the inode.
    pub sectors_count: u32,
    /// Flags
    pub flags: u32,
    /// Operating System Specific value #1
    pub _os_specific_1: [u8; 4],
    /// Direct block pointers
    pub direct_pointer: [u32; 12],
    /// Singly Indirect Block Pointer (Points to a block that is a list of
    /// block pointers to data)
    pub indirect_pointer: u32,
    /// Doubly Indirect Block Pointer (Points to a block that is a list of
    /// block pointers to Singly Indirect Blocks)
    pub doubly_indirect: u32,
    /// Triply Indirect Block Pointer (Points to a block that is a list of
    /// block pointers to Doubly Indirect Blocks)
    pub triply_indirect: u32,
    /// Generation number (Primarily used for NFS)
    pub gen_number: u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1,
    /// Extended attribute block (File ACL).
    pub ext_attribute_block: u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1, Upper
    /// 32 bits of file size (if feature bit set) if it's a file,
    /// Directory ACL if it's a directory
    pub size_high: u32,
    /// Block address of fragment
    pub frag_block_addr: u32,
    /// Operating System Specific Value #2
    pub _os_specific_2: [u8; 12],
    _padding: [u8; 128], // TODO: handle inode sizes != 128 according to superblock
}

#[repr(C)]
#[derive(Debug)]
pub struct DirectoryEntry {
    /// Inode
    pub inode: u32,
    /// Total size of this entry (Including all subfields)
    /// (offset to start of next entry)
    pub entry_size: u16,
    /// Name Length least-significant 8 bits
    pub name_length: u8,
    /// Type indicator (only if the feature bit for "directory entries have file type byte" is set, else this is the most-significant 8 bits of the Name Length)
    pub type_indicator: TypeIndicator,

    pub name: NulStr,
}

#[derive(Debug)]
pub enum TypeIndicator {
    Unknown,
    Regular,
    Directory,
    Character,
    Block,
    Fifo,
    Socket,
    Symlink,
}

bitflags! {
    pub struct TypePerm: u16 {
        /// FIFO
        const FIFO = 0x1000;
        /// Character device
        const CHAR_DEVICE = 0x2000;
        /// Directory
        const DIRECTORY = 0x4000;
        /// Block device
        const BLOCK_DEVICE = 0x6000;
        /// Regular file
        const FILE = 0x8000;
        /// Symbolic link
        const SYMLINK = 0xA000;
        /// Unix socket
        const SOCKET = 0xC000;
        /// Other—execute permission
        const O_EXEC = 0x001;
        /// Other—write permission
        const O_WRITE = 0x002;
        /// Other—read permission
        const O_READ = 0x004;
        /// Group—execute permission
        const G_EXEC = 0x008;
        /// Group—write permission
        const G_WRITE = 0x010;
        /// Group—read permission
        const G_READ = 0x020;
        /// User—execute permission
        const U_EXEC = 0x040;
        /// User—write permission
        const U_WRITE = 0x080;
        /// User—read permission
        const U_READ = 0x100;
        /// Sticky Bit
        const STICKY = 0x200;
        /// Set group ID
        const SET_GID = 0x400;
        /// Set user ID
        const SET_UID = 0x800;
    }
}
