# About voctrainer
This programm is a simple lerning tool for vocabulary. I initially made this to study old Norse and to learn Rust, so the languages (Old Norse <-> German) are kinda hard-coded. Maybe I will add multi-language support in the future.

## How to use it
Download the source code and compile it with your local Rust compiler.

In the source code you will find an SQLite Database file (vocabs.db) which contains all vocabulary and the number of times you got a word pair right. If you want to adjust it just use any Database editing tool, e.g. DBeaver. The source file vocabs.txt gets read everytime the programm is opened and all vocabs that are not in the Database will get added.

## ToDo
- Multi-Language Support
- Functionality:
  - Read vocabulary from file
  - Reset vocabulary answers
