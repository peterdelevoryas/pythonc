import compiler
import sys

if len(sys.argv) < 2:
    parse = compiler.parse
    source = sys.stdin.read()
else:
    parse = compiler.parseFile
    source = sys.argv[1]

parsed = parse(source)
print parsed
