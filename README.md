# Abacus OS
Named as it is going back to the basics of how an operating system is developed.

### Dependencies
+ To simplify the build procedure the project uses the bootimage cargo package. This allows for the kernel to be compiled
and then concatenated with the bootloader, creating a single bootable image. To install use
`cargo bootimage` 

+ Some dependencies of the os require rustup packages. 
   + `rustup component add llvm-tools-preview`

+ Requires the nightly build of rust at the time of writing.