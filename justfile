probers := "probe-rs"
chip := "STM32F072RBTx"

run:
    {{probers}} run --verify --chip {{chip}} ./target/thumbv7m-none-eabi/debug/rust-stm32f072b-disco-test

gdb:
    {{probers}} gdb --chip {{chip}}

list:
    {{probers}} list --chip {{chip}}

erase:
    {{probers}} erase --chip {{chip}}

reset:
    {{probers}} reset --chip {{chip}}
