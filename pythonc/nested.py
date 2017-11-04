a = 3
def f():
    a = 1
    def g():
        return a
    return g
a = 3
print f()()
