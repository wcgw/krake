# krake
Controls for NZXT bells and whistles

## WIP

### Requirements

Have `pkg-config` & `libusb` installed

On MacOS w/ home brew:

```
$ brew install pkg-config
$ brew install libusb
$ cargo build --release
$ ./target/release/krake list
Bus 001 Device 008: NZXT Smart Device
Bus 001 Device 006: NZXT Kraken X62
$
```

### Contributing

```
$ cargo +nigthly fmt
```


