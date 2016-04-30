from scirust import *

printVec(pAdd(mkVec(b"1,2,3"), mkVec(b"1,1,1")))
#>2,3,4

print(dot(mkVec(b"1,2,3"), mkVec(b"1,1,1")))
#> 6.0
