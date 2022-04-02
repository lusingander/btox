package ui

import (
	"fmt"
	"strings"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/lusingander/btox/internal/app"
	"github.com/lusingander/btox/internal/uuid"
	"github.com/muesli/termenv"
)

var (
	uuidPageItemStyle = lipgloss.NewStyle().
				Padding(1, 2)

	uuidPageSelectedItemColorStyle = lipgloss.NewStyle().
					Foreground(selectedColor).
					Bold(true)

	uuidPageDisabledItemColorStyle = lipgloss.NewStyle().
					Foreground(disabledColor)
)

const (
	uuidPageCountMin = 1
	uuidPageCountMax = 100
)

type uuidPageModel struct {
	idView viewport.Model

	delegateKeys uuidPageDelegateKeyMap

	dash     bool
	upper    bool
	version  uuidVersion
	count    int
	selected uuidPageSelectableItems

	ids []string

	width, height int
}

type uuidVersion int

const (
	uuidVersion4 uuidVersion = iota
)

type uuidPageSelectableItems int

const (
	uuidPageSelectableDash uuidPageSelectableItems = iota
	uuidPageSelectableUpper
	uuidPageSelectableVersion
	uuidPageSelectableCount
	uuidPageSelectableGenerate
	uuidPageSelectableNumberOfItems // not item
)

func newUuidPageModel() uuidPageModel {
	m := uuidPageModel{}
	m.idView = viewport.New(0, 0)
	m.delegateKeys = newUuidPageDelegateKeyMap()
	m.reset()
	return m
}

type uuidPageDelegateKeyMap struct {
	enter, back, tab, shiftTab, h, j, k, l, c, x key.Binding
}

func newUuidPageDelegateKeyMap() uuidPageDelegateKeyMap {
	return uuidPageDelegateKeyMap{
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
		c: key.NewBinding(
			key.WithKeys("c", "y"),
			key.WithHelp("c", ""),
		),
		x: key.NewBinding(
			key.WithKeys("x"),
			key.WithHelp("x", ""),
		),
	}
}

func (m *uuidPageModel) setSize(w, h int) {
	m.width, m.height = w, h

	m.idView.Width = w
	m.idView.Height = h - lipgloss.Height(m.menuView()) - lipgloss.Height(m.separetorView()) - 1
}

func (m *uuidPageModel) reset() {
	m.dash = true
	m.upper = false
	m.version = uuidVersion4
	m.count = 1
	m.selected = uuidPageSelectableGenerate
	m.ids = nil
	m.idView.GotoTop()
}

func (m *uuidPageModel) selectItem(reverse bool) {
	n := uuidPageSelectableNumberOfItems
	if reverse {
		m.selected = ((m.selected-1)%n + n) % n
	} else {
		m.selected = (m.selected + 1) % n
	}
}

func (m uuidPageModel) copyIds() {
	s := ""
	for _, id := range m.ids {
		s += id + "\n"
	}
	// fixme: err
	_ = app.CopyToClipboard(s)
}

func (m *uuidPageModel) switchSelectedItem(left bool) tea.Cmd {
	switch m.selected {
	case uuidPageSelectableDash:
		m.dash = !m.dash
		return uuidPageFormat
	case uuidPageSelectableUpper:
		m.upper = !m.upper
		return uuidPageFormat
	case uuidPageSelectableCount:
		if left && m.count > uuidPageCountMin {
			m.count--
		} else if !left && m.count < uuidPageCountMax {
			m.count++
		}
		return nil
	default:
		// do nothing
		return nil
	}
}

func (m *uuidPageModel) generate() {
	// fixme: err
	m.ids, _ = uuid.GenerateVersion4(m.count, m.dash, m.upper)
}

func (m *uuidPageModel) format() {
	m.ids = uuid.FormatIds(m.ids, m.dash, m.upper)
}

func (m *uuidPageModel) updateContent() {
	s := ""
	for _, id := range m.ids {
		s += id + "\n"
	}
	m.idView.SetContent(s)
}

func (m *uuidPageModel) edit() {
	// fixme: err
	before := strings.Join(m.ids, "\n")
	after, _ := app.EditInEditor(before)
	m.ids = strings.Split(after, "\n")
}

