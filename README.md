# aap

An aap (Dutch for monkey I think) interpreter in Go

## Prerequisite

Install Go

## Getting started

Clone the repo and cd into the repo

```bash
  git clone https://github.com/mrdvince/aap.git
  cd aap
```

## Run

This runs the REPL included

e.g REPL

```bash
➜  aap git:(master) ✗ go run .
Hello vince! Welcome to Aab lang!
Feel free to start typing stuff
>>let add = fn(x, y) { x + y; };
{Type:LET Literal:let}
{Type:IDENT Literal:add}
{Type:= Literal:=}
{Type:FUNCTION Literal:fn}
{Type:( Literal:(}
{Type:IDENT Literal:x}
{Type:, Literal:,}
{Type:IDENT Literal:y}
{Type:) Literal:)}
{Type:{ Literal:{}
{Type:IDENT Literal:x}
{Type:+ Literal:+}
{Type:IDENT Literal:y}
{Type:; Literal:;}
{Type:} Literal:}}
{Type:; Literal:;}
>>

```
