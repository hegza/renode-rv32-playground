# Platform naming protocol

## Requirements

1. Target arch: all targets are bare metal, so the rest of the triple is not necessary (OS + ABI)
2. Target sub (ext): very important for RISC-V software
3. Executing core identifier
4. Specifier: extra specifier to control features

i.e.

`<arch><sub>-<coreid>-<specifier>`

e.g.
`riscv32imac-ibex`
`riscv32imac-ibex-plic`

## Various notes

// Create a GPIO and connect a LED to it
gpio_out: GPIOPort.LiteX_GPIO @ sysbus 0x60000800
    type: Type.Out
    0 -> led0@0
    1 -> led1@0
led0: Miscellaneous.LED @ gpio_out 0
led1: Miscellaneous.LED @ gpio_out 1
