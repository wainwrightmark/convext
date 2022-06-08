## Convext

[run in your browser](https://wainwrightmark.github.io/convext/). 

Convext is a programming langauge for creating context free grammars representing SVG files. It is designed to be written on phone keyboards. To that tend it uses only the following characters:
- Letters a-z
- Numers 0-9
- Period `.`
- Question Mark `.`
- Whitespace characters for separating terms

### Examples

```
circle
```

```
circle v 0.5
circle p 0.5 h 120 v 0.5
```

```
myshape
rul myshape
circle v 0.5
myshape p 0.75 h 40
end
```

```
let blackrot 15
let whiterot 15
let hue 90

blackshape h 80 r 15 p 0.75 c 0.3

rul blackshape
square v 0.5 h 180
whiteshape p 0.5 x m0.5 y m0.5 r?whiterot h?hue
whiteshape p 0.5 x 0.5 y 0.5 r?whiterot 
end

rul whiteshape
square v 0.5
blackshape p 0.5 x m0.5 y m0.5 r?blackrot 
blackshape p 0.5 x 0.5 y 0.5 r?blackrot h?hue
end
```

```
eye p 0.2 xm0.3ym0.3
eye p 0.2 x0.3ym0.3

rul eye
circle w 0.5 l 0.9 v 0.9 r  90
circle w 0.5 l 0.9 p 0.5 v 0.8 h 235
```

```
let hue 40
pascal
rul pascal
triangle v0.5
pascal h ?hue p 0.5 ym0.5
pascal h ?hue p 0.5 y0.25 x0.5
pascal h ?hue p 0.5 y0.25 xm0.5
```
### Primitives

- `Circle`
- `triangle`
- `rtriangle`
- `square`
- `pentagon`
- `hexagon`
- `heptagon`
- `octagon`
- `nonagon`
- `decagon`
- `undecagon`
- `dodecagon`


### Properties

| Name | Key | Range | Description |
|---|---|---|---|
|Proportion|`p`|`0..`|The scale of this element. If 0.5, this element will be half the size of its parent. Elements with p 0 will be culled.|
|Length|`l`|`0..`|The scale of this element in the y axis. |
|Width|`w`|`0..`|The scale of this element in the x axis. |
|Curvature|`c`|`0..1`|How rounded the corners of polygons will be. |
|X|`x`|`..`|If 1.0, the x coordinate of the center of this element will be on the border of its parent.|
|Y|`y`|`..`|If 1.0, the y coordinate of the center of this element will be on the border of its parent.|
|Rotation|`r`|`0..360`|The rotation of this element around the x axis. |
|Hue|`h`|`0..360`|Affects the color. If the parent is green, and this is 120, it will be blue.|
|Saturation|`s`|`0..1`|The color saturation.|
|Value|`v`|`0..1`|The color lightness. 0 for black, 1 for white. |
|Alpha|`a`|`0..1`|The color alpha. Elements with a 0 will be culled. |




Please enjoy and share! 
Mark