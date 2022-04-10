package ui

import (
	"bytes"
	"fmt"
	"strconv"
	"strings"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/textinput"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/lusingander/btox/internal/color"
)

var (
	colorPageItemStyle = lipgloss.NewStyle().
				Padding(1, 2)

	colorPageSelectedItemColorStyle = lipgloss.NewStyle().
					Foreground(selectedColor).
					Bold(true)

	colorPageDisabledItemColorStyle = lipgloss.NewStyle().
					Foreground(disabledColor)
)

const (
	colorPageCountMin = 1
	colorPageCountMax = 10
)

type colorPageModel struct {
	listView   viewport.Model
	colorInput textinput.Model

	delegateKeys colorPageDelegateKeyMap

	selected colorPageSelectableItems
	dist     colorDistance
	count    int

	width, height int
}

type colorDistance int

const (
	colorDistanceRGB colorDistance = iota
	colorDistanceCIE76
	colorDistanceCIE94
	colorDistanceCIEDE2000
	colorDistanceNumberOfItems // not item
)

type colorPageSelectableItems int

const (
	colorPageSelectableInput colorPageSelectableItems = iota
	colorPageSelectableDistance
	colorPageSelectableCount
	colorPageSelectableFilter
	colorPageSelectableNumberOfItems // not item
)

func colorPageColorListContent(cols []color.Color) string {
	var buf bytes.Buffer
	for _, col := range cols {
		rect := lipgloss.NewStyle().
			Width(6).
			Background(lipgloss.Color(strconv.Itoa(col.ID)))
		buf.WriteString(fmt.Sprintf(" %3d  %s  %s\n", col.ID, rect, col.Hex))
	}
	return buf.String()
}

func newColorPageModel() colorPageModel {
	m := colorPageModel{}
	m.listView = viewport.New(0, 0)
	m.colorInput = textinput.New()
	m.colorInput.CharLimit = 6
	m.colorInput.Placeholder = "000000"
	m.colorInput.Prompt = ">  "
	m.colorInput.PromptStyle = colorPageSelectedItemColorStyle
	m.delegateKeys = newColorPageDelegateKeyMap()
	m.reset()
	return m
}

type colorPageDelegateKeyMap struct {
	enter, back, tab, shiftTab, h, l, j, k key.Binding
}

func newColorPageDelegateKeyMap() colorPageDelegateKeyMap {
	return colorPageDelegateKeyMap{
		enter: key.NewBinding(
			key.WithKeys("enter"),
			key.WithHelp("enter", ""),
		),
		back: key.NewBinding(
			key.WithKeys("backspace", "ctrl+h"),
			key.WithHelp("backspace", ""),
		),
		tab: key.NewBinding(
			key.WithKeys("tab"),
			key.WithHelp("tab", ""),
		),
		shiftTab: key.NewBinding(
			key.WithKeys("shift+tab"),
			key.WithHelp("shift+tab", ""),
		),
		h: key.NewBinding(
			key.WithKeys("h", "left"),
			key.WithHelp("h", ""),
		),
		l: key.NewBinding(
			key.WithKeys("l", "right"),
			key.WithHelp("l", ""),
		),
		j: key.NewBinding(
			key.WithKeys("j", "down"),
			key.WithHelp("j", ""),
		),
		k: key.NewBinding(
			key.WithKeys("k", "up"),
			key.WithHelp("k", ""),
		),
	}
}

func (m *colorPageModel) setSize(w, h int) {
	m.width, m.height = w, h

	m.listView.Width = w
	m.listView.Height = h - lipgloss.Height(m.menuView()) - lipgloss.Height(m.separetorView()) - 1
}

func (m *colorPageModel) reset() {
	m.selected = colorPageSelectableInput
	m.dist = colorDistanceCIEDE2000
	m.count = 5
	m.colorInput.Reset()
	m.colorInput.Focus()
	m.listView.SetContent(colorPageColorListContent(color.Cols))
	m.listView.GotoTop()
}

func (m *colorPageModel) selectItem(reverse bool) {
	n := colorPageSelectableNumberOfItems
	if reverse {
		m.selected = ((m.selected-1)%n + n) % n
	} else {
		m.selected = (m.selected + 1) % n
	}
	if m.selected == colorPageSelectableInput {
		m.colorInput.Focus()
		m.colorInput.PromptStyle = colorPageSelectedItemColorStyle
	} else {
		m.colorInput.Blur()
		m.colorInput.PromptStyle = lipgloss.Style{}
	}
}

