# Reading colors
You might be a bit confused on how to parse colors that are laid out like this: `-1.6740915E7`. If you're on java with the android framework, you could directly feed it into `Color` and you'll get the color directly. But for those who don't have the luxury, they will need to parse the color themselves.

It really is very simple, if you've seen an HTML color code before, these colors are structured like that. The difference is that HTML color codes are coded as __hex__, but sketchware stores it as an __integer__.

When you transform the integer that I've mentioned above into hex, you basically will get a valid HTML color code!

Example:
 - Read the color: `-1.6740915E7`
 - Evaluate the E: `-1.6740915E7` -> `-16743230.0`
 - Turn it into an integer: `-16743230.0` -> `-16743230`
 - Transform it into hex: `-16743230` -> `ff008dcd`
 - Finish up: `ff008dcd` -> `#ff08dcd` (valid html color code)

> If you're testing this on python with `hex()`, you might get a very weird result: `hex(int(-1.674323E7)) -> -0xff7b3e`. I honestly don't know why python does this, but seeing from this [stackoverflow question](https://stackoverflow.com/questions/3831833/printing-negative-values-as-hex-in-python), you need to force python to use a 32bit integer and then turn it into hex back: `hex(int(-1.674323E7) & 0xffffffff) -> 0xff0084c2`

What an HTML color code looks like:

| hex    | `ff`         | `00` | `8d`  | `cd` |
| ------ | ------------ | ---- | ----- | ---- |
|  int   | 255          | 0    | 141   | 205  |
|  type  | Transparency | Red  | Green | Blue |

There are 2 possible ways to separate the red, green, and blue values from a color.
 - A simple approach would be to transform the color (int) into hex and into string. Then substring the red, green, and blue parts, and finally turn them back into integers, and you'll then have the color!
 - A bit more advanced technique is by [bit masking & shifting](https://en.wikipedia.org/wiki/Bitwise_operation).
   
   To get the red color, first we will need to shift the bits to the right until we get the red bits in the start (which is 16, we're shifting the blue (8 bits), and green (8 bits) out) and then to get rid of the transparency bits on the end, we mask the value by 8 bits so we **only** have the red color in-place.
 
   And boom you've done it, now do this two more times to read the blue and green by shifting to them with different values (0 for green, 8 for blue, + 24 for transparency if you actually needed it).
   
   <details>
     <summary>Interactive explanation</summary>
 
     Get the color:
     ```
     hex: ffe61975
     bin: 11111111111001100001100101110101
     ```    
   
     Target: Read the red color

     Shift to the left by 16
     ```
     hex: ffe61975 -> ffe6
     bin: 11111111111001100001100101110101 -> 1111111111100110
     ```

     Mask to only get the first 8bits (perform an AND operation by 0b11111111 or 0xff)
     ```
     hex: ffe6 -> e6
     bin: 1111111111100110 -> 11100110
     ```
       
     And you got the red color! (`0xe6` or `0b11100110`)
   </details>
