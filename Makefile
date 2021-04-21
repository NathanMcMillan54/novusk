ARCH = x86_64
TARGET = targets/$(ARCH)-novusk.json

all: clean novusk boot

novusk:
	@ cargo build --target=$(TARGET)

boot:
	@ cargo bootimage --target=$(TARGET)

clean:
	@ cargo clean
	@ sh tools/clean/main.sh
