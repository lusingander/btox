package color

import (
	"sort"

	"github.com/lucasb-eyer/go-colorful"
)

type Color struct {
	ID  int
	Hex string
	R   int
	G   int
	B   int
}

func (c Color) Name16() string {
	switch c.ID {
	case 0:
		return "Black"
	case 1:
		return "Red"
	case 2:
		return "Green"
	case 3:
		return "Yellow"
	case 4:
		return "Blue"
	case 5:
		return "Purple"
	case 6:
		return "Cyan"
	case 7:
		return "White"
	case 8:
		return "Black (Bright)"
	case 9:
		return "Red (Bright)"
	case 10:
		return "Green (Bright)"
	case 11:
		return "Yellow (Bright)"
	case 12:
		return "Blue (Bright)"
	case 13:
		return "Purple (Bright)"
	case 14:
		return "Cyan (Bright)"
	case 15:
		return "White (Bright)"
	default:
		return ""
	}
}

type Distance int

const (
	RGB Distance = iota
	CIE76
	CIE94
	CIEDE2000
)

type cd struct {
	c Color
	d float64
}

func FilterColor(hex string, dist Distance, n int) []Color {
	target, err := colorful.Hex("#" + hex)
	if err != nil {
		return nil
	}
	f := distanceFunc(target, dist)
	if f == nil {
		return nil
	}
	cds := make([]cd, len(Cols))
	for i, col := range Cols {
		c, _ := colorful.Hex(col.Hex)
		d := f(c)
		cds[i] = cd{col, d}
	}
	sort.Slice(cds, func(i, j int) bool { return cds[i].d < cds[j].d })
	ret := make([]Color, n)
	for i := 0; i < n; i++ {
		ret[i] = cds[i].c
	}
	return ret
}

func distanceFunc(c colorful.Color, dist Distance) func(colorful.Color) float64 {
	switch dist {
	case RGB:
		return c.DistanceRgb
	case CIE76:
		return c.DistanceCIE76
	case CIE94:
		return c.DistanceCIE94
	case CIEDE2000:
		return c.DistanceCIEDE2000
	}
	return nil
}
