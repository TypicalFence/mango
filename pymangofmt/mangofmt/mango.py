import base64
import ctypes
from ctypes import *
from mangofmt.c import libmango
from mangofmt.enums import CompressionType, EncryptionType
from mangofmt.error import DecodeError, ReadError

class MangoFile(object):
    def __init__(self, pointer=None):
        if pointer is None:
            self._pointer = libmango.new_mango_file()
        else:
            self._pointer = pointer


    def __del__(self):
        libmango.mangofile_free(self._pointer)

    @property
    def meta_data(self):
        return MangoMetaData(libmango.mangofile_get_meta(self._pointer), self)

    @property
    def image_count(self):
        return libmango.mangofile_get_image_count(self._pointer)

    @staticmethod
    def open(path):
        error = c_int(-10)
        pointer = libmango.mangofile_open(path.encode("utf-8"), byref(error))

        if error.value != 0:
            if error.value == 1:
                raise DecodeError
            elif error.value == 2:
                raise ReadError
            elif error.value == 3:
                raise FileNotFoundError
            else:
                raise Exception("Unknown Error")

        return MangoFile(pointer)

    def get_image(self, index):
        if index < self.image_count and index >= 0:
            pointer = libmango.mangofile_get_image(self._pointer, index)
            return MangoImage(pointer, self)
        else:
            raise IndexError

    def set_image(self, img, index):
        if index < self.image_count and index >= 0:
            success = libmango.mangofile_set_image(self._pointer, img._pointer, index)

            if success == 1:
                return True
            elif success == 0:
                return False
            else:
                raise IndexError
        else:
            raise IndexError

    def add_image(self, img):
        # TODO check type of path
        libmango.mangofile_add_image(self._pointer, img._pointer)

    def add_image_by_path(self, path):
        # TODO check type of path
        libmango.mangofile_add_image_by_path(self._pointer, path.encode("utf-8"))

    # TODO add error handle
    def save(self, path):
        libmango.mangofile_save(self._pointer, path.encode("utf-8"))

    def save_cbor(self, path):
        libmango.mangofile_save_cbor(self._pointer, path.encode("utf-8"))

    def save_bson(self, path):
        libmango.mangofile_save_bson(self._pointer, path.encode("utf-8"))

    def save_json(self, path):
        libmango.mangofile_save_json(self._pointer, path.encode("utf-8"))

class MangoMetaData(object):
    def __init__(self, pointer, parent):
        self._pointer = pointer
        self._parent = parent

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
    def __init__(self, pointer, parent=None):
        self._pointer = pointer
        self._parent = parent

    def __del__(self):
        libmango.mangoimg_free(self._pointer)

    @staticmethod
    def from_path(path):
        error = c_int(-10)
        pointer = libmango.mangoimg_from_path(path.encode("utf-8"), byref(error))

        if error.value != 0:
            print(error)
            if error.value == 1:
                raise FileNotFoundError
            else:
                raise Exception("Unknown Error")

        return MangoImage(pointer)

    @property
    def meta_data(self):
        pointer = libmango.mangoimg_get_meta(self._pointer)
        return MangoImageMetadata(pointer, self)

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
    def __init__(self, pointer, parent):
        self._pointer = pointer
        self._parent = parent

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
            value = ptr.decode('utf-8')
        except:
            print("holy fucking shit")
            value = None
        finally:
            # TODO free pointer here desu
            pass

        return value

        return checksum

    @property
    def filename(self):
        ptr = libmango.mangoimgmeta_filename(self._pointer)
        if ptr is not None:
            return ptr.decode("utf-8")
        else:
            return None
