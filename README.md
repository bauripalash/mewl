# ![](media/mewl_banner.png)

# Mewl

##### The programming language of cats' with the taste of lisp

## 🤔 What,Why?

Well, 2 years ago in 2020, I created a esoteric programming language called [Mewmew](https://github.com/bauripalash/mewmew). It was just a fun and experimental language I made while tinkering with ANTLR, cause I was (am still) fascinated about programming languages, interpreter/compilers; how they work; how they basically become the medium of interaction between humans and computers and/or machines. But with ANTLR, I didn't actually learn much about compiler/interpreter construction. It just worked! No fun!

So, In the October of 2021, I started re-writing the whole thing from scratch with Rust. I was able to build a lexer; a basic parser; But then I realized, syntax of 'mewmew' is ugly. I hurts my eyes to look at a 'mewmew' source code.

Again in, March 2022, I started experimenting with mewmew lanugage but with lisp style syntax, 'cause lisp is awesome. So, here is the result, **Mewl** , mewmew reborn with the syntax similar to lisp. Most important thing, I am having fun while developing mewl, and the syntax doesn't hurt my eyes that much. 

## 🎉 Get Started

### Read Documention

[The Mewl Book](https://bauripalash.github.io/mewlbook)

### Hello World

```lisp
[=mew [[+[* mewmewmew mewmew] mew] mewmew] ] //H
[=mewmew [mew [- mew mew] mew]] //e
[=mewmewmew [mew [- mew mew] [* mewmewmewmew mewmew]]] //l
[=mewmewmewmew [mew mew mew]] //o
[=mewmewmewmewmew [mewmewmew mewmew]] //SPACE
[=mewmewmewmewmewmew [[* mewmewmewmew mewmew] mewmewmewmewmewmewmew]] //W
[=mewmewmewmewmewmewmew [mew mew mewmewmewmew]] //r
[=mewmewmewmewmewmewmewmew [mew [- mew mew] [- mew mew]]] //d
[=mewmewmewmewmewmewmewmewmew [mewmewmew mewmewmew]] //!

[::: ~mew //H
~mewmew //e
~mewmewmew //l
~mewmewmew //l
~mewmewmewmew //o
~mewmewmewmewmew //SPACE 
~mewmewmewmewmewmew //W
~mewmewmewmew //o
~mewmewmewmewmewmewmew //r
~mewmewmew //l
~mewmewmewmewmewmewmewmew //d
~mewmewmewmewmewmewmewmewmew //!
]

```

### Basic Syntax

```lisp
;; mew is equivalent to 1
;; mewmew is equivalent to 2
;; mewmewmewmewmew is equivalent to 5
;; [- mew mew] is equivalent to 0


(:: [+ mew mew]) ;; prints 2
;; :: -> is a symbol/function to print the next atoms/expressions to stdout

For more, Read Docs



```

### Some operations

```lisp
+ -> Addition
- -> Substraction
* -> Multiplication
/ -> Division
:: -> Print to stdout
::: -> Assumes next expressions/atom as byte value. convert them to string, including invalid chars and prints to stdout


Learn more on Docs.
```



[![Test](https://github.com/bauripalash/mewl/actions/workflows/rust.yml/badge.svg)](https://github.com/bauripalash/mewl/actions/workflows/rust.yml)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fbauripalash%2Fmewl.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fbauripalash%2Fmewl?ref=badge_shield)


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fbauripalash%2Fmewl.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fbauripalash%2Fmewl?ref=badge_large)