SHELL=/bin/bash
.SUFFIXES:
CARGO=cargo
UNAME_S=$(shell uname -s)

# setup target api level
API=34

# NDK version
NDK_VERSION=26.3.11579264

# setup ANDROID_HOME
ifeq ($(ANDROID_HOME),)
    $(warning set default ANDROID_HOME)
    ifeq ($(UNAME_S),Linux)
        ANDROID_HOME=/opt/android-sdk-linux
    endif
    ifeq ($(UNAME_S),Darwin)
        ANDROID_HOME=/opt/android-sdk-macosx
    endif
endif

# setup NDK
ifeq ($(NDK),)
    NDK=$(ANDROID_HOME)/ndk/$(NDK_VERSION)
endif

# setup TOOLCHAIN
ifeq ($(UNAME_S),Linux)
    TOOLCHAIN=$(NDK)/toolchains/llvm/prebuilt/linux-x86_64
endif
ifeq ($(UNAME_S),Darwin)
    TOOLCHAIN=$(NDK)/toolchains/llvm/prebuilt/darwin-x86_64
endif

# setup LINKER
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

.PHONY: test
test:
	$(CARGO) test

.PHONY: setup-ndk
setup-ndk:
	$(ANDROID_HOME)/cmdline-tools/latest/bin/sdkmanager --install 'ndk;$(NDK_VERSION)'
