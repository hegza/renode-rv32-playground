# Notes on scripts

## Snippets

### Start a GDB server and connect two leds to it

machine StartGdbServer 3333
logLevel -1 sysbus.gpio_out.led0
logLevel -1 sysbus.gpio_out.led01
