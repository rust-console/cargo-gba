
/* LCD */

.set DISPCNT, 0x4000000 @ LCD Control
.set DISPSTAT, 0x4000004 @ General LCD Status
.set VCOUNT, 0x4000006 @ Vertical Counter

.set BG0CNT, 0x4000008 @ BG0 Control
.set BG1CNT, 0x400000A @ BG1 Control
.set BG2CNT, 0x400000C @ BG2 Control
.set BG3CNT, 0x400000E @ BG3 Control

.set BG0HOFS, 0x4000010 @ BG0 X-Offset
.set BG0VOFS, 0x4000012 @ BG0 Y-Offset
.set BG1HOFS, 0x4000014 @ BG1 X-Offset
.set BG1VOFS, 0x4000016 @ BG1 Y-Offset
.set BG2HOFS, 0x4000018 @ BG2 X-Offset
.set BG2VOFS, 0x400001A @ BG2 Y-Offset
.set BG3HOFS, 0x400001C @ BG3 X-Offset
.set BG3VOFS, 0x400001E @ BG3 Y-Offset

.set BG2PA, 0x4000020 @ BG2 Affine Parameter A (dx)
.set BG2PB, 0x4000022 @ BG2 Affine Parameter B (dmx)
.set BG2PC, 0x4000024 @ BG2 Affine Parameter C (dy)
.set BG2PD, 0x4000026 @ BG2 Affine Parameter D (dmy)
.set BG2X, 0x4000028 @ BG2 Reference Point X-Coordinate
.set BG2Y, 0x400002C @ BG2 Reference Point Y-Coordinate

.set BG3PA, 0x4000030 @ BG3 Affine Parameter A (dx)
.set BG3PB, 0x4000032 @ BG3 Affine Parameter B (dmx)
.set BG3PC, 0x4000034 @ BG3 Affine Parameter C (dy)
.set BG3PD, 0x4000036 @ BG3 Affine Parameter D (dmy)
.set BG3X, 0x4000038 @ BG3 Reference Point X-Coordinate
.set BG3Y, 0x400003C @ BG3 Reference Point Y-Coordinate

.set WIN0H, 0x4000040 @ Window 0 Horizontal Dimensions
.set WIN1H, 0x4000042 @ Window 1 Horizontal Dimensions
.set WIN0V, 0x4000044 @ Window 0 Vertical Dimensions
.set WIN1V, 0x4000046 @ Window 1 Vertical Dimensions
.set WININ, 0x4000048 @ Inside of Window 0 and 1
.set WINOUT, 0x400004A @ Inside of OBJ Window & Outside of Windows

.set MOSAIC, 0x400004C @ Mosaic Size
.set BLDCNT, 0x4000050 @ Color Special Effects Selection
.set BLDALPHA, 0x4000052 @ Alpha Blending Coefficients
.set BLDY, 0x4000054 @ Brightness (Fade-In/Out) Coefficient

/* Sound */

.set SOUND1CNT_L, 0x4000060 @ Channel 1 Sweep register
.set SOUND1CNT_H, 0x4000062 @ Channel 1 Duty/Length/Envelope
.set SOUND1CNT_X, 0x4000064 @ Channel 1 Frequency/Control

.set SOUND2CNT_L, 0x4000068 @ Channel 2 Duty/Length/Envelope
.set SOUND2CNT_H, 0x400006C @ Channel 2 Frequency/Control

.set SOUND3CNT_L, 0x4000070 @ Channel 3 Stop/Wave RAM select
.set SOUND3CNT_H, 0x4000072 @ Channel 3 Length/Volume
.set SOUND3CNT_X, 0x4000074 @ Channel 3 Frequency/Control

.set SOUND4CNT_L, 0x4000078 @ Channel 4 Length/Envelope
.set SOUND4CNT_H, 0x400007C @ Channel 4 Frequency/Control

.set SOUNDCNT_L, 0x4000080 @ Control Stereo/Volume/Enable
.set SOUNDCNT_H, 0x4000082 @ Control Mixing/DMA Control
.set SOUNDCNT_X, 0x4000084 @ Control Sound on/off

.set WAVE_RAM, 0x4000090 @ Channel 3 Wave Pattern RAM

.set FIFO_A, 0x40000A0 @ Channel A FIFO, Data 0-3
.set FIFO_B, 0x40000A4 @ Channel B FIFO, Data 0-3

