from pymango import _lib as lib
print("nya")
mangofile = lib.new_mango_file(b"test");
print(mangofile)
print("fewasfaew")
name = lib.mangofile_get_name(mangofile)
print("name: " + name)

"""
from pymango import mango
from ctypes import c_char_p, POINTER, Structure

class List(Structure): pass

lib = mango._LIB
lib.new_mango_file.restype = POINTER(List)
lib.mangofile_get_name.argtypes = [POINTER(List)]

lel = lib.new_mango_file("lel")
print(lel)
lul = lib.mangofile_get_name(lel, "test.jpg")
"""
"""
c_char_p('lel')
test = mango.MangoFile()

print(test._pointer)
name = test.name
print("kek")
#print(test_file.name)
"""
