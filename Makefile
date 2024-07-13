SHELL=/bin/bash
.SUFFIXES:
CARGO?=cargo
UNAME_S:=$(shell uname -s)

# setup target api level
API:=34

# NDK version
NDK_VERSION?=26.3.11579264

# setup ANDROID_HOME
ifeq ($(UNAME_S),Linux)
    ANDROID_HOME?=/opt/android-sdk-linux
else ifeq ($(UNAME_S),Darwin)
    ANDROID_HOME?=/opt/android-sdk-macosx
endif

# setup NDK
NDK?=$(ANDROID_HOME)/ndk/$(NDK_VERSION)

# setup HOST_TAG, TOOLCHAIN and LINKER https://developer.android.com/ndk/guides/other_build_systems
ifeq ($(UNAME_S),Linux)
    HOST_TAG:=linux-x86_64
else ifeq ($(UNAME_S),Darwin)
    HOST_TAG:=darwin-x86_64
endif

TOOLCHAIN:=$(NDK)/toolchains/llvm/prebuilt/$(HOST_TAG)

# setup LINKER
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$(TOOLCHAIN)/bin/aarch64-linux-android$(API)-clang
export CARGO_TARGET_ARM_LINUX_ANDROIDEABI_LINKER=$(TOOLCHAIN)/bin/armv7a-linux-androideabi$(API)-clang
export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$(CARGO_TARGET_ARM_LINUX_ANDROIDEABI_LINKER)
export CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$(TOOLCHAIN)/bin/i686-linux-android$(API)-clang
export CARGO_TARGET_THUMBV7NEON_LINUX_ANDROIDEABI_LINKER=$(CARGO_TARGET_ARM_LINUX_ANDROIDEABI_LINKER)
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$(TOOLCHAIN)/bin/x86_64-linux-android$(API)-clang

# `--locked` for ci.
ifdef CI
APP_CARGO_LOCKED=--locked
else
APP_CARGO_LOCKED=
endif

.PHONY: debug
debug:
	$(CARGO) build $(APP_CARGO_LOCKED) --target aarch64-linux-android --target arm-linux-androideabi --target armv7-linux-androideabi --target i686-linux-android --target thumbv7neon-linux-androideabi --target x86_64-linux-android

.PHONY: release
release:
	$(CARGO) build $(APP_CARGO_LOCKED) --release --target aarch64-linux-android --target arm-linux-androideabi --target armv7-linux-androideabi --target i686-linux-android --target thumbv7neon-linux-androideabi --target x86_64-linux-android

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: test
test:
	$(CARGO) test $(APP_CARGO_LOCKED)

.PHONY: setup-ndk
setup-ndk:
	$(ANDROID_HOME)/cmdline-tools/latest/bin/sdkmanager --install 'ndk;$(NDK_VERSION)'

.PHONY: setup-rust-target
setup-rust-target:
	rustup target add aarch64-linux-android arm-linux-androideabi armv7-linux-androideabi i686-linux-android thumbv7neon-linux-androideabi x86_64-linux-android

.PHONY: print-vars
print-vars:
	$(info API: $(API))
	$(info NDK_VERSION: $(NDK_VERSION))
	$(info TOOLCHAIN: $(TOOLCHAIN))
	$(info JAVA_HOME: $(JAVA_HOME))
	$(info ANDROID_HOME: $(ANDROID_HOME))
	$(info CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER: $(CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER))
	$(info CARGO_TARGET_ARM_LINUX_ANDROIDEABI_LINKER: $(CARGO_TARGET_ARM_LINUX_ANDROIDEABI_LINKER))
	$(info CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER: $(CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER))
	$(info CARGO_TARGET_I686_LINUX_ANDROID_LINKER: $(CARGO_TARGET_I686_LINUX_ANDROID_LINKER))
	$(info CARGO_TARGET_THUMBV7NEON_LINUX_ANDROIDEABI_LINKER: $(CARGO_TARGET_THUMBV7NEON_LINUX_ANDROIDEABI_LINKER))
	$(info CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER: $(CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER))
	$(info APP_CARGO_LOCKED: $(APP_CARGO_LOCKED))
	@true
