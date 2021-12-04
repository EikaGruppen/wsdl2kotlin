
compile:
	@cargo build --release

cross_compile:
	@echo "Building for MacOS..."
	@cargo build --release
	@echo "Building for Linux..."
	@cargo build --release --target x86_64-unknown-linux-musl

move:
	@cp target/release/wsdl2kotlin ../kotlin-wsdl-wrapper-maven-plugin/src/main/resources/generate
	@cp target/x86_64-unknown-linux-musl/release/wsdl2kotlin ../kotlin-wsdl-wrapper-maven-plugin/src/main/resources/generate_linux
