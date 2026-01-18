#!/usr/bin/python
#
# Custom Brainfuck Interpreter (Modified per DEVLOG-0002)
# Based on original by Sebastian Kaspari
# Modifications by: Reborn
#
# Usage: ./brainfuck.py [FILE]

import sys
import threading
import queue
import time

try:
    from msvcrt import getch # type: ignore
except ImportError:
    # Fallback for Unix-like systems
    def getch():
        import tty, termios
        fd = sys.stdin.fileno()
        old_settings = termios.tcgetattr(fd) # type: ignore
        try:
            tty.setraw(sys.stdin.fileno()) # type: ignore
            ch = sys.stdin.read(1)
        finally:
            termios.tcsetattr(fd, termios.TCSADRAIN, old_settings) # type: ignore
        return ch


class InputThread(threading.Thread):
    def __init__(self, input_queue: queue.Queue[int]):
        super().__init__()
        self.input_queue = input_queue
        self.daemon = True
        self.running = True
        self.is_ctrl_c = False

    def run(self):
        while self.running:
            try:
                # Get character (blocking)
                char = getch()
                if char in [b'\x03', b'\x1a']:
                    self.running = False
                    self.is_ctrl_c = True
                if char:
                    self.input_queue.put(ord(char))
                # Small delay to prevent CPU spinning
                time.sleep(0.01)
            except Exception:
                break

    def stop(self):
        self.running = False


