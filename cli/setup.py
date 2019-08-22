from setuptools import setup

setup(name="mango_cli",
      version='0.1',
      description='cli utils for the mango file format',
      author='Alex Fence',
      license='GPL3',
      py_modules=["mangocreate", "mangoconvert", "mangoinfo", "mangometa"],
      entry_points={
          'console_scripts': ["mangocreate=mangocreate:main",
                              "mangoconvert=mangoconvert:mango_convert",
                              "mangoinfo=mangoinfo:main",
                              "mangometa=mangometa:main"
                              ]
      })
