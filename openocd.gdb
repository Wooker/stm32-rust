target remote :3333
set print asm-demangle on
set print pretty on
set backtrace limit 32

break main

#monitor arm semihosting enable
#monitor tpiu config internal itm.fifo uart off 8000000
#monitor itm port 0 on

load
continue
layout src
