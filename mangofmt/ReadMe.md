# Mangofmt

Mango is small file format I created, it tries to replace CBZ/CBR, which are lacking metadata, among other things, for me personally.

This library implements the file format and can be used to create, read, write and manipulate ".mango" files.
There are also bindings to C and Python available, check the [git repository](https://github.com/AlexFence/mango) for those.

Also checkout the Documentation! It explains much more than this little readme, and of course checkout the main read me in the root of the git repository, it goes into more detail as well.

## Features
The following Features are currently available, none are enabled by default:
* aes
   * requires openssl
* gzip

If you do not enable them, then you will be all of the compression and encryption features.

