
This is a starting point for parsing and navigating ext2 file systems.
`cargo run` will start a session that looks like a shell. All you can
do for now are the `ls`, and `cd` commands.
It's left as an exercise to implement `cat` to view the contents of files,
and removing other limitations.

Here's an example session:
```
% cargo run
   <building and intro stuff>
:> ls
.	..	lost+found	test_directory	hello.txt	
:> cat hello.txt
cat not yet implemented
:> cd test_directory
:> ls
.	..	file_in_folder.txt	
:> cd file_in_folder.txt    # <- whoops
:> ls
'm a file inside a folder.  # <- whoops^2
	
:> 
```

Limitations (also possible exercises):

 - ~~see "TODO" in `cd` command - you can currently `cd` into a text file - whoops!~~
 - implement `cat` command to view text files
 - currently it only parses small directories, remove this limitation
 - implement `mkdir`
 - implement `link <source name> <destination path>` to create hard
   links
 - write tests
 - write more tests
 - implement `rm` (aka unlink) for plain files
 - make `link` robust against ... (what should `link` be robust
   against?)
 - once modifications can be made, implement `unmount` which cleanly
   writes modifications back to the "device" (file)
 - implement `import` to get a file from the "host" filesystem into
   ours
 - implement a `mount <host-file> <dirname>` command to mount a local file as an ext2
   filesystem over an empty directory.


Big projects:

 - make it `#[no_std]` compatible
 - instead of reading from a big byte-buffer, read from a device into
   manually managed page-sized buffers
 - implement a buffer cache
 - implement `fsck` - identify different inconsistencies and find them
 - implement a simple line editor (ed?) to create text files in the
   filesystem

Bigger projects:

 - ext4 support?
 - integrate with reedos kernel memory allocation
 - integrate caching with kernel VM

Credits: Reed College CS393 students, @tzlil on the Rust #osdev discord
