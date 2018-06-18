from ._rust_pymango import PyMangoFile, PyMangoImage


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
    pass
