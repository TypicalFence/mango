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

class IntOption(Structure):
    _fields_ = [("value", c_int),
                ("present", c_int)]

# -----------------------------------------------------------------------------
# Mango File
# -----------------------------------------------------------------------------
libmango.new_mango_file.restype = POINTER(RustMangoFile)

libmango.mangofile_free.argtypes = (POINTER(RustMangoFile),)

# Image Manipulation
libmango.mangofile_add_image.argtypes = (POINTER(RustMangoFile), POINTER(RustMangoImage))
libmango.mangofile_add_image.restype = None

libmango.mangofile_add_image_by_path.argtypes = (POINTER(RustMangoFile), c_char_p)
libmango.mangofile_add_image_by_path.restype = c_int

libmango.mangofile_get_image.argtypes = (POINTER(RustMangoFile), c_uint)
libmango.mangofile_get_image.restype = POINTER(RustMangoImage)

libmango.mangofile_set_image.argtypes = (POINTER(RustMangoFile), POINTER(RustMangoImage), c_int)
libmango.mangofile_set_image.restype =  c_int

libmango.mangofile_get_image_count.argtypes = (POINTER(RustMangoFile),)
libmango.mangofile_get_image_count.restype = c_int

libmango.mangofile_get_meta.argtypes = (POINTER(RustMangoFile),)
libmango.mangofile_get_meta.restype = POINTER(RustMangoMetadata)

# Save
libmango.mangofile_save.argtypes = (POINTER(RustMangoFile), c_char_p)
libmango.mangofile_save.restype = c_int

libmango.mangofile_save_cbor.argtypes = (POINTER(RustMangoFile), c_char_p)
libmango.mangofile_save_cbor.restype = c_int

libmango.mangofile_save_bson.argtypes = (POINTER(RustMangoFile), c_char_p)
libmango.mangofile_save_bson.restype = c_int

libmango.mangofile_save_json.argtypes = (POINTER(RustMangoFile), c_char_p)
libmango.mangofile_save_json.restype = c_int

# Open
libmango.mangofile_open.argtypes = (c_char_p, POINTER(c_int))
libmango.mangofile_open.restype = POINTER(RustMangoFile)

# -----------------------------------------------------------------------------
# Mango Metadata
# -----------------------------------------------------------------------------
# title
libmango.mangometa_get_title.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_title.restype = c_void_p

libmango.mangometa_set_title.argtypes = (POINTER(RustMangoMetadata), c_char_p)
libmango.mangometa_set_title.restype = None

# author
libmango.mangometa_get_author.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_author.restype = c_void_p

libmango.mangometa_set_author.argtypes = (POINTER(RustMangoMetadata), c_char_p)
libmango.mangometa_set_author.restype = None

# publisher
libmango.mangometa_get_publisher.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_publisher.restype = c_void_p

libmango.mangometa_set_publisher.argtypes = (POINTER(RustMangoMetadata), c_char_p)
libmango.mangometa_set_publisher.restype = None

# source
libmango.mangometa_get_source.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_source.restype = c_void_p

libmango.mangometa_set_source.argtypes = (POINTER(RustMangoMetadata), c_char_p)
libmango.mangometa_set_source.restype = None

# translation
libmango.mangometa_get_translation.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_translation.restype = c_void_p

libmango.mangometa_set_translation.argtypes = (POINTER(RustMangoMetadata), c_char_p)
libmango.mangometa_set_translation.restype = None

# volume
libmango.mangometa_get_volume.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_volume.restype = IntOption

libmango.mangometa_set_volume.argtypes = (POINTER(RustMangoMetadata), POINTER(c_int))
libmango.mangometa_set_volume.restype = None

# chapter
libmango.mangometa_get_chapter.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_chapter.restype = IntOption

libmango.mangometa_set_chapter.argtypes = (POINTER(RustMangoMetadata), POINTER(c_int))
libmango.mangometa_set_chapter.restype = None

# chapter
libmango.mangometa_get_year.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_year.restype = IntOption

libmango.mangometa_set_year.argtypes = (POINTER(RustMangoMetadata), POINTER(c_int))
libmango.mangometa_set_year.restype = None

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

libmango.mangoimg_encrypt.argtypes = (POINTER(RustMangoImage), c_char_p, c_char_p)
libmango.mangoimg_encrypt.restype = c_bool

libmango.mangoimg_decrypt.argtypes = (POINTER(RustMangoImage), c_char_p)
libmango.mangoimg_decrypt.restype = c_bool

libmango.mangoimg_from_path.argtypes = (c_char_p, POINTER(c_int))
libmango.mangoimg_from_path.restype = POINTER(RustMangoImage)

libmango.mangoimg_save.argtypes = (POINTER(RustMangoImage), c_char_p)
libmango.mangoimg_save.restype = c_int

# -----------------------------------------------------------------------------
# Mango Image Metadata
# -----------------------------------------------------------------------------
libmango.mangoimgmeta_compression.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_compression.restype = c_char_p

libmango.mangoimgmeta_encryption.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_encryption.restype = c_char_p

libmango.mangoimgmeta_filename.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_filename.restype = c_char_p

libmango.mangoimgmeta_mime.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_mime.restype = c_char_p

# char * mangoimgmeta_checksum(MangoImageMeta);
libmango.mangoimgmeta_checksum.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_checksum.restype = c_char_p

libmango.mangoimgmeta_iv.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_iv.restype = POINTER(c_int)

libmango.mangoimgmeta_iv_size.argtypes = (POINTER(RustMangoImageMetadata),)
libmango.mangoimgmeta_iv_size.restype = c_int

