# DirSize

DirSize is a Rust utility that lists the contents of a specified directory along with their sizes. If the entry is a directory, it lists the total size of the directory. The output is sorted by increasing size. It is meant to be compiled and run from the command line to list directory contents and sizes. I use it to keep an eye on hard-disk space.

## Usage

To use DirSize, run the following command:

```
cargo run -- <directory_path>
```

Once compiled, you can just run:

```
dirsize <directory_path>
```

Full or relative path can be used.

If no directory path is specified, the current directory (`.`) will be used.

### Example

```
dirsize /path/to/directory
```

## Output

The output will display the size of each file and directory in the specified directory, sorted by increasing size. Directories will be highlighted in blue.

### Example Output

```
Checking /path/to/directory/
1K      file1.txt
2K      file2.txt
3K      file3.txt
4K      subdir1
5K      subdir2
Total: 15K
```

## Notes

- This utility has only been tested on Linux so far.
- Symbolic links are ignored to avoid "too many levels of symbolic links" errors.

## Dependencies

- [termcolor](https://crates.io/crates/termcolor) for colored terminal output.

## License

This project is licensed under the MIT License.
