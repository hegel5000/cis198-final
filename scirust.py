from ctypes import *
libc = CDLL("libc.so.6")
scirust = CDLL("target/debug/libscirust.so")

printVec = scirust.print_vec_64
printVec.argtypes = [c_void_p]

mkVec = scirust.parse_vec_64
mkVec.restype = c_void_p
mkVec.argtypes = [c_char_p]

dot = scirust.dot_64
dot.restype = c_double
dot.argtypes = [c_void_p, c_void_p]

pAdd = scirust.pairwise_add_64
pAdd.restype = c_void_p
pAdd.argtypes = [c_void_p, c_void_p]
