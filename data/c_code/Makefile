compile_main: clean main

clean:
	rm -f main

main:
	gcc -static -o main main.c

run: clean main
	./main

strace: clean main
	strace ./main

