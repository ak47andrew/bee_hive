# DEVLOG 0002: Using custom brainF runner bc it's easier

**Developer:** ak47andrew
**Date:** 25.09.2025 (Thu)<br>
**Mood:** Calm and focused, yet a bit tired<br>
**Writing start time:** 21:30<br>
**Writing end time:** *Oopsie, forgot to add xD*<br>
**Current song playing:** ["Good Riddance" by Ellie Minibot, Shoomimi](https://music.yandex.ru/track/142549342?utm_source=desktop&utm_medium=copy_link)
(x6 or something like that)

Okay... I have 5 days left, and my current concern is about stack and heap manipulations really. It's actually super
hard to think about and create a cohesive algorithm for. So just to make things a bit easier, I'll customize brainfuck
interpreter/runner a bit further than I previously thought. Here's what I've got:
1) Stripe is infinite in both directions. I'm not yet sure how to implement this properly, especially if porting to C 
for microcontrollers, but I'll figure this one out
2) After the program ends, it'll return value from cell with index 2 (being the status cell) as a termination code of a
process
3) `,` is no longer writes into the current cell, it writes into a cell with index 0 (being the stdin cell). All the 
inputs are buffered for one value so if user pressed letters "a", "b" and "c" it would look like this:
`[ord(a), ord(b), ord(c)]` and then each time you use `,` first pressed button will be pushed to the zeroth cell 
rewriting what was there. If no value ready - it'll just drop a zero there
4) More status cells! Index 3 is a "Stdout format flag" and index 4 is "Stdin readiness flag"
   1) Index 3 will determine weather output a value from index 1 (stdout cell) as an ASCII index (0) or just output a 
number (1) and will be set by an interpreter
   2) Index 4 will be set by a program and 0 if input buffer isn't empty and 1 otherwise. Like this it's super easy to 
halt while waiting for an input! Just `[]` that's it! It also can't be set by a program! I tried, it raised 
YouHaveAMemoryLeakDumbassException xD So yeah it could prevent values from overflowing just in case
5) Oh, and final thing! Moving points! To not fuck around with format, you can jump to index 0 cell by using `>!`. `!`
doesn't do anything on its own, but with combination with `>` it sends you straight to the stdin value, and then you'll 
figure out what to do from that

Now, after talking to my Chinese friend D.S. I feel a bit less, but it still feels like I'm cheating on my old wife, 
with wife being the original idea of transpiling into more conventional brainF, yet I think I just hit the roof of the
kindergarten and I need to grow taller to do anything useful. That's my point on that whole situation

P.S. This new BrainF interpreter is called NJ (No jokes)! Fun story, but I have no time today so I'll tell it to you
some other time :)