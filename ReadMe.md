# Mango

Mango is a very cool file format for manga and comics. Its a rather pointless little experiment, solving a problem that wasn't really there.
I was always a bit bummed out about the fact, that a format like cbz/cbr doesn't store any metadata and would be easily readable for any cloud storage provider. In the end its just a archive with a bunch image files.

The .mango file format solves those issues, that for 99% of everyone isn't even a issue to begin with because it keeps stuff simple.

## Quick rundown
.mango files are basically just json, but not really, because that would make them huge due to base64, there is the option to save it as json tho. So yeah its just a binray-json format containing some metadata and all the images, with a checksum. There are currently bson and cbor as options for storing a .mango file, cbor is the default, because it has less overhead.
Its also possible to compress and encrypt it, by design every image can be encrypted with its own password and encryption algorithm, the same goes for the compression.

## Folder Structure
This git-repository features multiple projects:
    
* mangogmt
* libmango
* pymangofmt

I'll write individual ReadMe's later.

### Mangofmt
This is the main library, its written in Rust. Its the place where the magic happens basically.

### Libmango
This is a wrapper around mangofmt which acts as a C interface for ffi. It empowers other languages to use the mango file format easily! Rust is also used for it, but it features unit-tests, which are more like integration-tests written in plain C, to ensure everything operates how excepted.

### Pymangofmt
This is a Python library, which uses libmango for calling the Rust code, via the ctypes module. It also has tests, which use pytest.

## License
The 3 Subprojects are licensed differently:

- MIT
    - mangofmt
- LGPL3
    - libmango
    - pymangofmt

Please check the License file in each directory for more information.
