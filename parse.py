import compiler
import sys

if len(sys.argv) < 2:
    print "Usage: parse.py <source>"
else:
    source = sys.argv[1]
    parsed = compiler.parseFile(source)
    sys.stdout.write(str(parsed))
