from ctypes import *
libc = CDLL("libc.so.6")
scirust = CDLL("target/debug/libscirust.so")
