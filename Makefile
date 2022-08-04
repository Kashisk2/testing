NAME =  wuw

clean:
	rm -f target/debug/$(NAME)
	rm -f target/release/$(NAME)

target/debug/$(NAME):
	cargo build

target/release/$(NAME):
	cargo build --release

.PHONY: build
build: clean target/debug/$(NAME)

.PHONY: release
release: clean target/release/$(NAME)
