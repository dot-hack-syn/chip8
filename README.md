CHIP-8 Emulator
This project is a CHIP-8 emulator written in Rust. It simulates the behavior of a CHIP-8 virtual machine, executing programs (usually games) written for the CHIP-8 architecture. The emulator mimics the hardware, including memory, registers, stack, timers, and display, as well as handling keypresses and audio.

Features
Emulates the CHIP-8 virtual machine, including:
4KB of memory
16 general-purpose registers
16-bit index register
Stack for managing program flow
64x32 pixel screen
16-key hexadecimal keypad
Delay and sound timers
Implements the fetch-decode-execute cycle for processing instructions.
Supports loading CHIP-8 programs into memory and executing them.
How to Use
Prerequisites
You need to have Rust installed on your machine. You can install Rust from the official website:

Rust Installation Guide
Getting Started
Clone the repository:
bash
Copy
git clone https://github.com/yourusername/chip8-emulator.git
cd chip8-emulator
Build the project:
bash
Copy
cargo build --release
Run the emulator:
To run the emulator, pass a CHIP-8 binary file (e.g., a .ch8 file) as an argument:

cargo run -- path_to_chip8_program.ch8
Loading a CHIP-8 Program
The emulator expects the CHIP-8 program to be in binary format (a .ch8 file).
You can find sample CHIP-8 programs online or create your own.
Controls
The emulator emulates a 16-key keypad using the following mapping:
Key	CHIP-8 Key
1	1
2	2
3	3
4	C
Q	4
W	5
E	6
R	D
A	7
S	8
D	9
F	E
Z	A
X	0
C	B
V	F

Arrow keys can be used to move around, depending on the program you're running.
How the Emulator Works
The emulator follows the fetch-decode-execute cycle for the CHIP-8 processor:

Fetch: The emulator fetches the next 16-bit instruction from memory using the program counter (pc).

Decode: The instruction is decoded to determine what operation needs to be performed. CHIP-8 instructions are 2 bytes (16 bits) long.

Execute: Based on the decoded instruction, the corresponding action is executed. This may involve modifying registers, memory, or screen output.
The emulator also handles input from the keypad, updates the timers (dt and st), and refreshes the screen accordingly.

Project Structure
The project is structured as follows:

chip8-emulator/
├── src/
│   ├── main.rs           # Entry point for the emulator
│   ├── emulator.rs       # CHIP-8 emulator logic (Emu struct)
│   └── utils.rs          # Helper functions (optional)
├── Cargo.toml            # Rust project configuration
└── README.md             # This file

emulator.rs
Contains the Emu struct, which represents the CHIP-8 system. The struct includes the program counter, registers, memory, stack, and methods for fetching, decoding, and executing instructions.

main.rs
The entry point of the program. It initializes the emulator, loads a CHIP-8 program, and starts the emulation cycle.

Contributing
If you’d like to contribute to this project, feel free to submit a pull request with any improvements or bug fixes. Before submitting, please ensure that:

The code is properly formatted (you can use cargo fmt).
The code is well-documented.
Any new features are tested.
License
This project is licensed under the MIT License - see the LICENSE file for details.
