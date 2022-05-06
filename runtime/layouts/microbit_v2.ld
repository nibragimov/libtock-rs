/* Layout for the micro:bit v2 board, used by the examples in this repository. */

MEMORY {
  FLASH (X) : ORIGIN = 0x00040000, LENGTH = 256K
  RAM (W) : ORIGIN = 0x20004000, LENGTH = 112K
}
/* Need to add entry in TBF header about writeable flash region for app_state */
TBF_HEADER_SIZE = 0x50;
INCLUDE libtock_layout.ld
