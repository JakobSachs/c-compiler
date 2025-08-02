#!/usr/bin/env fish

# Check if an input file was provided
if test -z "$argv[1]"
    echo "Usage: asm_build.fish <assembly_file.s>"
    echo "  Example: asm_build.fish my_program.s"
    exit 1
end

set input_file $argv[1]

# Extract the base name (without extension) for the output executable
set output_name (basename $input_file .s)

# Check if the input file exists
if not test -f "$input_file"
    echo "Error: Input file '$input_file' not found."
    exit 1
end

echo "Assembling '$input_file'..."
# Assemble the .s file into a .o object file
as "$input_file" -o "$output_name.o"
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
    exit 1
end

echo "Successfully created executable: './$output_name'"
# Clean up the object file
rm "$output_name.o"
