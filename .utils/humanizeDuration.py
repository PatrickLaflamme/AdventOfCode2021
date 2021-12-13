import sys

NS = float(sys.argv[1])
US = NS / 1000
MS = US / 1000
S = MS / 1000

if MS > 1000:
  print("{:.3g}s".format(S))
elif US > 1000:
  print("{:.3g}ms".format(MS))
elif NS > 1000:
  print("{:.3g}µs".format(US))
else:
  print("{:.3g}ns".format(NS))