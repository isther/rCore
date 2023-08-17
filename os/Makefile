BOARD = qemu
TARGET = riscv64gc-unknown-none-elf
KERNEL_ELF = target/$(TARGET)/release/os
KERNEL_BIN = $(KERNEL_ELF).bin

# Qemu
SBI := rustsbi-qemu
BOOTLOADER := ../bootloader/$(SBI).bin
KERNEL_ENTRY_PA_QEMU := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

run: build
	@qemu-system-riscv64 $(QEMU_ARGS)

QEMU_ARGS := -machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA_QEMU)


build: env kernel-bin

kernel-bin: kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

kernel:
	@echo Platform: $(BOARD)
	@cargo build --release --target $(TARGET)


env:
	(rustup target list | grep "$(TARGET) (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

clean:
	@cargo clean
	@rm src/link_app.asm

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
