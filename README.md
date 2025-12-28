# Matplotlib Rust Wrapper
It contains two matplotlibs API: `plt.plot` and `fig, axes = plt.subplots`

## Generic parameters
- `Line2D`:
  - alpha
  - antialiased
  - color
  - figure
  - label
  - linestyle
  - linewidth
  - marker
  - markeredgecolor
  - markeredgewidth
  - markerfacecolor
  - markerfacecoloralt
  - markersize
  - rasterized
  - zorder

## Functions supported:
- `plot`:
Required:
  - y
  - y
Optional:
  - fmt
  - scalex/scaley

- `scatter`:
- `hist`:
- `imshow`:
- `fillbetween`
- `bar`
- `axhline`
- `axvline`
- `text`
- `suptitle`

- `set_axis`
- `grid`
- `set_xlim`
- `set_ylim`
- `set_xlabel`
- `set_ylabel`
- `set_title`
- `legend`
- `set_xticks`
- `set_yticks`
- `set_xtickslabels`
- `set_ytickslabels`
- `sharex`
- `sharey`