func (m uuidPageModel) Init() tea.Cmd {
	return nil
}

type uuidPageGenerateMsg struct{}

func uuidPageGenerate() tea.Msg {
	return uuidPageGenerateMsg{}
}

type uuidPageFormatMsg struct{}

func uuidPageFormat() tea.Msg {
	return uuidPageFormatMsg{}
}

func (m uuidPageModel) Update(msg tea.Msg) (uuidPageModel, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, m.delegateKeys.enter):
			if m.selected == uuidPageSelectableGenerate {
				return m, uuidPageGenerate
			}
			return m, nil
		case key.Matches(msg, m.delegateKeys.back):
			return m, goBack
		case key.Matches(msg, m.delegateKeys.tab):
			m.selectItem(false)
			return m, nil
		case key.Matches(msg, m.delegateKeys.shiftTab):
			m.selectItem(true)
			return m, nil
		case key.Matches(msg, m.delegateKeys.h):
			cmd := m.switchSelectedItem(true)
			return m, cmd
		case key.Matches(msg, m.delegateKeys.l):
			cmd := m.switchSelectedItem(false)
			return m, cmd
		case key.Matches(msg, m.delegateKeys.j):
			m.selectItem(false)
			return m, nil
		case key.Matches(msg, m.delegateKeys.k):
			m.selectItem(true)
			return m, nil
		case key.Matches(msg, m.delegateKeys.c):
			m.copyIds()
			return m, nil
		case key.Matches(msg, m.delegateKeys.x):
			m.edit()
			m.updateContent()
			termenv.ClearScreen()
			termenv.AltScreen()
			return m, tea.Batch(redraw, tea.HideCursor)
		}
	case selectUuidMenuMsg:
		m.reset()
		m.updateContent()
		return m, nil
	case uuidPageGenerateMsg:
		m.generate()
		m.updateContent()
		return m, nil
	case uuidPageFormatMsg:
		m.format()
		m.updateContent()
		return m, nil
	}
	var cmd tea.Cmd
	m.idView, cmd = m.idView.Update(msg)
	return m, cmd
}

func (m uuidPageModel) View() string {
	menu := m.menuView()
	sep := m.separetorView()
	return lipgloss.JoinVertical(0, menu, sep, m.idView.View())
}

func (m uuidPageModel) menuView() string {
	s := ""

	var dash, upper, version string
	if m.dash {
		dash = "  With dash "
	} else {
		dash = "Without dash"
	}
	if m.upper {
		upper = "  Uppercase "
	} else {
		upper = "  Lowercase "
	}
	switch m.version {
	case uuidVersion4:
		version = "  Version 4 "
	}
	count := fmt.Sprintf("     %3d    ", m.count)

	s += uuidPageItemStyle.Render(m.withStyle(dash, m.selected == uuidPageSelectableDash, false, false))
	s += uuidPageItemStyle.Render(m.withStyle(upper, m.selected == uuidPageSelectableUpper, false, false))
	s += uuidPageItemStyle.Render(m.withStyle(version, m.selected == uuidPageSelectableVersion, true, true))
	s += uuidPageItemStyle.Render(m.withStyle(count, m.selected == uuidPageSelectableCount, m.count <= uuidPageCountMin, m.count >= uuidPageCountMax))

	generate := "    Generate    "
	if m.selected == uuidPageSelectableGenerate {
		generate = uuidPageSelectedItemColorStyle.Render(generate)
	}
	s += uuidPageItemStyle.Render(generate)

	return s
}

func (m uuidPageModel) separetorView() string {
	sep := strings.Repeat("-", m.width)
	return uuidPageDisabledItemColorStyle.Render(sep)
}

func (uuidPageModel) withStyle(s string, selected, first, last bool) string {
	l := "<"
	r := ">"
	if first {
		l = uuidPageDisabledItemColorStyle.Render(l)
	} else if selected {
		l = uuidPageSelectedItemColorStyle.Render(l)
	}
	if last {
		r = uuidPageDisabledItemColorStyle.Render(r)
	} else if selected {
		r = uuidPageSelectedItemColorStyle.Render(r)
	}
	if selected {
		s = uuidPageSelectedItemColorStyle.Render(s)
	}
	return fmt.Sprintf("%s %s %s", l, s, r)
}
