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
blackshape
rul blackshape
square v 0
whiteshape p 0.5 x m0.5 y m0.5
whiteshape p 0.5 x 0.5 y 0.5
end

rul whiteshape
square v 1
blackshape p 0.5 x m0.5 y m0.5
blackshape p 0.5 x 0.5 y 0.5
end
```


### Properties

| Name | ShortName | Default | Range | Wrapping | Description |
|---|---|---|---|---|---|
Proportion|`p`|`1.0`|`0.0..1.0`|`false`|The scale of this element. If 0.5, this element will be half the size of its parent. Elements with p 0 will be culled.|
|X Transform|`x`|`0.0`|Any number|`false`|If 1.0, the x coordinate of the center of this element will be on the border of its parent.|
|Y Transform|`y`|`0.0`|Any number|`false`|If 1.0, the y coordinate of the center of this element will be on the border of its parent.|
|Rotation|`r`|`0`|`0..360`|`true`|The rotation of this element around the x axis. |
|Hue|`h`|`0`|`0..360`|`true`|Affects the color. If the parent is green, and this is 120, it will be blue.|
|Saturation|`s`|`1.0`|`0.0..1.0`|`false`|The color saturation.|
|Value|`v`|`0.5`|`0.0..1.0`|`false`|The color lightness. 0 for black, 1 for white. |
|Alpha|`a`|`1.0`|`0.0..1.0`|`false`|The color alpha. Elements with a 0 will be culled. |




Please enjoy and share! 
Mark