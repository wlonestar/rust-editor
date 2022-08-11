# Part 2: Low-level terminal input and output handling.

## 1. refactor keyboard input

create `Reader` struct to read various keypress

## 2. main master mind for project

1. create `Editor` struct to run program

2. write fn to clear the screen

## 3. process output

1. create `Writer` struct to process output

2. reposition the cursor

3. create `EditorContents` struct as append buffer

4. add welcome message

5. create `CursorController` to move cursor position

6. process arrow keys and PageUp, PageDown, Home and End keys

7. fix out of bounds error
