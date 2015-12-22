from distutils.core import setup
import py2exe
import sys
sys.path.append("C:\Python25\Lib\site-packages")

setup( windows=["Rubik's Cube.py"])
#       datafiles=[ ("Images",["Blue256.png","Green256.png","Instructions.png",
#                              "None64.png","Orange256.png","Red256.png",
#                              "RubiksCubeIcon.png","White256.png","Yellow256.png"])],
#       )
