`v0.1.0 BETA`
# Official Edgecode Specification

Edgecode is a somewhat simple encoding format that is supposed to be hard to get behind for people who don't know about this Specification.
It is inspired by the [Bottom encoding format](https://github.com/bottom-software-foundation/spec/blob/main/README.md).

**DO NOT USE THIS TO ENCODE ANY IMPORTANT DATA! IT'S NOT SECURE AT ALL! THIS IS MAINLY MADE AS A JOKE!**

## Character Table
Each character in Edgecode maps to a specific value which will be referred to later.



### Character Values
|  Unicode Escape(s)  |    Character   |     Value    |
|:-------------------:|:--------------:|:------------:|
| `U+16D4`            | ᛔ              | Integer: 200 |
| `U+16E4`            | ᛤ              | Integer: 50  |
| `U+16B8`            | ᚸ              | Integer: 10  |
| `U+16BB`            | ᚻ              | Integer: 5   |
| `U+16A4`            | ᚤ              | Integer: 1   |
| `U+16D7`            | ᛗ              | Integer: 0   |


### Special Characters
|  Unicode Escape(s)  |    Character   |      Meaning     |
|:-------------------:|:--------------:|:----------------:|
| `U+26B8`            | ⚸              | Block Seperation |

-----

## Notes on Encoding
- The input stream must be valid UTF-8 encoded text. Encoding invalid UTF-8 is illegal!

- The output will be a sequence of blocks of characters which map to certain values ([see table above](https://github.com/SirShine/edgecoding-specification/blob/main/README.md#character-values)) seperated by the [Block Seperation](https://github.com/SirShine/edgecoding-specification/blob/main/README.md#character-values#special-characters) character, ie.

```
ᛤᛤᚸ⚸ᛤᛤᚸᚸᚸᚸᚻ⚸ᛤᛤᛤᚤᚤᚤᚤ⚸ᛤᛤᛤᚤᚤᚤᚤ⚸ᛤᛤᛤᚻᚤᚤ⚸ᚸᚸᚸᚸ⚸ᛤᛤᚸᚸᚻᚤᚤ⚸ᛤᛤᛤᚻᚤᚤ⚸ᛤᛤᛤᚸᚤᚤ⚸ᛤᛤᛤᚤᚤᚤᚤ⚸ᛤᛤᚸᚸᚸᚸᚤᚤᚤᚤ⚸ᚸᚸᚸᚸᚤ⚸
```

- The total numerical value of each block must add up to the decimal representation of the corresponding input character converted to base 8 and treated as base 10.

    - ie. the numerical value of `ᛤᛤᚸ`, according to the [table above](https://github.com/SirShine/edgecoding-specification/blob/main/README.md#character-values), is `50 + 50 + 10 = 110`, converted back to base 10 from base 8 would make it `72`, therefore it represents `U+0048` or `H` which has the decimal value of `72`.

    - Please note that value characters within blocks should be ordered in descending order! Not ordering them descendingly usually doesn't affect the output from decoders in any way, it can cause performance issues and depending on the decoder, might be read is invalid edgecode and therefore might not work with some decoders.

- Blocks of value characters must be followed by a [Block Seperator](https://github.com/SirShine/edgecoding-specification/blob/main/README.md#character-values#special-characters), `ᛤᛤᚸ` is illegal but `ᛤᛤᚸ⚸` is valid.

- The null value counts as a block and therefore must be followed by a [Block Seperator](https://github.com/SirShine/edgecoding-specification/blob/main/README.md#character-values#special-characters), `ᛤᛤᚸ⚸ᛗ` is illegal but `ᛤᛤᚸ⚸ᛗ⚸` is valid.

-----

## Notes on Decoding
- Decoding is quite simple, one way of doing it is to split the input string into groups of characters, then looping through all characters in each group, increasing a counter by the corresponding value, then converting the recieved value from base 8 to base 10, which then gives you the original decimal representation of said character. There can be examples found in this repository which I highly recommend looking through as they should show how the decoding process works.

-----
<sub><sup><sup>
(also, this is my first time writing something like this Specification, I will probably improve it later on.)
</sup></sup></sub>

<sub><sup><sup><sub><sup><sub><sup><sub><sup><sub><sup>
(I also referenced the bottom specification for this.)
</sup></sup></sub></sup></sup></sup></sup></sup></sup></sup></sup>