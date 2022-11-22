# Icons

The Flipper Zero uses a custom bitmap format for icons.

Image data is stored as a 1-bit monochrome bitmap. You can use ImageMagic to convert existing images:

```bash
convert rustacean-48x32.png mono:rustacean-48x32.dat
```

Icons files have a simple length header and can optionally be compressed with [heatshrink](https://github.com/atomicobject/heatshrink).

**Uncompressed:**

```
0x00 LEN_LOWER LEN_UPPER DATA...
```

**Compressed (`heatshrink -w 8 -l 4`):**

```
0x01 0x00 LEN_LOWER LEN_UPPER COMPRESSED_DATA...
```
