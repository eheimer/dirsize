# DirSize

DirSize is a Rust rewrite of a bash script that lists the contents of a specified directory along with their sizes.

The original bash script is included in the `reference` directory.

## Rust

This is a personal exercise to begin learning Rust. Much of the code was generated with Copilot, tested and confirmed by me to be working as well or better than the original bash script on my linux machine.

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

The output will display the size of each file and directory, sorted by increasing size. Directories will be highlighted in blue.

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
- Symbolic links are ignored to speed up the execution.

## Dependencies

- [termcolor](https://crates.io/crates/termcolor) for colored terminal output.

## License

This project is licensed under the MIT License.
