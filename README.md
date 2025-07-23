# rmosaic

Randomly split parts of an image recursively, making a random
mosaic-like image.

# usage

```
Usage: rmosaic [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  input file for mosaic'ing

Options:
  -o, --output-file <OUTPUT_FILE>  output file name [default: output.png]
  -c, --chance <CHANCE>            compounded chance of continuing to split [default: 0.5]
      --width-min <WIDTH_MIN>      minimum width of mosaic piece [default: 5]
      --height-min <HEIGHT_MIN>    minimum height of mosaic piece [default: 5]
  -h, --help                       Print help
```