func (m *colorPageModel) switchSelectedItem(left bool) {
	switch m.selected {
	case colorPageSelectableDistance:
		n := colorDistanceNumberOfItems
		if left {
			m.dist = ((m.dist-1)%n + n) % n
		} else {
			m.dist = (m.dist + 1) % n
		}
	case colorPageSelectableCount:
		if left && m.count > colorPageCountMin {
			m.count--
		} else if !left && m.count < colorPageCountMax {
			m.count++
		}
	default:
		// do nothing
	}
}

func (m *colorPageModel) filter() {
	var d color.Distance
	switch m.dist {
	case colorDistanceRGB:
		d = color.RGB
	case colorDistanceCIE76:
		d = color.CIE76
	case colorDistanceCIE94:
		d = color.CIE94
	case colorDistanceCIEDE2000:
		d = color.CIEDE2000
	}
	filtered := color.FilterColor(m.colorInput.Value(), d, m.count)
	if filtered == nil {
		m.listView.SetContent(colorPageColorListContent(color.Cols))
		return
	}
	m.listView.SetContent(colorPageColorListContent(filtered))
}

func (m colorPageModel) Init() tea.Cmd {
	return nil
}

type colorPageFilterMsg struct{}

func colorPageFilter() tea.Msg {
	return colorPageFilterMsg{}
}

func (m colorPageModel) Update(msg tea.Msg) (colorPageModel, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, m.delegateKeys.enter):
			if m.selected == colorPageSelectableFilter {
				return m, colorPageFilter
			}
			return m, nil
		case key.Matches(msg, m.delegateKeys.back):
			if m.selected != colorPageSelectableInput {
				return m, goBack
			}
		case key.Matches(msg, m.delegateKeys.tab):
			m.selectItem(false)
			return m, nil
		case key.Matches(msg, m.delegateKeys.shiftTab):
			m.selectItem(true)
			return m, nil
		case key.Matches(msg, m.delegateKeys.h):
			m.switchSelectedItem(true)
			return m, nil
		case key.Matches(msg, m.delegateKeys.l):
			m.switchSelectedItem(false)
			return m, nil
		case key.Matches(msg, m.delegateKeys.j):
			m.selectItem(false)
			return m, nil
		case key.Matches(msg, m.delegateKeys.k):
			m.selectItem(true)
			return m, nil
		}
	case selectColorMenuMsg:
		m.reset()
		return m, nil
	case colorPageFilterMsg:
		m.filter()
		return m, nil
	}
	var cmd tea.Cmd
	if m.selected == colorPageSelectableInput {
		m.colorInput, cmd = m.colorInput.Update(msg)
		return m, cmd
	} else {
		m.listView, cmd = m.listView.Update(msg)
		return m, cmd
	}
}

func (m colorPageModel) View() string {
	menu := m.menuView()
	sep := m.separetorView()
	return lipgloss.JoinVertical(0, menu, sep, m.listView.View())
}

func (m colorPageModel) menuView() string {
	s := ""

	input := ""
	input += m.colorInput.View()
	s += colorPageItemStyle.Render(input)

	var dist string
	switch m.dist {
	case colorDistanceRGB:
		dist = "    RGB    "
	case colorDistanceCIE76:
		dist = "   CIE76   "
	case colorDistanceCIE94:
		dist = "   CIE94   "
	case colorDistanceCIEDE2000:
		dist = " CIEDE2000 "
	}
	s += colorPageItemStyle.Render(m.withStyle(dist, m.selected == colorPageSelectableDistance, false, false))

	count := fmt.Sprintf("     %2d    ", m.count)
	s += colorPageItemStyle.Render(m.withStyle(count, m.selected == colorPageSelectableCount, m.count <= colorPageCountMin, m.count >= colorPageCountMax))

	filter := "     Filter    "
	if m.selected == colorPageSelectableFilter {
		filter = colorPageSelectedItemColorStyle.Render(filter)
	}
	s += colorPageItemStyle.Render(filter)

	return s
}

func (m colorPageModel) separetorView() string {
	sep := strings.Repeat("-", m.width)
	return colorPageDisabledItemColorStyle.Render(sep)
}

func (colorPageModel) withStyle(s string, selected, first, last bool) string {
	l := "<"
	r := ">"
	if first {
		l = colorPageDisabledItemColorStyle.Render(l)
	} else if selected {
		l = colorPageSelectedItemColorStyle.Render(l)
	}
	if last {
		r = colorPageDisabledItemColorStyle.Render(r)
	} else if selected {
		r = colorPageSelectedItemColorStyle.Render(r)
	}
	if selected {
		s = colorPageSelectedItemColorStyle.Render(s)
	}
	return fmt.Sprintf("%s %s %s", l, s, r)
}
