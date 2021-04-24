BOARD =?

kernel:
	@ cd ../../ && cargo build --target=targets/x86_64-novusk.json
	@ echo "Done compiling kernel"

boot:
	@ cd ../../ && cargo bootimage --target=targets/x86_64-novusk.json

install:
	@ echo "Install Novusk x86_64 build requirements..."
	@ echo "Updating rust..."
	@ rustup update
	@ echo "Adding components..."
	@ rustup component add rust-src
	@ rustup component add llvm-tools-preview
	@ echo "Installing bootimage..."
	@ cargo install bootimage
