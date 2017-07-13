import os
import ctypes
from ctypes import cdll, c_uint32, c_char_p, c_void_p, Structure, POINTER, c_void_p


_HERE = os.path.dirname(os.path.realpath(__file__))
_LIB = cdll.LoadLibrary(_HERE + "/../libmango.so")


free_mangofile = _LIB.free_mangofile
free_mangofile.argtypes = (c_void_p, )

_new_mango_file = _LIB.new_mango_file
_new_mango_file.argtypes = (c_char_p, )
_new_mango_file.restype = c_void_p

def new_mango_file(name):
    _new_mango_file(name)

_mangofile_get_name = _LIB.mangofile_get_name
_mangofile_get_name.argtypes = (c_void_p, )
_mangofile_get_name.restype = c_char_p

def mangofile_get_name(mangofile):
    ptr = _mangofile_get_name(mangofile)
    try:
        return ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
    finally:
        free_mangofile(ptr)
