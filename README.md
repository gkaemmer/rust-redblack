## Red-Black tree in Rust

Mostly built for fun, this is a reasonably performant implementation of a RB-tree that exposes the following methods:

- `tree.insert(value: T)`: Inserts a new value into the tree
- `tree.delete(value: T)`: Deletes a value from the tree if it exists
- `tree.predecessor(value: T)`: Gets the predecessor to a value in the tree, if it exists
- `tree.successor(value: T)`: Gets the successor to a value in the tree, if it exists
- `tree.print(to_string: F)`: Given a function that converts `T` to a `String`, the `print` method prints a graphical representation of the tree, useful for debugging, like this:

```
               .------------B:27-----------.
       .----B:23---.                   .----B:31-----------.
    B:21            B:25            B:29               .----B:35---.
B:20    B:22    B:24    B:26    B:28    B:30        R:33            R:37
                                                B:32    B:34    B:36    B:38
```
