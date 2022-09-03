CC = cargo build
CFLAGS = --release

all: tar

tar: multiplatform
	$(CC) $(CFLAGS)
	tar -czvf target/release/$(NAME).tar.gz target/release/$(NAME)/fput

multiplatform: linux64 win64 macos macosaarch64

linux64:
	$(CC) $(CFLAGS) --target x86_64-unknown-linux-gnu

linuxaarch64:
	$(CC) $(CFLAGS) --target aarch64-unknown-linux-gnu

win64:
	$(CC) $(CFLAGS) --target x86_64-pc-windows-gnu

win32:
	$(CC) $(CFLAGS) --target i686-pc-windows-gnu

macos:
	$(CC) $(CFLAGS) --target x86_64-apple-darwin

macosaarch64:
	$(CC) $(CFLAGS) --target aarch64-apple-darwin

clean:
	cargo clean