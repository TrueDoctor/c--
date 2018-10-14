CC ?= gcc
CCFLAGS += -Wall
LDFLAGS += -lstdc++
PROGRAM ?= inter-fuck
PREFIX ?= /usr/local

OBJS := $(wildcard interfuck/*.cpp) $(wildcard interfuck/*.h)
OBJS := $(OBJS:.c=.o)

%.o: %.cpp %.h
	$(CC) $(CCFLAGS) -c -o $@ $<

$(PROGRAM): $(OBJS)
	$(CC) $(LDFLAGS) -o $@ $<

install: $(PROGRAM)
	mkdir -p $(PREFIX)/bin
	cp $(PROGRAM) $(PREFIX)/bin/

all: $(PROGRAM)

.PHONY: install
