# compute n-th element of Fibonacci Sequence

plusCount = 0
mulCount = 0

# x: matrix
# y: matrix
# return: x * y
def matrixMul2x2(x, y):
    global plusCount
    global mulCount
    
    r = [[0,0], [0,0]]
    
    for destI in range(2):
        for destJ in range(2):
            for c in range(2):
                r[destJ][destI] += x[destJ][c] * y[c][destI];
                plusCount = plusCount + 1
                mulCount = mulCount + 1
    return r


def identyMatrix():
    return [[1,0], [0,1]]


# x: matrix
# y: integer
# return: x^y

# algorithm: y = (a0, a1, a2, a3, a4, ......) * (b0, b1, b2, b3, b4, ....)
# so x^y = x^(a0*b0) * x^(a1*b1) * x^(a2*b2) * ...
# let bn = 2^n, an E {0, 1}
def matrixPow2_2x2(x, y):
    r = identyMatrix()
    curBase = x # curbase = x^bn
    while y != 0:
        if y % 2 == 1: # an == 1
            r = matrixMul2x2(r, curBase)
        y = y // 2
        curBase = matrixMul2x2(curBase, curBase)
    return r
    

def fib(n):
    r = matrixPow2_2x2([[1,1], [1,0]], n)
    return r[0][0]



# test code
n = 20000
print("{}-th element of Fibonacci Sequence is {}".format(n, fib(n)))
print("it runs {} times of plus operation and {} times of multiple operation".format(plusCount, mulCount))
