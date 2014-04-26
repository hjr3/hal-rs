CC=rustc

%: src/%.rs
	$(CC) --test $<

test: hal
	./$<
