# MDDO

MDDO is a simple todo list application that is based on markdown files. It is part specification and part application to better organize todos.

The purpose is mostly to learn rust, but for usage outside of the todo application you can edit the markdown files directly in another application. Or you can write another application to interface with the files as well.

## Specification

> **Note** This is an ever evolving spec

Todos are based on markdown files. The title of the file is the title of the todo. The rest of the file is the description of the todo. The file is stored in a directory that is the name of the project. The project directory is stored in the root directory of the todo list.

Parts of a todo

Title: <filename>.md
Status: x
Priority: A | B | C
Order: <number>
Creation: <date>
Due: <date>
tags: comma separated list
<empty line>
Description and details is the rest of the file in markdown

---

When you open up the application it will point to a folder that will have a bunch of folders with project names.

inside each project name will be .md files that are the todos.

To load a new item we need to pass the path.


file save

load files

todo item

save to file per item to the file

---

## Example Toml Config

```toml
data_location = "/home/<user>/Documents/mddo/"
```
