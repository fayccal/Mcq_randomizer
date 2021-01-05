# QcmRandomizer

program which create multiple randomized mcq from a text file with a minimal syntax.
(changes may come).

## Used libs
- rand
- anyhow
- pdf_canvas

## How to use it
you will  have to give a txt file to the program with a small syntax to use it:

question on the first line
answers:-rep 1 -rep2 ...

you will have to use the keyword "answers:" and every answers will need a "-"

DISCLAIMER:Do not use the keyword and "-" in your answers which will give you broken questions,
if you need it, it is very simple to modify the syntax which is up to you.
I will may make further changes to simplfy or remove the syntax restrictions in the future.

## Upgrade 

- minimal interface to choose a file, how much mcq you want to create.
- squares to make checked answers.
- bigger interface to make the questions and answers with it.