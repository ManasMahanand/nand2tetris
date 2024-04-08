# Nand2Tetris

This is my work done for [nand2tetris](https://www.nand2tetris.org/).

The purpose of the course is to build a full computer from scratch using Nand gates. This is my work done for the same.

My current progress:

- [x] Project 1: Boolean Logic (Build basic chipsets for the computer)
- [x] Project 2: Boolean Arithmetic (Build Arithmetic chipsets including the ALU)
- [x] Project 3: Memory (Build the RAM chip)
- [x] Project 4: Machine Language (Understanding the specifications of Hack Machine language - the machine language for the built machine)
- [x] Project 5: Computer Architecture (Building a working Computer along with CPU and RAM)
- [ ] Project 6: Assembler - working on this currently :)
- [ ] Project 7: VM I: Stack Arithmetic
- [ ] Project 8: VM II: Program Control
- [ ] Project 9: High-Level Language
- [ ] Project 10: Compiler I: Syntax Analysis
- [ ] Project 11: Compiler II: Code Generation
- [ ] Project 12: Operating System

---
## How to try the computer for yourself:

1. Clone this git repo locally
2. Navigate to /tools
3. Run "HardwareSimulator" .bat for windows .sh for linux/macos
4. Click on the chip icon on the top left
5. Open "Computer.hdl" under /projects/05
6. You now are running my own working computer! But unfortunately the computer doesn't do anything unless you feed it machine code... tho you can feel free to write basic programs and test it if you're patient enough.

Also since the Computer is running on a simulated hardware written in Java, it's not going to do much

So instead do the following to see how the computer would run:

1. Follow Steps 1 and 2 from the previous list
2. Run "CPU Emulator" again .sh if linux/macos or .bat if on windows
3. Click on the folder icon next to "ROM" and navigate to projects/06/pong
4. Load pong.asm
5. Set "Animate" to "No animation" on the top menu
6. Set slider to "Fast"
7. Click on the blue double arrow
8. Click on the Keyboard Icon on the right and Enjoy playing Pong on the Virtual computer! (Left and Right arrows are the controls)

You may be asking why can't I run Pong on the computer I have built... unfortunately since the Hardware simulator is literally simulating every single binary operation in Java, it makes the process very slow taking up lot of real RAM.

But you can feel free to check every chipset I've built in HDL from project 01 to project 05, all of these work!
