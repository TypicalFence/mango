import ctypes
from ctypes import *

libmango = ctypes.cdll.LoadLibrary("libmango.so")

class RustMangoFile(Structure):
    pass

class RustMangoMetadata(Structure):
    pass

class RustMangoImage(Structure):
    pass

class RustMangoImageMetadata(Structure):
    pass


# -----------------------------------------------------------------------------
# Mango File
# -----------------------------------------------------------------------------
# MangoFile new_mango_file();
libmango.new_mango_file.restype = POINTER(RustMangoFile)

libmango.mangofile_get_meta.argtypes = (POINTER(RustMangoFile),)
libmango.mangofile_get_meta.restype = POINTER(RustMangoMetadata)

# MangoImage mangofile_get_image(MangoFile, int);
libmango.mangofile_get_image.argtypes = (POINTER(RustMangoFile), c_uint)
libmango.mangofile_get_image.restype = POINTER(RustMangoImage)

# -----------------------------------------------------------------------------
# Mango Metadata
# -----------------------------------------------------------------------------
libmango.mangometa_get_title.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_title.restype = c_void_p

libmango.mangometa_set_title.argtypes = (POINTER(RustMangoMetadata), c_char_p)
libmango.mangometa_set_title.restype = None

# -----------------------------------------------------------------------------
# Mango Image
# -----------------------------------------------------------------------------
libmango.mangoimg_get_meta.argtypes = (POINTER(RustMangoImage),)
libmango.mangoimg_get_meta.restype = POINTER(RustMangoImageMetadata)

libmango.mangoimg_compress.argtypes = (POINTER(RustMangoImage), c_char_p)
libmango.mangoimg_compress.restype = c_bool

libmango.mangoimg_uncompress.argtypes = (POINTER(RustMangoImage),)
libmango.mangoimg_uncompress.restype = c_bool

# -----------------------------------------------------------------------------
# Mango Image Metadata
# -----------------------------------------------------------------------------
# char * mangoimgmeta_compression(MangoImageMeta);
libmango.mangoimgmeta_compression.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_compression.restype = c_char_p

# char * mangoimgmeta_checksum(MangoImageMeta);
libmango.mangoimgmeta_checksum.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_checksum.restype = c_char_p

