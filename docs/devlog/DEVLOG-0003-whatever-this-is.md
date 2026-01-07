# DEVLOG 0003: Idk, I just wanna yap about some things :P

**Date:** 18.10.2025 (Saturday night)<br>
**Mood:** Quite playfull, yet I bit sleepy<br>
**Writing start time:** 02:18<br>
**Writing end time:** 02:50<br>
**Current song playing:** Going between [Brad Williams' - Daddy Issues standup](https://youtu.be/K8Afn6ChrBY?si=DbaWEGu4sMWHsDco) and [Nightcore version of "Chris Brenner - Strip That Down (cover)"](https://www.youtube.com/watch?v=BSeJcvQjB28) in equal proportions

Yahoo! I just got 3 weeks off the initial deadline xD

Well, I guess shit happens. Yet by that time I created, fixed and refixed automatically generated animations of translation process and nj execution. Also, I feel kinda burned out of this project tbh. This just doesn't turn the same fire that was there in the first two devlogs. Probably because I was working on animations, and it's not as interesting as writing the translator's code itself, but who knows. For today, I'll just write this devlog, commit & push and tomorrow... I don't even know. I probably will program some features, yet I'm not sure if I'll have enough willpower and other things like that. If I will - great! If not - probably not a big deal anyway

About further development I have a few thoughts: optimization, strings or input

1. Strings
   That's the easiest one: add full support for strings and numbers higher than 255 aka do some encoding stuff. I already have this in past commits! I just need to pull it up and correctly setup to work with this "variable size" style. And that means writing a lot of brainfuck by hand... That quite a pain in the ass tbh.
2. Input
   Add `input_char` function to create blocking char input from a user. Because it's a single char and cause we can't do much about it at the moment other than output - I can do it without much trouble or extra work. This feels like low-hanging fruit 
3. Optimization

   I have a few ideas of how to split the opcodes a bit further and make them indeed undivisable. Let me demonstrate what I have in mind

   Here's what IL looks like right now for code, for example, `print("1", "2");`:
   ```
   LOAD_IMMEDIATE "1"
   PUT
   LOAD_IMMEDIATE " "
   PUT
   LOAD_IMMEDIATE "2"
   PRINT
   ```
   At first, we can split it into something like this:
   ```
   MOVE_TO_EMPTY_STACK
   LOAD_IMMEDIATE_RELATIVE "1"
   PUT
   MOVE_TO_EMPTY_STACK
   LOAD_IMMEDIATE_RELATIVE " "
   PUT
   MOVE_TO_EMPTY_STACK
   LOAD_IMMEDIATE_RELATIVE "2"
   PRINT
   ```
   and then this being optimized into something like this:
   ```
   MOVE_TO_EMPTY_STACK
   LOAD_IMMEDIATE_RELATIVE "1"
   LOAD_IMMEDIATE_RELATIVE " "
   LOAD_IMMEDIATE_RELATIVE "2"
   PRINT_ALL
   ```
   or even
   ```
   MOVE_TO_EMPTY_STACK
   LOAD_IMMEDIATE_RELATIVE_ALL "1 2"
   PRINT_ALL
   ```

   Here `MOVE_TO_EMPTY_STACK` finds the first empty cell at the stack that we're ready to write into, `LOAD_IMMEDIATE_RELATIVE` sets current cell (no check) to the specified value while `LOAD_IMMEDIATE_RELATIVE_ALL` does the same, but optimized to set a lot of values at the same time, `PRINT_ALL` or `PUT_ALL` just prints/puts the whole stack instead of a single value in it

Well, I'm thinking about any of this three directions with each having different level of impact on the project and my own enjoyment. And until that - see you later ^^

P.S. Don't forget to drop animation examples to Kseniya tomorrow morning, she's waiting for the three weeks now xD