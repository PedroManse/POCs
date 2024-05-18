from functools import reduce

class Piper:
    def __init__(self, fn=None):
        self.fn = fn or (lambda x: x)
    def _pipe(self, f1, f2):
        return Piper(lambda *x: f2(f1(*x)))
    def __or__(self, next):
        if (type(next) == tuple):
            next = B(*next)
        return self._pipe(self.fn, next)
    def __call__(self, *args):
        return self.fn(*args)

class GMID:pass
_ = GMID()

class B:
    def __init__(self, fn, *args):
        self.fn = fn
        self.args = list(args)
    def __call__(self, arg):
        args = list(self.args)
        for i in range(len(self.args)):
            if args[i] is _:
                args[i] = arg
                break
        else:
            return self.fn(*args, arg)
        return self.fn(*args)

pipe = Piper()
def double(a):
    return a*2
def mul(a, b):
    return a*b
def div(a, b):
    return a/b;

x = pipe\
    |(mul, 6)\
    |(div, 1000, _)\
    |(div, _, 10)\
    | int | str | print
    # x*6
    # 1000/(x*6)
    # (1000/(x*6))/10
    # int((1000/(x*6))/10)
    # str(int((1000//x*6))/10))

x(1)
x(2)
x(4)
x(8)
x(16)
    # str(int((1000/(2*6))/10))
    # str(int(83.3/10))
    # str(int(8.33))
    # str(8)
    # "8"


