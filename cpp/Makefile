# Makefile for compiling and running shapes program

# Compiler to use
CXX = g++

# Compiler flags
CXXFLAGS = -Wall -std=c++11 -O3 -march=native

# Target executable name
TARGET = clean

# Source files
SRCS = clean.cpp

# Object files
OBJS = $(SRCS:.cpp=.o)

# Default target to compile the program
all: $(TARGET)

# Rule to link object files and create executable
$(TARGET): $(OBJS)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(OBJS)

# Rule to compile source files into object files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@


# Clean up object files and executable
clear:
	rm -f $(OBJS) $(TARGET)

# Run the program
run: $(TARGET)
	./$(TARGET)

rust:
	RUSTFLAGS="-C target-cpu=native" cargo run --release

cpp: $(TARGET)
	./$(TARGET)
