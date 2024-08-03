from turtle import *
while True:
    r = input("Please enter radius: ")
    if r.isdigit():
        r = int(r)
        break
    print("Please enter an integer: ")
small_gap = 0.2 * r
top_move = r*2 + small_gap
pensize(3)
penup()
backward(r * 2)
pendown()
color("blue")
circle(r)
penup()
forward(top_move)
x = pos()
color("black")
circle(r)
penup()
forward(top_move)
pendown()
color("red")
circle(r)
penup()
goto(-r * 2, (3*r)/4)
right(90)
forward(r*2)
left(90)
forward(top_move/2)
pendown()
color("yellow")
circle(r)
penup()
goto(x)
forward(top_move/2)
left(90)
forward((3*r)/4)
left(180)
forward(r*2)
left(90)
pendown()
color("green")
circle(r)
done()
