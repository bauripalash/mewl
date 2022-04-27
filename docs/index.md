# ![](../media/mewl_banner.png)

# Mewl Language

> An esoteric language which lets you program in cats' language.

## Introduction

### 🤔 What,Why?

Well, 2 years ago in 2020, I created a esoteric programming language called [Mewmew](https://github.com/bauripalash/mewmew). It was just a fun and experimental language I made while tinkering with ANTLR, cause I was (am still) fascinated about programming languages, interpreter/compilers; how they work; how they basically become the medium of interaction between humans and computers and/or machines. But with ANTLR, I didn't actually learn much about compiler/interpreter construction. It just worked! No fun!

So, In the October of 2021, I started re-writing the whole thing from scratch with Rust. I was able to build a lexer; a basic parser; But then I realized, syntax of 'mewmew' is ugly. I hurts my eyes to look at a 'mewmew' source code.

Again in, March 2022, I started experimenting with mewmew lanugage but with lisp style syntax, 'cause lisp is awesome. So, here is the result, **Mewl** , mewmew reborn with the syntax similar to lisp. Most important thing, I am having fun while developing mewl, and the syntax doesn't hurt my eyes that much. 

## Basics

### Numbers

There is no concept of traditional numbers in mewl instead we use `mew`s.
For example you want to write `5` you could do something like this

```lisp
mewmewmewmewmew
```

or maybe 2

```lisp
mewmew
```

So basically 1 `mew` is equivalent to 1; 2 `mew`s is equivalent to 2 and so on.

* Note: We don't have zero, to get zero we use subtraction `[- mew mew]` (it's like `1-1` = 0) [Mathematical operations will be discussed later]

But what about large numbers like 100 , 2022 , 500 or maybe `-10` , `-3.1`, for that we have some shortcuts, like these :
To write large numbers we can use mathematics, like `[* mewmew mewmewmewmewmew]` is equal to 10, or multiplying that expression with itself we can get 100. But that's cumbersome and complicated! Instead we have a simpler (relatively) syntax shortcut.

Lets write 100:

```lisp
[mew [- mew mew] [- mew mew]]
```

How about 2022:

```lisp
[mewmew [- mew mew] mewmew mewmew]
```
