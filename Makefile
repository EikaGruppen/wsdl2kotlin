
compile:
	@cargo build --release

cross_compile:
	@echo "Building for Apple Silicon..."
	@cargo build --release --target aarch64-apple-darwin
	@echo "Building for Mac Intel..."
	@cargo build --release --target x86_64-apple-darwin
	@echo "Building for Linux..."
	@CC_x86_64_unknown_linux_musl="x86_64-unknown-linux-musl-gcc" cargo build --release --target x86_64-unknown-linux-musl

move:
	@cp target/release/wsdl2kotlin ../kotlin-wsdl-wrapper-maven-plugin/src/main/resources/generate
	@cp target/x86_64-unknown-linux-musl/release/wsdl2kotlin ../kotlin-wsdl-wrapper-maven-plugin/src/main/resources/generate_linux
