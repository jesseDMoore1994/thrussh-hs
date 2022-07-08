THRUSSHLIB = thrusshlib
LIBTHRUSSHLIB = lib$(THRUSSHLIB).so
DEBUG ?= no
USRLIB = /usr/local/lib

ifneq (DEBUG,no)
  THRUSSH_TARGET = release
  CARGO_OPTS = -r
else
  THRUSSH_TARGET = debug
  CARGO_OPTS = 
endif

THRUSSH_SO = $(THRUSSHLIB)/target/$(THRUSSH_TARGET)/$(LIBTHRUSSHLIB)

$(THRUSSH_SO):
	cd $(THRUSSHLIB) && cargo build $(CARGO_OPTS)

$(USRLIB)/$(LIBTHRUSSHLIB): $(THRUSSH_SO)
	ln -s $(shell pwd)/$(THRUSSH_SO) $(USRLIB)/$(LIBTHRUSSHLIB)
	ldconfig

.PHONY: build 
build: $(THRUSSH_SO)

.PHONY: install
install: $(USRLIB)/$(LIBTHRUSSHLIB)

clean:
	-rm -f .docker $(THRUSSH_SO) $(USRLIB)/$(LIBTHRUSSHLIB)
