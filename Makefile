# Compile for architecture
ARCH =
BOOT_METHOD =

# Architecture target
TARGET_ARCH =

# Target for the host device
HOST_TARGET =

# Target device
PLATFORM =
# Build features
FEATURES =

# Name of tool that's being used
TOOL =
TOOL_ARGS =

# Command[s] for the architecture being compiled
ARCH_ARGUMENTS =
# Novusk build command
NOVUSK_BUILD =

ifeq ($(ARCH), aarch64)
	TARGET_ARCH = aarch64-novusk
	ARCH_ARGUMENTS = PLATFORM=$(PLATFORM)
else ifeq ($(ARCH), armv8-a)
	TARET_ARCH = aarch64-novusk
	ARCH = aarch64
else ifeq ($(ARCH), arm)
	TARGET_ARCH = thumbv7em-none-eabihf
else ifeq ($(ARCH), armv7-a)
	TARGET = arm-a-novusk
	ARCH = arm
else ifeq ($(ARCH), armv7-m)
	TARGET_ARCH = thumbv7m-none-eabi
	ARCH = arm
else ifeq ($(ARCH), armv7e-m)
	TARGET_ARCH = thumbv7em-none-eabihf
	ARCH = arm
else ifeq ($(ARCH), riscv)
	TARGET_ARCH = riscv32imac-unknown-none-elf
else ifeq ($(ARCH), riscv32)
	ARCH = riscv
	TARGET_ARCH = riscv32imac-unknown-none-elf
else ifeq ($(ARCH), riscv32imac)
	TARGET_ARCH = riscv32imac-unknown-none-elf
	ARCH = riscv
else ifeq ($(ARCH), x86_64)
	TARGET_ARCH = x86_64-novusk
else ifeq ($(ARCH), xtensa)
	TARGET_ARCH = xtensa-$(PLATFORM)-novusk
endif

novusk: build_arch build_kernel link
	@ echo "Compiling $(ARCH) Novusk..."

build_arch:
	@ echo "Compiling $(ARCH) specific code..."
	make -C arch/$(ARCH)/ build

build_kernel:
	@ echo "Compiling Novusk..."
	cargo rustc -p $(ARCH)@0.1.0 --release --crate-type=staticlib --features $(PLATFORM),$(FEATURES) --target targets/$(TARGET_ARCH).json

link:
	@ echo "Linking..."
	make -C arch/$(ARCH)/ link BOOT_METHOD=$(BOOT_METHOD) TARGET=$(TARGET_ARCH)

libc:
	@ $(MAKE) -C lib/cinclude TARGET=$(TARGET_ARCH) HOST=$(HOST_TARGET)

build_tool:
	@ cargo build -p $(TOOL) --target $(HOST_TARGET)

run_tool: build_tool
	@ ./target/$(HOST_TARGET)/debug/$(TOOL) $(TOOL_ARGS)

clean:
	@ cargo clean
	@ $(MAKE) -C lib/cinclude/ clean
	@ rm -rf arch/aarch64/src/dif.rs
	@ rm -rf arch/arm/src/dif.rs
	@ rm -rf arch/riscv/src/dif.rs
	@ rm -rf arch/x86_64/src/dif.rs
	@ rm -rf arch/xtensa/src/dif.rs
