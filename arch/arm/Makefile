BOARD =?

kernel: setup build

setup:
	@ echo "Setting up arch/arm/ before compiling..."
	@ cd ../../ && python3 tools/arm/setup.py $(BOARD)

build:
	@ echo "Compiling Novusk..."
	@ cd ../../ && sh tools/arm/build.sh $(BOARD)

boot:
	@ cd ../../ && sh tools/arm/boot.sh $(BOARD)
