import ctypes
from pymango.c import libmango
from pymango.enums import CompressionType, EncryptionType


class MangoFile(object):
    def __init__(self):
        self._pointer = libmango.new_mango_file()

    @property
    def meta_data(self):
        return MangoMetaData(libmango.mangofile_get_meta(self._pointer))

    def get_image(self, index):
        pointer = libmango.mangofile_get_image(self._pointer, index)
        return MangoImage(pointer)


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

class MangoImage(object):
    def __init__(self, pointer):
        self._pointer = pointer

    @property
    def meta_data(self):
        pointer = libmango.mangoimg_get_meta(self._pointer)
        return MangoImageMetadata(pointer)

    def compress(self, ctype):
        comp_type = None

        if isinstance(ctype, CompressionType):
            comp_type = ctype.value
        elif isinstance(ctype, str):
            comp_type = ctype
        else:
            raise TypeError

        libmango.mangoimg_compress(self._pointer, comp_type)

class MangoImageMetadata(object):
    def __init__(self, pointer):
        self._pointer = pointer

    @property
    def compression(self):
        comp_type = libmango.mangoimgmeta_compression(self._pointer)
        return CompressionType(comp_type)

    @property
    def checksum(self):
        checksum = libmango.mangoimgmeta_checksum(self._pointer)
        return checksum