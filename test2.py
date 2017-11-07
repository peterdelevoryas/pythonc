x = 1
def f():
    y = 1
    def g():
        return x + y
    return g
