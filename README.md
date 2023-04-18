# cli

A small command that allows you to use gpt-3 to generate cli commands similar to how warp's terminal works.

**Example:**

```sh
$ cli find all files that start with the letter q in this current directory and exclude node_modules
find . -type f -name 'q*' ! -path './node_modules/*'
```
