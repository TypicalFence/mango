from pymango.c import libmango
import ctypes

class MangoMetaData(object):
    def __init__(self, pointer):
        self._pointer = pointer
    
    @property
    def title(self):
        ptr = libmango.mangometa_get_title(self._pointer)
        try:
            value = ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
        except:
            value = None
        finally:
            # TODO free pointer here desu
            pass

        return value
    
    @title.setter
    def title(self, value):

        libmango.mangometa_set_title(self._pointer, value.encode("utf-8"))

class MangoFile(object):
    def __init__(self):
        self._pointer = libmango.new_mango_file()
    
    @property
    def meta_data(self):
        return MangoMetaData(libmango.mangofile_get_meta(self._pointer))
