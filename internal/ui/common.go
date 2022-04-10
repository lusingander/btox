package ui

import (
	"fmt"
	"strings"

	"github.com/charmbracelet/lipgloss"
)

var (
	selectedColor = lipgloss.Color("117")

	disabledColor = lipgloss.Color("240")

	listNormalTitleColorStyle = lipgloss.NewStyle().
					Foreground(lipgloss.AdaptiveColor{Light: "#1a1a1a", Dark: "#dddddd"})

	listNormalItemStyle = lipgloss.NewStyle().
				Padding(0, 0, 0, 2)

	listNormalTitleStyle = listNormalTitleColorStyle.Copy().
				Padding(0, 0, 0, 2)

	listNormalDescColorStyle = lipgloss.NewStyle().
					Foreground(lipgloss.AdaptiveColor{Light: "#A49FA5", Dark: "#777777"})

	listNormalDescStyle = listNormalDescColorStyle.Copy().
				Padding(0, 0, 0, 2)

	listSelectedTitleColorStyle = lipgloss.NewStyle().
					Foreground(selectedColor)

	listSelectedItemStyle = lipgloss.NewStyle().
				Border(lipgloss.NormalBorder(), false, false, false, true).
				BorderForeground(selectedColor).
				Padding(0, 0, 0, 1)

	listSelectedTitleStyle = listSelectedItemStyle.Copy().
				Foreground(selectedColor)

	listSelectedDescColorStyle = listSelectedTitleColorStyle.Copy().
					Foreground(selectedColor)

	listSelectedDescStyle = listSelectedItemStyle.Copy().
				Foreground(selectedColor)

	itemStyle = lipgloss.NewStyle().
			Padding(1, 2)

	selectedItemColorStyle = lipgloss.NewStyle().
				Foreground(selectedColor).
				Bold(true)

	disabledItemColorStyle = lipgloss.NewStyle().
				Foreground(disabledColor)
)

func selectView(s string, selected, first, last bool) string {
	l := "<"
	r := ">"
	if first {
		l = disabledItemColorStyle.Render(l)
	} else if selected {
		l = selectedItemColorStyle.Render(l)
	}
	if last {
		r = disabledItemColorStyle.Render(r)
	} else if selected {
		r = selectedItemColorStyle.Render(r)
	}
	if selected {
		s = selectedItemColorStyle.Render(s)
	}
	return fmt.Sprintf("%s %s %s", l, s, r)
}

func separetorView(w int) string {
	sep := strings.Repeat("-", w)
	return disabledItemColorStyle.Render(sep)
}
