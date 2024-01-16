# libsml-rs

High-level Rust bindings to [libsml](https://github.com/volkszaehler/libsml).

---

## Installation

According to [libsml](https://github.com/volkszaehler/libsml/blob/master/README.md#usage), install the following dependencies:

```
$ sudo apt-get install uuid-dev uuid-runtime
```

Download `libsml1_1.1.2_amd64.deb` as well as `libsml-dev_1.1.2_amd64.deb` from [libsml's releases page](https://github.com/volkszaehler/libsml/releases/) and install them using

```
$ sudo dpkg -i libsml1_1.1.2_amd64.deb
$ sudo dpkg -i libsml-dev_1.1.2_amd64.deb
```

## Progress

- [x] `sml_attention_response`
- [x] `sml_boolean`
- [x] `sml_close_request`
- [x] `sml_close_response`
- [x] `crc16`
