package color

import (
	"sort"

	"github.com/lucasb-eyer/go-colorful"
)

type Color struct {
	ID  int
	Hex string
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
