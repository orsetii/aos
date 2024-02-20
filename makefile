# Compiler and linker
CC = riscv64-unknown-linux-gnu-gcc
LD = riscv64-unknown-linux-gnu-ld
AS = riscv64-unknown-linux-gnu-as 

# Standard C/C++ flags
CFLAGS = -march=rv64g -mabi=lp64d -Wall -Wextra -O2 -g
ASFLAGS= -g


BUILD_DIR=build


# Source Files
OS_SOURCES := $(shell find src/ -name '*.c')
OS_ASM_SOURCES := $(shell find src/ -name '*.s')


# Object files
OS_OBJECTS := $(OS_SOURCES:.c=.o) $(OS_ASM_SOURCES:.s=.o)
OS_OBJECTS := $(addprefix $(BUILD_DIR)/, $(OS_OBJECTS))

# Name of your kernel executable
KERNEL_TARGET = $(BUILD_DIR)/walnut

# Linking step 
$(BUILD_DIR)/$(KERNEL_TARGET): $(OS_OBJECTS)
	$(LD) -T linker.ld -o $(KERNEL_TARGET) $(OS_OBJECTS)

# Compilation rules
$(BUILD_DIR)/%.o: %.c
	$(MKDIR_P) $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.s
	$(MKDIR_P) $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# Build target
build: $(KERNEL_TARGET)

# Run target
run: build
	qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 512M -serial mon:stdio -bios none -kernel $(KERNEL_TARGET) 

debug: build
	qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 512M -serial mon:stdio -bios none -kernel $(KERNEL_TARGET) -s -S &
	riscv64-unknown-elf-gdb $(KERNEL_TARGET) --tui -ex "target remote :1234" -x ./gdb/config.gdb

no_display_run: build
	qemu-system-riscv64 -machine virt -nographic -bios none -kernel $(KERNEL_TARGET) 


# Clean target 
clean:
	rm -f $(KERNEL_TARGET) $(OS_OBJECTS)

MKDIR_P = mkdir -p
