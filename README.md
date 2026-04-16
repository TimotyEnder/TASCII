 ./TASCII  --help\n

My Rust practice and your cli tool to convert images into ACII text!\n

Usage: TASCII [OPTIONS] <FILE_NAME>\n

Arguments:\n
  <FILE_NAME>  path or name of the image file\n

Options:\n
  -c, --color            Output is greyscale by default but you can add color using this flag!\n
  -i, --inverted         inverts ASCII ramp sequence to make white spots sparse and black spots dense\n
  -q, --quality          \n
  -x, --width <WIDTH>    width of output if not present, scales according to height if present, set to 200 if not\n
  -y, --height <HEIGHT>  height of outputif not present, scales according to width if present, set to 200 if not\n
  -o, --output <OUTPUT>  output file name\n
  -h, --help             Print help\n
  -V, --version          Print version \n
Executable located in target/build
