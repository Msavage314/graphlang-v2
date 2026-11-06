# Graphlang
A programming language which compiles to desmos.

#### Features
###### Variables
variables are automatically subscripted, so you can use multi-letter names
```
longName = 1
s = 2
```
###### Points
```
p = (1,2)
```
###### Lists
```
mylist = [1,2,3,4+4]
y = mylist[2,3]
```
###### Functions
```
fn Add(a,b) = a + b
```
###### Namespaces
Namespaces use scoping rules, simplifying larger projects
```
namespace Square {
    xPos = 0
    yPos = 0
    size = 10
}

p = (Square.xPos,Square.yPos)

```
###### Macros
Macros can be used to generate code. For example, here is a macro which generates a rectangle at specified coordinates
These are variable powerful, and can be used to make almost libraries
```
macro Rectangle($name, $x, $y, $w, $h) {
    namespace $name {
        Xpos = $x
        Ypos = $y
        Width = $w
        Height = $h
        polygon((Xpos - (Width / 2), Ypos - (Height / 2)),(Xpos + (Width / 2), Ypos - (Height /2 )),(Xpos + (Width / 2), Ypos + (Height /2 )),(Xpos - (Width / 2), Ypos + (Height /2 )))
    }
}

Rectangle!(A,1,1,1,1)

A.Xpos
```


#### Future features
- Style information
- Full desmos parity...

