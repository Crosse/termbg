BIN	:= termbg
SRCS	:= $(wildcard *.c)
OBJS	:= $(SRCS:.c=.o)

$(BIN): $(OBJS)
	$(CC) $(CFLAGS) -o "$@" "$<"

.PHONY: clean
clean:
	$(RM) -f $(BIN) $(OBJS)
