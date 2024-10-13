# libsml-sys

Low-level bindings to the system [libsml](https://github.com/volkszaehler/libsml) library.

> libSML is a library which implements the Smart Message Language (SML) protocol specified by VDE's Forum Netztechnik/Netzbetrieb (FNN). It can be utilized to communicate to FNN specified Smart Meters or Smart Meter components (EDL/MUC).  
> — [github.com/volkszaehler/libsml](https://github.com/volkszaehler/libsml)


## Installation

According to the [libsml documentation](https://github.com/volkszaehler/libsml?tab=readme-ov-file#dependencies), install the following dependencies:

```
$ sudo apt-get install uuid-dev uuid-runtime
```

For building `libsml-sys` also `clang` is required:

```
$ sudo apt-get install clang
```

Download `libsml1_1.1.3_amd64.deb` as well as `libsml-dev_1.1.3_amd64.deb` from [libsml's releases page](https://github.com/volkszaehler/libsml/releases/) and install them using

```
$ sudo dpkg -i libsml1_1.1.2_amd64.deb
$ sudo dpkg -i libsml-dev_1.1.2_amd64.deb