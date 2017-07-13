import os
from ctypes import cdll, c_char_p


_HERE = os.path.dirname(os.path.realpath(__file__))
_LIB = cdll.LoadLibrary(_HERE + "/../libmango.so")
#_LIB.mangofile_get_name.restype = c_char_p

class MangoFile:
    def __init__(self, name):
        self._pointer = _LIB.new_mango_file(name)
    def __del__(self):
        _LIB.mango_free(self._pointer)
    @property
    def name(self):
        return _LIB.mangofile_get_name(self._pointer)

    @name.setter
    def name(self, name):
        _LIB.mangofile_set_name(self._pointer, name)
