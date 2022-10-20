BOARD ?= qemu
TARGET ?= riscv64gc-unknown-none-elf
KERNEL_ELF ?= target/$(TARGET)/release/os
KERNEL_BIN ?= $(KERNEL_ELF).bin

# Qemu
SBI := rustsbi-qemu
BOOTLOADER := ../bootloader/$(SBI).bin
KERNEL_ENTRY_PA_QEMU := 0x80200000

# D1
XTASK ?= ../../rustsbi-d1/xtask/
KERNEL_BIN_FOR_XTASK ?= ../../rCore/os/$(KERNEL_BIN)

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

run: build

ifeq ($(BOARD),qemu)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA_QEMU)

else
	@cd $(XTASK) && cargo run debug --see --kernel $(KERNEL_BIN_FOR_XTASK)

endif

build: env kernel-bin 

kernel-bin: kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

kernel:
	@echo Platform: $(BOARD)
	@cd ../user && make build
	@cp src/linker-$(BOARD).ld src/linker.ld
	@cp src/batch-$(BOARD).rs src/batch.rs
	@cargo build --release --target $(TARGET)


env:board
	(rustup target list | grep "$(TARGET) (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

board:

ifneq ($(BOARD), qemu)

TARGET = riscv64imac-unknown-none-elf

endif

clean:
	@cargo clean
	@rm src/link_app.S
	@rm src/linker.ld
	@rm src/batch.rs

gdb-server: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA_QEMU) \
		-s -S

gdb-client:
	@riscv64-unknown-elf-gdb \
		-ex 'file target/riscv64gc-unknown-none-elf/release/os' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'
