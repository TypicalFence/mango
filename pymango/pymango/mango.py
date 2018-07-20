import base64
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

    def add_image_by_path(self, path):
        # TODO check type of path
        libmango.mangofile_add_image_by_path(self._pointer, path.encode("utf-8"))

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

    @property
    def author(self):
        ptr = libmango.mangometa_get_author(self._pointer)
        try:
            value = ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
        except:
            value = None
        finally:
            # TODO free pointer here desu
            pass

        return value

    @author.setter
    def author(self, value):
        libmango.mangometa_set_author(self._pointer, value.encode("utf-8"))


class MangoImage(object):
    def __init__(self, pointer):
        self._pointer = pointer
    
    def from_path(path):
        return MangoImage(libmango.mangoimg_from_path(path.encode("utf-8")))

    @property
    def meta_data(self):
        pointer = libmango.mangoimg_get_meta(self._pointer)
        return MangoImageMetadata(pointer)

    @property
    def image_data(self):
        ptr = libmango.mangoimg_get_base64_image_data(self._pointer)
        try:
            value = ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
        except:
            value = None
        
        if value is not None:
            value = base64.b64decode(value) 

        return value

    def compress(self, ctype):
        comp_type = None

        if isinstance(ctype, CompressionType):
            comp_type = ctype.value
        elif isinstance(ctype, str):
            comp_type = ctype
        else:
            raise TypeError

        libmango.mangoimg_compress(self._pointer, comp_type.encode("utf-8"))

    def uncompress(self):
        libmango.mangoimg_uncompress(self._pointer)

class MangoImageMetadata(object):
    def __init__(self, pointer):
        self._pointer = pointer

    @property
    def compression(self):
        comp_type = libmango.mangoimgmeta_compression(self._pointer)
        if comp_type is not None:
            return CompressionType(comp_type)
        else:
            return None
    
    @property
    def encryption(self):
        enc_type = libmango.mangoimgmeta_encryption(self._pointer)
        if enc_type is not None:
            return EncryptionType(enc_type)
        else:
            return None

    @property
    def checksum(self):
        ptr = libmango.mangoimgmeta_checksum(self._pointer)
        try:
            value = ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
        except:
            print("holy fucking shit")
            value = None
        finally:
            # TODO free pointer here desu
            pass

        return value

        return checksum
