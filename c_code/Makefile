.PHONY: fmt

SRC_DIR=./src

SOURCE = $(wildcard $(SRC_DIR)/*.c)
HEADERS = $(wildcard $(SRC_DIR)/*.h)

fmt:
	clang-format --style=file -i $(SOURCE) $(HEADERS)
