# -- Define variables. -- #
BUILD_DIR = target/doc
SRC = src/*.rs
LANDING = landing/*.html landing/*.css
RESOURCES = resources/* resources/.nojekyll

# -- Define build rules -- #
.PHONY: all clean test docs publish

default: all

clean:
	rm -rf $(BUILD_DIR)

# Generate everything, but just leave it in the build directory.
all: test docs landing resources

test:
	cargo test

docs:
	env RUSTDOCFLAGS="--html-after-content src/includes/media-playback-speed.html" cargo doc --no-deps --document-private-items

# Fancy landing page components.
landing: $(LANDING) target_dir
	cp $(LANDING) target/doc

# GitHub pages settings, favicon, RSS feed, etc. Note: the RSS feed is updated
# manually, so running `make modules` won't update it.
resources: $(RESOURCES) target_dir
	cp $(RESOURCES) target/doc

target_dir:
	mkdir -p $(BUILD_DIR)
