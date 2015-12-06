# -- Define variables. -- #
BUILD_DIR = target/doc
SRC = src/*.rs
LANDING = landing/index.html landing/landing.css
RESOURCES = resources/* resources/.nojekyll

# -- Define build rules -- #
.PHONY: all clean test modules publish

default: all

clean:
	rm -rf $(BUILD_DIR)

publish: all
	ghp-import -n $(BUILD_DIR) -m "Update with latest changes from master."
	@git push -fq https://${TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages

# Generate everything, but just leave it in the build directory.
all: test modules landing resources

test: $(SRC)
	cargo test

modules: $(SRC)
	cargo doc

# Fancy landing page components.
landing: $(LANDING) target_dir
	cp $(LANDING) target/doc

# GitHub pages settings, favicon, RSS feed, etc. Note: the RSS feed is updated
# manually, so running `make modules` won't update it.
resources: $(RESOURCES) target_dir
	cp $(RESOURCES) target/doc

target_dir:
	mkdir -p $(BUILD_DIR)
