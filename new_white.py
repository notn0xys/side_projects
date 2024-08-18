i = 0
height = 5

while i < height:
    print(" " * (i), end="")
    print("*" * (((height- i)* 2) - 1))
    i += 1
i = 2
while i <= height:
    print(" " * (height - i), end="")
    print("*" * ((i * 2)- 1))
    i += 1       