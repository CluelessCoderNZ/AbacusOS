# Abacus OS
Named as it is going back to the basics of how an operating system is developed.

### Dependencies
+ To simplify the build procedure the project uses the bootimage cargo package. This allows for the kernel to be compiled
and then concatenated with the bootloader, creating a single bootable image. To install use
`cargo bootimage` 

+ Some dependencies of the os require rustup packages. 
   + `rustup component add llvm-tools-preview`

+ Requires the nightly build of rust at the time of writing.

### Creating a Bootable Image
By default the bootimage package creates a bootable image however it is missing partitions.
This is undesirable as having a partitioned filesystem is very useful for hardware testing without risking
other drives. To create this manual you can use `qemu-img resize <img> <size>` to set the image size for the device
you plan to use. Then partition a filesystem using `fdisk <img>`. You can then create a loop back of the file to edit it 
as a device using `losetup --partscan --show --find <img>`. Format the drive using `mkfs -t vfat /dev/<loop device>`. 
Optionally mount the loop device to edit files. Then close the loop device using `losetup -d /dev/<loop device>`