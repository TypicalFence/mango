from ._rust_pymango import PyMangoFile, PyMangoImage, PyMangoImageMetadata
from .enums import CompressionType, EncryptionType


class MangoFile(PyMangoFile):

    def add_image(self, image):
        if isinstance(image, MangoImage):
            self._add_image(image)
        elif issubclass(type(image), str):
            # maybe check here if actually a path?
            self.add_image_by_path(image)
        else:
            raise TypeError

    def get_image(self, index):
        img = self._get_image(index)
        return MangoImage(img)
    """
    @property
    def meta(self):
        return self.meta
        """

class MangoImage(object):
    def __init__(self, base):
        # isinstance does not work
        if not issubclass(type(base), PyMangoImage):
            raise ValueError

        self._base = base

    def from_path(path):
        img = PyMangoImage(path)
        return MangoImage(img)

    @property
    def image_data(self):
        return self._base.image_data

    @property
    def meta_data(self):
        return MangoImageMetadata(self._base._get_meta_data())

    def compress(self, ctype):
        comp_type = None
        if isinstance(ctype, CompressionType):
            comp_type = ctype.value
        elif isinstance(ctype, str):
            comp_type = ctype
        else:
            raise TypeError

        return self._base.compress(comp_type)

    def uncompress(self):
        return self._base.uncompress()

    def save(self, filename):
        self._base.save(filename)


class MangoImageMetadata(object):
    def __init__(self, base):
        # isinstance does not work
        if not issubclass(type(base), PyMangoImageMetadata):
            raise ValueError

        self._base = base

    @property
    def checksum(self):
        return self._base.checksum

    @property
    def encryption(self):
        enc_str = self._base._get_encryption()
        if enc_str is not None:
            return EncryptionType(enc_str)
        else:
            return None
    @property
    def compression(self):
        enc_str = self._base._get_compression()
        if enc_str is not None:
            return CompressionType(enc_str)
        else:
            return None