/* DMA Units */

.set DMA0_SRC, 0x40000B0 @ DMA 0 Source Address
.set DMA0_DST, 0x40000B4 @ DMA 0 Destination Address
.set DMA0_COUNT, 0x40000B8 @ DMA 0 Word Count
.set DMA0_CTRL, 0x40000BA @ DMA 0 Control

.set DMA1_SRC, 0x40000BC @ DMA 1 Source Address
.set DMA1_DST, 0x40000C0 @ DMA 1 Destination Address
.set DMA1_COUNT, 0x40000C4 @ DMA 1 Word Count
.set DMA1_CTRL, 0x40000C6 @ DMA 1 Control

.set DMA2_SRC, 0x40000C8 @ DMA 2 Source Address
.set DMA2_DST, 0x40000CC @ DMA 2 Destination Address
.set DMA2_COUNT, 0x40000D0 @ DMA 2 Word Count
.set DMA2_CTRL, 0x40000D2 @ DMA 2 Control

.set DMA3_SRC, 0x40000D4 @ DMA 3 Source Address
.set DMA3_DST, 0x40000D8 @ DMA 3 Destination Address
.set DMA3_COUNT, 0x40000DC @ DMA 3 Word Count
.set DMA3_CTRL, 0x40000DE @ DMA 3 Control

/* Timers */

.set TIMER0_RELOAD, 0x4000100 @ write only, sets the reload value
.set TIMER0_COUNTER, 0x4000100 @ read only, reads out the counter value
.set TIMER0_CONTROL, 0x4000102 @ RW, Timer 0 control

.set TIMER1_RELOAD, 0x4000104 @ write only, sets the reload value
.set TIMER1_COUNTER, 0x4000104 @ read only, reads out the counter value
.set TIMER1_CONTROL, 0x4000106 @ RW, Timer 1 control

.set TIMER2_RELOAD, 0x4000108 @ write only, sets the reload value
.set TIMER2_COUNTER, 0x4000108 @ read only, reads out the counter value
.set TIMER2_CONTROL, 0x400010A @ RW, Timer 2 control

.set TIMER3_RELOAD, 0x400010C @ write only, sets the reload value
.set TIMER3_COUNTER, 0x400010C @ read only, reads out the counter value
.set TIMER3_CONTROL, 0x400010E @ RW, Timer 3 control

/* Serial Port (part 1) */

.set SIODATA32, 0x4000120 @ SIO Data (Normal-32bit Mode; shared with below)
.set SIOMULTI0, 0x4000120 @ SIO Data 0 (Parent)    (Multi-Player Mode)
.set SIOMULTI1, 0x4000122 @ SIO Data 1 (1st Child) (Multi-Player Mode)
.set SIOMULTI2, 0x4000124 @ SIO Data 2 (2nd Child) (Multi-Player Mode)
.set SIOMULTI3, 0x4000126 @ SIO Data 3 (3rd Child) (Multi-Player Mode)
.set SIOCNT, 0x4000128 @ SIO Control Register
.set SIOMLT_SEND, 0x400012A @ SIO Data (Local of MultiPlayer; shared below)
.set SIODATA8, 0x400012A @ SIO Data (Normal-8bit and UART Mode)

/* Buttons */

.set KEYINPUT, 0x4000130 @ Key Status
.set KEYCNT, 0x4000132 @ Key Interrupt Control

/* Serial Port (part 2) */

.set RCNT, 0x4000134 @ SIO Mode Select/General Purpose Data
.set JOYCNT, 0x4000140 @ SIO JOY Bus Control
.set JOY_RECV, 0x4000150 @ SIO JOY Bus Receive Data
.set JOY_TRANS, 0x4000154 @ SIO JOY Bus Transmit Data
.set JOYSTAT, 0x4000158 @ SIO JOY Bus Receive Status

/* Interrupts */

.set IE, 0x4000200 @ Interrupt Enable Register
.set IF, 0x4000202 @ Interrupt Request Flags / IRQ Acknowledge
.set WAITCNT, 0x4000204 @ Game Pak Waitstate Control
.set IME, 0x4000208 @ Interrupt Master Enable Register

/* Video */

.set VRAM, 0x6000000 @ start of vram, regardless of video mode
