all: build_lib

build_lib:
	cd ./raw_processor && cargo ndk -t armeabi-v7a -t arm64-v8a -o ./../app/src/main/jniLibs build --release
