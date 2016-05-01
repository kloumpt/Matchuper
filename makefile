BUILD_PATH=build
SRI_RS_REPOSITORY="https://github.com/kloumpt/sri_rs.git"
SRI_RS_PROJECT=sri_rs

all: doc compile

clean: clean-build clean-compile

doc:
	@echo "Generating matchuper documentation"
	cargo doc
	@echo


compile: sri-rs matchuper

clean-compile: clean-sri-rs clean-matchuper


matchuper:
	@echo "Compiling the latest version of matchuper"
	cargo build --release
	@echo

clean-matchuper:
	cargo clean


sri-rs:
	@echo "Retrieving the latest version of sri_rs into $(SRI_RS_PROJECT)"
	sh -c "if cd $(SRI_RS_PROJECT); then git pull; else git clone $(SRI_RS_REPOSITORY) $(SRI_RS_PROJECT); fi"
	@echo "Compiling the latest version of sri_rs"
	sh -c "cd $(SRI_RS_PROJECT) && cargo build --release"
	@echo

clean-sri-rs:
	sh -c "cd $(SRI_RS_PROJECT) && cargo clean"
