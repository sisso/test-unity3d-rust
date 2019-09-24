from ctypes import cdll

lib = cdll.LoadLibrary("target/release/librustlib.so")

r = lib.add_numbers(3, 3)

print("done! %s" % str(r))
