BOARD =?

kernel:
	@ cd ../../ && cargo build --target=targets/x86_64-novusk.json
	@ echo "Done compiling kernel"

boot:
	@ cd ../../ && cargo bootimage --target=targets/x86_64-novusk.json
