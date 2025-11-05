SOURCES := $(wildcard *.slang)
TARGETS := $(SOURCES:.slang=.spv)

all: $(TARGETS)

%.spv: %.slang
	slangc $< -profile glsl_450 -target spirv -o $@ -entry computeMain

.PHONY: clean
clean:
	rm -f $(TARGETS)
