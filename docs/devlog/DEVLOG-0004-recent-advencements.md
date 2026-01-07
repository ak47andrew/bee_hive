# DEVLOG 0004: Whatever tf happened

- **Date:** 01.12.2025 (Monday, in the middle of the 5th class)
- **Mood:** Super hyped
- **Writing start time:** 13:35
- **Writing end time:** *TODO: DON'T FORGET TO ADD*
- **Current song playing:** My Year in Review playlist

Okay, okay, before we start let me do a bit of an offtop
```
OH MY FUCKING GOD THIS SUMMARY IS SO DAMN GOOOOODDDD!
```
I just got year's summary from Yandex music and I just hype every fucking time, sorry. Also, my total listening time this year is 2 months... Yeah, I feel like I'm addicted to music xD

Oh, and I also completed Dispatch this weekend, and it's also fucking amazing, the best 9 hours of my life.

Anyway let's come back to the code because this is also pretty fucking good.

## IL optimization

First of all, I wanted to optimize IL a bit.

For example if we have something like this:
```
print("asd")
```

Initially the IL was like this:
```
LOAD_IMMIDIATE "a"
PUT
LOAD_IMMIDIATE "s"
PUT
LOAD_IMMIDIATE "d"
PRINT
```

Which is... far from perfect. So I decided to change it around a bit (It reversed because it pops it from the top of the stack):
```
LOAD_IMMIDIATE "\n"
LOAD_IMMIDIATE "d"
LOAD_IMMIDIATE "s"
LOAD_IMMIDIATE "a"
PRINT_ALL
```

## The problem

But here we stumbled into a little problem. Because PRINT operated on a single loaded value it worked perfectly. But because of specific NJ code this IL generated. The thing is. Let's say that we have this tape:
```
                   IN  OUT
[0] [32] [10] [32] [0] [0]
```

And after it's wanted to move value of the first cell with this NJ code: `>!<[<]>[->!>+<<[<]>]`. See the problem? Let's say we already moved the first 32 into an out cell:
```
                   IN  OUT
[0] [0] [10] [32] [0] [32]
```

But then `<<[<]>` moves to the next non-zero cell being 10! And it continues until it just adds up all the values on the stack.


## The solution

And there was no real solution! Well, yeah, I could use the `(0a0)(0b0)` solution, but it felt like I'm just moving the wall a few feet away that I'll need to fix later. Sooo came the best idea I've ever had in my entire life (btw starting this project was the worst one xD)

Yet I still needed a solution, so I started to brainstorm... (I would say if I was someone else. Me myself just got into a deep frustration). And then, out off nowhere, like in some cheap book, idea came to me: extra cell! Just a single one, but special: the one that can be accessed from anywhere on the tape. And, like this, four more commands added to the NJ: +!, -!, # and #!. First two increment and decrement value of this special cell, but next one... They're NOPs, but can be used to show what cell should loop `[]` refer to. This fixed everything! [(with a cheery on top)](#extra-cream).

Like this, we have new idea of how to print a single cell without [this bug](#the-problem):
- Cleanup the output cell: `>!>[-]`
- Cleanup the format cell: `>!>>>[-]`
- Move the pointer to the top of the stack: `>!<[<]>`
- Move the value to the special cell: `[+!-]`
- Move the value from the special cell to the output cell: `>!>#![+-!]`
- Output the value: `.`

## Extra cream

But this special cell give us so much more! Now I can much more easily move in my yet-to-come system `(0a0)(0b0)`: just `<[<]` and then decrement the special cell's value until we're at the correct spot

Sooo that's pretty much the devlog. All that left is to finish up the variables (to actually do something with the heap xD), split source code to statements more easily, fix newlines in strings..... Yeahh... It's a lot actually....