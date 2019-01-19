CC ?= gcc
CCFLAGS += -Wall -Ofast
LDFLAGS += -lstdc++
JANSFLAGS ?= -Ofast
PROGRAM ?= inter-fuck
PREFIX ?= /usr/local

OBJS := $(wildcard interfuck/*.cpp) $(wildcard interfuck/*.h)
OBJS := $(OBJS:.c=.o)

%.o: %.cpp %.h
	$(CC) $(CCFLAGS) $(JANSFLAGS) -c -o $@ $<

$(PROGRAM): $(OBJS)
	$(CC) $(LDFLAGS) $(JANSFLAGS) -o $@ $<

install: $(PROGRAM)
	mkdir -p $(PREFIX)/bin
	cp $(PROGRAM) $(PREFIX)/bin/

all: $(PROGRAM)

.PHONY: install
