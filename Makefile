# Compile for architecture
ARCH =

# Architecture target
TARGET_ARCH =

# Target for the host device
HOST_TARGET =

# Target device
PLATFORM = none
# Build features
FEATURES =

# Output library for linking
# true/false
LIB_OUTPUT = false

# Command[s] for the architecture being compiled
ARCH_COMMAND =
# Novusk build command
NOVUSK_BUILD =

ifeq ($(ARCH), aarch64)
	TARGET_ARCH = aarch64-novusk
	ARCH_COMMAND = RUSTFLAGS="-Larch/aarch64/build/libentry.a"
else ifeq ($(ARCH), armv8-a)
	TARET_ARCH = aarch64-novusk
else ifeq ($(ARCH), arm)
	TARGET_ARCH = thumbv7em-none-eabihf
else ifeq ($(ARCH), armv7-a)
	TARGET = arm-a-novusk
else ifeq ($(ARCH), armv7-m)
	TARGET_ARCH = thumbv7m-none-eabi
else ifeq ($(ARCH), armv7e-m)
	TARGET_ARCH = thumbv7em-none-eabihf
else ifeq ($(ARCH), riscv32)
	TARGET_ARCH = riscv32imac-unknown-none-elf
else ifeq ($(ARCH), riscv32imac)
	TARGET_ARCH = riscv32imac-unknown-none-elf
else ifeq ($(ARCH), x86_64)
	TARGET_ARCH = x86_64-novusk
else ifeq ($(ARCH), xtensa)
	TARGET_ARCH = xtensa-$(PLATFORM)-novusk
endif

ifeq ($(LIB_OUTPUT), true)
	NOVUSK_BUILD = cargo rustc --release --lib --crate-type=staticlib --features $(PLATFORM),$(FEATURES) --target targets/$(TARGET_ARCH).json
else ifeq ($(LIB_OUTPUT), false)
	NOVUSK_BUILD = cargo rustc --release --bin novusk_kernel --features $(PLATFORM),$(FEATURES) --target targets/$(TARGET_ARCH).json
endif

novusk:
	@ echo "Compiling $(ARCH) Novusk..."
	$(ARCH_COMMAND)
	$(NOVUSK_BUILD)

libc:
	@ $(MAKE) -C lib/cinclude TARGET=$(TARGET_ARCH) HOST=$(HOST_TARGET)

clean:
	@ cargo clean
	@ $(MAKE) -C lib/cinclude/ clean
	@ rm -rf arch/aarch64/src/dif.rs
	@ rm -rf arch/arm/src/dif.rs
	@ rm -rf arch/riscv/src/dif.rs
	@ rm -rf arch/x86_64/src/dif.rs
	@ rm -rf arch/xtensa/src/dif.rs
