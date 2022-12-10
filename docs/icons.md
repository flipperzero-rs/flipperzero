# Icons

The Flipper Zero uses a simple bitmap format for icons.

Image data is stored as a 1-bit monochrome bitmap. You can use [ImageMagic](https://imagemagick.org/) to convert existing images:

```bash
convert rustacean.png mono:rustacean.bitmap
```

Icons can be optionally compressed with [heatshrink](https://github.com/atomicobject/heatshrink) to reduce their size.

## Uncompressed

If an icon is uncompressed, the first byte will be `0x00` followed by the uncompressed image data:

```
0x00 DATA...
```

### Example

```bash
# Create an uncompressed icon
(echo -ne '\x00'; convert rustacean.png mono:-) > rustacean.icon
```

## Compressed

If an icon is compressed, the first byte will be `0x01`,
followed by a 16-bit unsigned integer (little endian) data length,
followed by the heatshrink compressed image data (`heatshrink -w 8 -l 4`):

```
0x01 0x00 LEN_LOWER LEN_UPPER COMPRESSED_DATA...
```

### Example

```bash
# Create a compressed icon
convert rustacean.png mono:- | ./heatshrink -w 8 -l 4 > rustacean.heatshrink
python -c 'import struct; d=open("rustacean.heatshrink", "rb").read(); o=open("rustacean.icon", "wb"); o.write(struct.pack("<BH", 1, len(d))); o.write(d)'
```
