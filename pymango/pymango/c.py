import ctypes
from ctypes import *




libmango = ctypes.cdll.LoadLibrary("libmango.so")

class RustMangoFile(Structure):
    pass

class RustMangoMetadata(Structure):
    pass

libmango.new_mango_file.restype = POINTER(RustMangoFile)

libmango.mangofile_get_meta.argtypes = (POINTER(RustMangoFile),)
libmango.mangofile_get_meta.restype =  POINTER(RustMangoMetadata)

libmango.mangometa_get_title.argtypes = (POINTER(RustMangoMetadata),)
libmango.mangometa_get_title.restype = c_void_p

libmango.mangometa_set_title.argtypes = (POINTER(RustMangoMetadata), c_char_p)
libmango.mangometa_set_title.restype = None