class NJBrainfuckInterpreter:
    def __init__(self, code: str, no_input_thread: bool = False):
        self.cells = {i: 0 for i in range(-1000, 1001)}
        self.special_cell = 0
        self.check_special = False
        self.input_queue: queue.Queue[int] = queue.Queue()
        if not no_input_thread:
            self.input_thread = InputThread(self.input_queue)
        else:
            self.input_thread = None
        self.codeptr: int = 0
        self.cellptr: int = 0
        self.code = code
        self.cleanup()
        self.bracemap = self.buildbracemap()

        # Initialize special cells
        self.cells[3] = 0  # Stdout format flag (0=ASCII, 1=number)
        self.update_input_status()

        # Start input thread
        if self.input_thread is not None:
            self.input_thread.start()

    def update_input_status(self):
        """Update stdin readiness flag (cell 4)"""
        self.cells[4] = 1 if self.input_queue.empty() else 0

    def execute(self):
        try:
            return self.evaluate()
        except Exception as e:
            print(f"Error: {e}")
            return 1
        finally:
            if self.input_thread is not None:
                self.input_thread.stop()

    def evaluate(self):
        while self.codeptr < len(self.code):
            self.update_input_status()
            if self.input_thread is not None:
                if not self.input_thread.running:
                    print("Input thread terminated, shutting down")
                    if self.input_thread.is_ctrl_c:
                        print("Process was terminated via Ctrl + C or Ctrl + Z")
                        self.cells[2] = 130
                    break

            self.step()
        
        # Return termination code from cell 2
        return self.cells[2]
    
    def step(self):
        command = self.code[self.codeptr]

        # Handle multi-character commands first
        if command == ">!":
            self.cellptr = 0  # Jump to stdin cell
        
        if command == "+!":
            self.special_cell += 1  # Increment special cell
        if command == "-!":
            self.special_cell -= 1  # Decrement special cell

        # Standard commands
        elif command == ">":
            self.cellptr += 1
            # Extend cells dictionary if needed
            if self.cellptr not in self.cells:
                self.cells[self.cellptr] = 0

        elif command == "<":
            self.cellptr -= 1
            # Extend cells dictionary if needed
            if self.cellptr not in self.cells:
                self.cells[self.cellptr] = 0

        elif command == "+":
            if self.cellptr != 4:  # Protect stdin status cell
                self.cells[self.cellptr] = (self.cells[self.cellptr] + 1) % 256

        elif command == "-":
            if self.cellptr != 4:  # Protect stdin status cell
                self.cells[self.cellptr] = (self.cells[self.cellptr] - 1) % 256

        elif command == "[" and ((self.cells[self.cellptr] == 0 and not self.check_special) or (self.special_cell == 0 and self.check_special)):
            self.codeptr = self.bracemap[self.codeptr]

        elif command == "]" and ((self.cells[self.cellptr] != 0 and not self.check_special) or (self.special_cell != 0 and self.check_special)):
            self.codeptr = self.bracemap[self.codeptr]

        elif command == ".":
            # Output from stdout cell (index 1) based on format flag (index 3)
            if self.cells[3] == 0:  # ASCII output
                char = self.cells[1]
                if 0 <= char <= 255:
                    sys.stdout.write(chr(char))
                else:
                    sys.stdout.write(str(char))
            else:  # Numeric output
                sys.stdout.write(str(self.cells[1]))
            sys.stdout.flush()

        elif command == ",":
            # Read into stdin cell (index 0) from input queue
            try:
                # Non-blocking get from queue
                char_value = self.input_queue.get_nowait()
                self.cells[0] = char_value
            except queue.Empty:
                # No input available
                self.cells[0] = 0

            self.update_input_status()

        
        self.check_special = command.endswith("!") and command != ">!"
        
        self.codeptr += 1

    def cleanup(self):
        # Step 1: Cleanup
        cleaned: list[str] = list(filter(lambda x: x in ['.', ',', '[', ']', '<', '>', '+', '-', '!', "#", "$"], self.code))
        self.code = cleaned

        cleaned: list[str] = []
        i = 0
        while i < len(self.code):
            if self.code[i] in ['.', ',', '[', ']', '<', '>', '+', '-', '!', "#", "$"]:
                # Handle the special case of ">!" as a single unit
                if (i + 1 < len(self.code) and self.code[i] in [">", "+", "-", "#"] and self.code[i + 1] == "!"):
                    cleaned.append(self.code[i] + "!")
                    i += 1
                else:
                    cleaned.append(self.code[i])
            i += 1
        self.code = cleaned

    def buildbracemap(self):
        temp_bracestack: list[int] = []
        bracemap: dict[int, int] = {}

        for position, command in enumerate(self.code):
            if command == "[":
                temp_bracestack.append(position)
            elif command == "]":
                if not temp_bracestack:
                    raise SyntaxError("Unmatched ']'")
                start = temp_bracestack.pop()
                bracemap[start] = position
                bracemap[position] = start

        if temp_bracestack:
            raise SyntaxError("Unmatched '['")

        return bracemap
    
    def show_debug_step(self):
        command = self.code[self.codeptr - 1]
        """Display debug information about the current execution state"""
        print("\n" + "="*50)
        print(f"DEBUG: Executed command '{command}'")
        print(f"Code pointer: {self.codeptr}, Cell pointer: {self.cellptr}")
        
        # Show current cell and neighbors
        print("\n--- Memory Cells ---")
        start = max(-1000, self.cellptr - 2)
        end = min(1000, self.cellptr + 2)
        
        for i in range(start, end + 1):
            pointer_indicator = " <--" if i == self.cellptr else ""
            print(f"  [{i}]: {self.cells[i]}{pointer_indicator}")
        
        # Show special cells with their meanings
        print("\n--- Special Cells ---")
        special_cells = {
            0: "Stdin buffer",
            1: "Stdout buffer", 
            2: "Exit code",
            3: f"Stdout format ({'ASCII' if self.cells[3] == 0 else 'Numeric'})",
            4: f"Stdin status ({'ready' if self.cells[4] == 1 else 'not ready'})"
        }
        
        for addr, description in special_cells.items():
            pointer_indicator = " <--" if addr == self.cellptr else ""
            print(f"  [{addr}]: {self.cells[addr]} - {description}{pointer_indicator}")
        print(f"Special cell: {self.special_cell}")
        print(f"Cell check: {self.check_special}")
        
        print("="*50)
        print("\nPress Enter to step...")
        print("Press Ctrl + C to terminate the program")
        print("Enter `run` to run the program to the breakpoint character ($)")

def debug(interpreter: NJBrainfuckInterpreter):
    is_running = False
    while interpreter.codeptr < len(interpreter.code):
        interpreter.step()
        print(interpreter.code[interpreter.codeptr - 1])
        if interpreter.code[interpreter.codeptr - 1] == "$":
            is_running = False
        if is_running:
            continue
        # system("cls")
        interpreter.show_debug_step()
        try:
            inp = input(">>>")
            if inp == "run":
                is_running = True
        except KeyboardInterrupt:
            break

def main():
    if "--debug" in sys.argv:
        sys.argv.remove("--debug")
        debug_mode = True
    else:
        debug_mode = False
    
    if len(sys.argv) == 2:
        with open(sys.argv[1], "rb") as f:
            interpreter = NJBrainfuckInterpreter(bytes(b for b in f.read() if b in b".,[]<>+-!#$").decode('ascii'), debug_mode)
        
        if debug_mode:
            debug(interpreter)
        else:
            exit_code = interpreter.execute()
            sys.exit(exit_code)
    else:
        print("Usage:", sys.argv[0], "<filename> [--debug]")
        sys.exit(1)


if __name__ == "__main__":
    main()