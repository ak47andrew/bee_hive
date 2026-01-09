# DEVLOG 0001: Global rewrite or "Why first idea you have is probably the worst"

**Developer:** ak47andrew
**Date:** 20.09.2025 (Sat)<br>
**Mood:** Calm, obsessed with organizing everything xD<br>
**Writing start time:** 0:21<br>
**Writing end time:** 0:51<br>
**Current song playing:** ["Serenade To A Dream" by Suidakra](https://music.yandex.ru/track/5951383?utm_source=desktop&utm_medium=copy_link)

---

Okay, it's midnight so it's time for my usual "Let's create perfect structure for my horseshit" so that's pretty much 
why I'm currently writing this devlog and why there's THAT many folders after this commit.

### Here's pretty much what happened:

It was about a few days ago when I started thinking about adding math operations to the language. Initially I only had
problems with parsing (regex is a string tool, but sometimes can be annoying as fuck), but then I stumbled into more
serious problem: compile-time memory management vs runtime memory management. The thing is that in previous implementation
almost everything was determined at compiling up to the specific values so when, in the future, I would add runtime based
values (for example user input or random values) - it would be pretty difficult. Looking back I, potentially, could add
some "dynamic" data type, but that's just the same thing. So I was going around in circles before I found solution!

### Solution and new architecture

The thing is that we have infinite strip, right? And we have it infinite in both directions, right? Why can't we use
values less than zero as stack and zero and forward as a heap! That's pretty classic stuff: you use heap for variables, 
and you use stack for instructions! 

Also instead of compiling directly into brainfuck I'll do a little trick: intermediate language. So I pretty much use 
Python's approach but taking it even further in terms of simplicity. Let me show you what I'm talking about.

Here's I have a simple python program:
```python
x = 10
y = 20
print(x + y)
```

[Compiler explorer](https://godbolt.org/) shows that it returns result something like this:
```
0         RESUME                   0

1         LOAD_CONST               0 (10)
          STORE_NAME               0 (x)

2         LOAD_CONST               1 (20)
          STORE_NAME               1 (y)

3         LOAD_NAME                2 (print)
          PUSH_NULL
          LOAD_NAME                0 (x)
          LOAD_NAME                1 (y)
          BINARY_OP                0 (+)
          CALL                     1
          POP_TOP
          RETURN_CONST             2 (None)
```

So it loads constant `10` and stores it as `x`, then loads `print` function and new variable and calls it with that 
variable as an argument

I have pretty similar idea. Let's say we have the same program in BEE:
```
var x = 10;
var y = 20;
print(x + y);
```

In my head this would return something like that:
```
LOAD_IMMIDIATE 10
STORE x

LOAD_IMMIDIATE 10
STORE y

LOAD_VARIABLE x
LOAD_VARIABLE y
ADD
MOVE_TO_STDOUT
PRINT

SET_ERROR 0  # sucsess
```

You would also need to convert this number into a string and then byte by byte move to stdout... Blah-blah-blah, you got 
the point! Oh, and I forgot important detail. Cells with indexes 0, 1 and 2 is reserved for stdout, stdin and error code 
respectively

And that's pretty much the core two ideas that I have in mind now. Probably there will be endlessly many devlogs where I
come up with something new along the way, but that's not that important now. The thing is that I have working foundation,
and I'll build up from it! ... Probably ... I hope ... ... ... Woah, woah, woah, leave this earthquake away from my 
little boy, okay!?

*P. S. when writing the last paragraph instead of `foundation` I accidentally pasted and got `I have working LOAD_VARIABLE x`.
For some reason I think it's funny*