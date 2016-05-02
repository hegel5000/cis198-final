import sys, os
import ctypes 
top_path = os.path.dirname(__file__)
libc = ctypes.CDLL("libc.so.6")
scirust = ctypes.CDLL(os.path.join(top_path, "target/debug/libscirust.so"))

printVec = scirust.print_vec_64
printVec.argtypes = [ctypes.c_void_p]

mkVec = scirust.parse_vec_64
mkVec.restype = ctypes.c_void_p
mkVec.argtypes = [ctypes.c_char_p]

dot = scirust.dot_64
dot.restype = ctypes.c_double
dot.argtypes = [ctypes.c_void_p, ctypes.c_void_p]

pAdd = scirust.pairwise_add_64
pAdd.restype = ctypes.c_void_p
pAdd.argtypes = [ctypes.c_void_p, ctypes.c_void_p]

# Clean up namespace
del sys, os, ctypes
