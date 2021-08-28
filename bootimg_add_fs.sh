#!/bin/bash

BOOT_IMAGE='target/x86-64-abacus_os/debug/bootimage-abacus_os.bin'
FS_PART_SIZE='4G'
BOOT_SIZE=$(ls -l $BOOT_IMAGE | awk '{print $5}')
BOOT_SECTOR_SIZE=$(($BOOT_SIZE+511/512)) # Round up
PARTITION_ID=4
MOUNT_DIR='/mnt/abacus_os'

# Resize image to fit filesystem
qemu-img resize $BOOT_IMAGE +$FS_PART_SIZE

# Add filesystem partition
(
echo n # Add a new partition
echo p # Primary partition
echo $PARTITION_ID # Partition number
echo $(($BOOT_SECTOR_SIZE + 1)) # Set filesystem after boot section
echo   # Last sector (Accept default: end of drive)
echo w # Write changes
) | fdisk $BOOT_IMAGE

# Format  filesystem partition
LOOPBACK_DEVICE=$(losetup --partscan --show --find $BOOT_IMAGE)
PARTITION_DEVICE=$LOOPBACK_DEVICE'p'$PARTITION_ID
mkfs -t vfat $PARTITION_DEVICE

if [$1 -eq 'mount']
then
  mkdir $MOUNT_DIR
  mount $PARTITION_DEVICE $MOUNT_DIR
else
  losetup -d $LOOPBACK_DEVICE
fi

echo "Filesystem Created"