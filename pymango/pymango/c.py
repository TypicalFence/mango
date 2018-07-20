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

class ImageData(Structure):
	_fields_ = [("pointer", POINTER(c_ubyte)),
				("length", c_size_t)]
	
	def __exit__(self, exc_type, exc_value, traceback):
            libmango.mango_imagedata_free(self)

# -----------------------------------------------------------------------------
# Mango File
# -----------------------------------------------------------------------------
# MangoFile new_mango_file();
libmango.new_mango_file.restype = POINTER(RustMangoFile)

libmango.mangofile_get_meta.argtypes = (POINTER(RustMangoFile),)
libmango.mangofile_get_meta.restype = POINTER(RustMangoMetadata)

libmango.mangofile_add_image_by_path.argtypes = (POINTER(RustMangoFile), c_char_p)
#libmangi.mangofile_add_image_by_path.restype = 

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

libmango.mangometa_get_author.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_author.restype = c_void_p

libmango.mangometa_set_title.argtypes = (POINTER(RustMangoMetadata), c_char_p)
libmango.mangometa_set_title.restype = None

# -----------------------------------------------------------------------------
# Mango Image
# -----------------------------------------------------------------------------
libmango.mangoimg_get_meta.argtypes = (POINTER(RustMangoImage),)
libmango.mangoimg_get_meta.restype = POINTER(RustMangoImageMetadata)

libmango.mangoimg_get_image_data.argtypes = (POINTER(RustMangoImage),)
libmango.mangoimg_get_image_data.restype = ImageData

libmango.mangoimg_get_base64_image_data.argtypes = (POINTER(RustMangoImage),)
libmango.mangoimg_get_base64_image_data.restype = c_void_p

libmango.mangoimg_compress.argtypes = (POINTER(RustMangoImage), c_char_p)
libmango.mangoimg_compress.restype = c_bool

libmango.mangoimg_uncompress.argtypes = (POINTER(RustMangoImage),)
libmango.mangoimg_uncompress.restype = c_bool

libmango.mangoimg_from_path.argtypes = (c_char_p,)
libmango.mangoimg_from_path.restype = POINTER(RustMangoImage)

# -----------------------------------------------------------------------------
# Mango Image Metadata
# -----------------------------------------------------------------------------
# char * mangoimgmeta_compression(MangoImageMeta);
libmango.mangoimgmeta_compression.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_compression.restype = c_char_p

libmango.mangoimgmeta_encryption.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_encryption.restype = c_char_p

libmango.mangoimgmeta_compression.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_compression.restype = c_char_p


# char * mangoimgmeta_checksum(MangoImageMeta);
libmango.mangoimgmeta_checksum.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_checksum.restype = c_void_p # char_p


