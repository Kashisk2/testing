NAME =  wuw

clean:
	rm -f target/debug/$(NAME)
	rm -f target/release/$(NAME)

target/debug/$(NAME):
	cargo build

target/release/$(NAME):
	cargo build --release

.PHONY: docker
docker:
	docker build -t $(NAME) --build-arg APP_NAME=$(NAME) .

.PHONY: build
build: clean target/debug/$(NAME)

.PHONY: release
release: clean target/release/$(NAME)
