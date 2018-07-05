from pymango import new
import ctypes

file = new.MangoFile()

meta = file.meta_data
print(file._pointer)
print(meta._pointer)
print(file.meta_data.title)
meta.title = "lol"

print(file.meta_data.title)

