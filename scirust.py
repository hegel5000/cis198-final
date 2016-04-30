from ctypes import *
scirust = CDLL("target/debug/libscirust.so")
libc = CDLL("libc.so.6")
