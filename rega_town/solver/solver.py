from z3 import *

flag_length = 33
flag_pattern = "HTB{"

a1 = [Int('DomVect_%s' % i) for i in range(flag_length)]
s = Solver()

# validate characters
s.add([Or(And(a1[i] >= 48, a1[i] <= 57), And(a1[i] >= 64, a1[i] <= 90), And(a1[i] >= 97, a1[i] <= 122), a1[i]==95) for i in range(4, len(a1) - 1)])
s.add([a1[i] == ord(flag_pattern[i]) for i in range(len(flag_pattern))])
s.add(a1[-1] == ord("}"))

# .{24}\x54.\x65.\x54.*
s.add(a1[24] == ord("T"))
s.add(a1[26] == ord("e"))
s.add(a1[28] == ord("T"))

s.add(a1[7] == 95)
s.add(a1[11] == 95)
s.add(a1[15] == 95)
s.add(a1[20] == 95)
s.add(a1[23] == 95)
s.add(a1[27] == 95)

# ^[[:upper:]]{3}.[[:upper:]].{3}[[:upper:]].{3}[[:upper:]].{3}[[:upper:]].{4}[[:upper:]].{2}[[:upper:]].{3}[[:upper:]].{4}$
s.add(And(a1[4] > 64, a1[4] < 91))
s.add(And(a1[8] > 64, a1[8] < 91))
s.add(And(a1[12] > 64, a1[12] < 91))
s.add(And(a1[16] > 64, a1[16] < 91))
s.add(And(a1[21] > 64, a1[21] < 91))
s.add(And(a1[24] > 64, a1[24] < 91))
s.add(And(a1[28] > 64, a1[28] < 91))

# ^.{4}[X-Z]\d..[A]\D\d.................[[:upper:]][n-x]{2}[n|c].$
s.add(And(a1[4] >= ord("X"), a1[4] <= ord("Z")))
s.add(And(a1[5] >= 48, a1[5] <= 57))
s.add(a1[8] == ord("A"))
s.add(Or(And(a1[9] >= 64, a1[9] <= 90), And(a1[9] >= 97, a1[9] <= 122)))
s.add(And(a1[10] >= 48, a1[10] <= 57))
s.add(And(a1[28] >= 64, a1[28] <= 90))
s.add(And(a1[29] >= ord("n"), a1[29] <= ord("x")))
s.add(And(a1[30] >= ord("n"), a1[30] <= ord("x")))
s.add(Or(a1[31] == ord("n"), a1[31] == ord("c")))

# {11}_T[h|7]\d_[[:upper:]]\dn[a-h]_[O]\d_[[:alpha:]]{3}_.{5}
s.add(a1[12] == ord('T'))
s.add(Or(a1[13] == ord("h"),a1[13] == ord("7")))
s.add(And(a1[14] >= 48, a1[14] <= 57))
s.add(And(a1[16] >= 64, a1[16] <= 90))
s.add(And(a1[17] >= 48, a1[17] <= 57))
s.add(a1[18] == ord("n"))
s.add(And(a1[19] >= ord("a"), a1[19] <= ord("h")))
s.add(a1[21] == ord("O"))
s.add(And(a1[22] >= 48, a1[22] <= 57))

words = [a1[4:7], a1[8:11], a1[12:15], a1[16:20], a1[21:23], a1[24:27], a1[28:32]]
corr_muls = [0x7a070, 0x5c436, 0x6cc60, 0x27b5776, 0x10f9, 0xd76a0, 0x7465a58]

for i in range(len(words)):
    mul1 = 1
    for j in words[i]:
        mul1 *= j
    s.add(mul1 == corr_muls[i])

while s.check() == sat:
    m = s.model()
    final = ""
    for x in a1:
        final += chr(m[x].as_long())
    print(final)
    s.add(Or([a1[i] != m[a1[i]] for i in range(flag_length)]))     
else:
    print("No solution")