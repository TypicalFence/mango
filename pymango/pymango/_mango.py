from ._rust_pymango import PyMangoFile, PyMangoImage, PyMangoImageMetadata
from .enums import CompressionType


class MangoFile(PyMangoFile):

    def add_image(self, image):
        if issubclass(type(image), PyMangoImage):
            self._add_image(image)
        elif issubclass(type(image), str):
            # maybe check here if actually a path?
            self.add_image_by_path(image)
        else:
            raise TypeError


class MangoImage(PyMangoImage):
    @property
    def meta_data(self):
        return self._get_meta_data()

    def compress(self, ctype):
        comp_type = None
        if isinstance(type(ctype), CompressionType):
            comp_type = ctype.value
        elif isinstance(type(ctype), str):
            comp_type = ctype
        else:
            raise TypeError
        
        return self._compress(comp_type)
        



class MangoMetadata(PyMangoImageMetadata):
    pass
