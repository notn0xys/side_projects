from turtle import *
def triangle(l):
    for i in range(3):
        forward(l)
        left(120)
def rest(x = 2, l = 250):
    for i in range(x):
        triangle(l/(4**i))
        forward(l/(2*(4**i)))
        left(60)
        triangle(l/(2*(4**i)))
        left(60)
        forward(l/(4*(4**i)))
        right(120)

rest(2)
done()