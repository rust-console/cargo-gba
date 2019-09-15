.global __start

.include "mmio.s"

.text
.arm

__start:
  b main
  .include "rom-header.s"

main:
  @ set video mode
  ldr r0, =DISPCNT
  ldr r1, =0x0403
  str r1, [r0]

  @ draw a red pixel at (1,3) in VRAM
  ldr r2, =VRAM+2*(3*240+1)
  ldr r3, =31
  strh r3, [r2]

  @ Loop Forever
  loop_point:
    b loop_point
