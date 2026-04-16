 ./TASCII  --help
Executable located in target/build
My Rust practice and your cli tool to convert images into ACII text!

Usage: TASCII [OPTIONS] <FILE_NAME>

Arguments:
  <FILE_NAME>  path or name of the image file

Options:
  -c, --color            Output is greyscale by default but you can add color using this flag!
  -i, --inverted         inverts ASCII ramp sequence to make white spots sparse and black spots dense
  -q, --quality          
  -x, --width <WIDTH>    width of output if not present, scales according to height if present, set to 200 if not
  -y, --height <HEIGHT>  height of outputif not present, scales according to width if present, set to 200 if not
  -o, --output <OUTPUT>  output file name
  -h, --help             Print help
  -V, --version          Print version
