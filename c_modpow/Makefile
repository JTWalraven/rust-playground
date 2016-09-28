# The compiler: gcc for C program, define as g++ for C++
CC = gcc
#
# Compiler flags:
#  -g    adds debugging information to the executable file
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -g -Wall -O3

# The main source file
MAIN = main.c

# The build target executable:
TARGET = target/main

all: $(TARGET)

run: $(TARGET)
	$(TARGET)

$(TARGET): $(MAIN)
	$(CC) $(CFLAGS) -o $(TARGET) $(MAIN)

clean:
	$(RM) $(TARGET)
