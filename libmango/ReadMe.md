# Libmango
Mango is small file format I created, it tries to replace CBZ/CBR, which are lacking metadata, among other things, for me personally.

This is a wrapper around the mangofmt crate, which enables C to call its functionality. 
It also enables most Languages (that have a way to call C) to use mangofmt.

# Installation
It can be installed install via the make file.

**Important:** 
you must run  `make release` before you can run `make install`.
This has to do with the fact that rust probably won't be install for the root user and the project must be build first with your regular user.
(I assume here that you have installed rust via rustup)

```
make release
sudo make install
```

If you want to uninstall it, you can do as follows:

```
sudo make uninstall
```

You can specify a few flags, like $PREFIX and $FEATURES. The latter will be passed to cargo, for more info about the available features, consult mangofmt's readme, currently both aes and gzip are enabled per default when compiling libmango
