## Bugs
- [ ] Polygons are not centred on the origin
- [ ] `c` does not currently work for polygons
- [ ] Flip y coordinates
- [ ] Operator precedence

## Language Features

- [x] Length and width `l` and `w`
- [x] Curvature `c`
- [x] Allow probabilities for rules 
- [ ] Add ranges for animation. Also compound ranges, time `t` property
- [x] More primitives `tri` `rtri` `pent` `hex`
- [ ] `b` to draw strokes rather than fill
- [x] Rule probabilities should be cascading

- [x] Support math operators `sub` `add` `mul` `div` `abs`
- [x] Support comparison operators `eq` `neq` `lt` `gt` `leq` `geq`
- [x] Support logic operators `and` `or` `xor` `not`
- [ ] Support conditionals `if` `elif`  `else` `fi`
- [x] Property access in rule probabilities e.g. grow ?d <= 10
- [x] Property acces in invocations e.g. square r-?r

- [ ] Touch / Click events


## Code Editor

- [x] Editor for prop values
- [ ] Syntax highlighting
- [ ] Diagnostics
- [ ] Code Completion
- [ ] Hover
- [ ] Rename
- [ ] Duplicate Line
- [ ] Evolution editor

- [x] Settings Panel
- [x] Examples select
- [x] Save Creations
- [ ] Show preview behind code window
- [ ] Export SVG
- [ ] Css Classes

## Other

- [ ] Use anyhow for error handling
- [ ] Stop making everything public and in the prelude