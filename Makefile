SHELL=/bin/bash
.SUFFIXES:
CARGO=cargo

API=34
UNAME_S=$(shell uname -s)
ifeq ($(UNAME_S),Linux)
    NDK=/opt/android-sdk-linux/ndk/26.3.11579264
    TOOLCHAIN=$(NDK)/toolchains/llvm/prebuilt/linux-x86_64
endif
ifeq ($(UNAME_S),Darwin)
    NDK=/opt/android-sdk-macosx/ndk/26.3.11579264
    TOOLCHAIN=$(NDK)/toolchains/llvm/prebuilt/darwin-x86_64
endif
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$(TOOLCHAIN)/bin/aarch64-linux-android$(API)-clang
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$(TOOLCHAIN)/bin/armv7a-linux-androideabi$(API)-clang
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$(TOOLCHAIN)/bin/x86_64-linux-android$(API)-clang

.PHONY: debug
debug:
	$(CARGO) build --target aarch64-linux-android --target armv7-linux-androideabi --target x86_64-linux-android

.PHONY: release
release:
	$(CARGO) build --release --target aarch64-linux-android --target armv7-linux-androideabi --target x86_64-linux-android

.PHONY: clean
clean:
	$(CARGO) clean
