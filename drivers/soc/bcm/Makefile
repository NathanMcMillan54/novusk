all:
	@ cargo build --release -Z build-std=core --target ../../../targets/aarch64-novusk.json
	@ mv target/aarch64-novusk/release/libbcm.a ../../../include/novusk/kms/soc.km
