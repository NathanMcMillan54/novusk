TARGET =?
FEATURES =?

all:
	@ cargo build --release --features $(FEATURES) --target $(TARGET)

clean:
	@ cargo clean
