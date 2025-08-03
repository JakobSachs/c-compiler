#!/usr/bin/env fish

# Check if an input file was provided
if test -z "$argv[1]"
    echo "Usage: build.fish <source_file.c>"
    echo "  Example: build.fish my_program.c"
    exit 1
end

set input_file $argv[1]
set base_name "$(dirname $input_file)/$(basename $input_file .c)"
set asm_file "$base_name.s"

echo "Compiling '$input_file'..."
# Run the compiler to generate assembly
cargo r --bin compiler "$input_file"
if test $status -ne 0
    echo "Error: Compilation failed."
    exit 1
end

# Check if the assembly file was generated
if not test -f "$asm_file"
    echo "Error: Assembly file '$asm_file' not found."
    exit 1
end

# Extract the base name (without extension) for the output executable
set output_name $base_name

echo "Assembling '$asm_file'..."
# Assemble the .s file into a .o object file
as "$asm_file" -o "$output_name.o"
rm -f $asm_file
if test $status -ne 0
    echo "Error: Assembly failed."
    exit 1
end

echo "Linking '$output_name.o' into '$output_name'..."
# Link the .o file into an executable
ld -o "$output_name" "$output_name.o" \
   -lSystem \
   -syslibroot (xcrun -sdk macosx --show-sdk-path) \
   -e _start \
   -arch arm64
if test $status -ne 0
    echo "Error: Linking failed."
    # Clean up the object file on linking failure
    rm -f "$output_name.o"
    rm -f $asm_file
    exit 1
end

echo "Successfully created executable: './$output_name'"
# Clean up the object file and generated assembly file
rm "$output_name.o"
