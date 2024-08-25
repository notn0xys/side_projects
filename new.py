height = 8;
for i in range(1,height + 1):
    print(" " * (i - 1), end="")
    if i % 2 ==0:
        print("#" * (((height + 1 - i ) * 2) - 1))
    else:
        print("*" * (((height + 1 - i ) * 2) - 1))

for i in range(2, height + 1):
    print(" " * (height - i) ,end = "")
    if i % 2 == 0:
        print("*" * ((i * 2) - 1))
    else: 
        print("#" * ((i * 2) - 1))

