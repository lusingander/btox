package ui

import (
	"fmt"
	"strings"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/lusingander/btox/internal/app"
	"github.com/lusingander/btox/internal/hash"
	"github.com/muesli/reflow/wrap"
)

var (
	hashPageItemStyle = lipgloss.NewStyle().
				Padding(1, 2)

	hashPageSelectedItemColorStyle = lipgloss.NewStyle().
					Foreground(selectedColor).
					Bold(true)

	hashPageDisabledItemColorStyle = lipgloss.NewStyle().
					Foreground(disabledColor)
)

type hashPageModel struct {
	inputView  viewport.Model
	outputView viewport.Model

	delegateKeys hashPageDelegateKeyMap

	algo     hashAlgorithm
	selected hashPageSelectableItems

	input, output string

	width, height int
}

type hashAlgorithm int

const (
	md5 hashAlgorithm = iota
	sha1
	sha224
	sha256
	sha384
	sha512_224
	sha512_256
	sha512
	hashAlgorithmNumberOfItems // not item
)

type hashPageSelectableItems int

const (
	hashPageSelectableAlgorithm     hashPageSelectableItems = iota
	hashPageSelectableNumberOfItems                         // not item
)

func newHashPageModel() hashPageModel {
	m := hashPageModel{}
	m.inputView = viewport.New(0, 0)
	m.outputView = viewport.New(0, 0)
	m.delegateKeys = newHashPageDelegateKeyMap()
	m.reset()
	return m
}

type hashPageDelegateKeyMap struct {
	back, h, l, c, v key.Binding
}

func newHashPageDelegateKeyMap() hashPageDelegateKeyMap {
	return hashPageDelegateKeyMap{
		back: key.NewBinding(
			key.WithKeys("backspace", "ctrl+h"),
			key.WithHelp("backspace", ""),
		),
		h: key.NewBinding(
			key.WithKeys("h", "left"),
			key.WithHelp("h", ""),
		),
		l: key.NewBinding(
			key.WithKeys("l", "right"),
			key.WithHelp("l", ""),
		),
		c: key.NewBinding(
			key.WithKeys("c", "y"),
			key.WithHelp("c", ""),
		),
		v: key.NewBinding(
			key.WithKeys("v", "p"),
			key.WithHelp("v", ""),
		),
	}
}

func (m *hashPageModel) setSize(w, h int) {
	m.width, m.height = w, h

	m.outputView.Width = w
	m.outputView.Height = 2

	hh := h - lipgloss.Height(m.menuView()) - (lipgloss.Height(m.separetorView()) * 2) - 2
	m.inputView.Width = w
	m.inputView.Height = hh - 1
}

func (m *hashPageModel) reset() {
	m.algo = md5
	m.selected = hashPageSelectableAlgorithm
	m.input = ""
	m.output = ""
	m.inputView.GotoTop()
	m.outputView.GotoTop()
}

func (m *hashPageModel) updateContent() {
	switch m.algo {
	case md5:
		m.output = hash.MD5(m.input)
	case sha1:
		m.output = hash.SHA1(m.input)
	case sha224:
		m.output = hash.SHA224(m.input)
	case sha256:
		m.output = hash.SHA256(m.input)
	case sha384:
		m.output = hash.SHA384(m.input)
	case sha512_224:
		m.output = hash.SHA512_224(m.input)
	case sha512_256:
		m.output = hash.SHA512_256(m.input)
	case sha512:
		m.output = hash.SHA512(m.input)
	}

	m.inputView.SetContent(wrap.String(m.input, m.width))
	m.outputView.SetContent(wrap.String(m.output, m.width))
}

func (m *hashPageModel) switchSelectedItem(left bool) {
	switch m.selected {
	case hashPageSelectableAlgorithm:
		n := hashAlgorithmNumberOfItems
		if left {
			m.algo = ((m.algo-1)%n + n) % n
		} else {
			m.algo = (m.algo + 1) % n
		}
	default:
		// do nothing
	}
}

func (m hashPageModel) copy() {
	// fixme: err
	_ = app.CopyToClipboard(m.output)
}

func (m *hashPageModel) paste() {
	// fixme: err
	m.input, _ = app.PasteFromClipboard()
}

func (m hashPageModel) Init() tea.Cmd {
	return nil
}

type hashPageCalculateMsg struct{}

func hashPageCalculate() tea.Msg {
	return hashPageCalculateMsg{}
}

func (m hashPageModel) Update(msg tea.Msg) (hashPageModel, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, m.delegateKeys.back):
			return m, goBack
		case key.Matches(msg, m.delegateKeys.h):
			m.switchSelectedItem(true)
			return m, hashPageCalculate
		case key.Matches(msg, m.delegateKeys.l):
			m.switchSelectedItem(false)
			return m, hashPageCalculate
		case key.Matches(msg, m.delegateKeys.c):
			m.copy()
			return m, nil
		case key.Matches(msg, m.delegateKeys.v):
			m.paste()
			return m, hashPageCalculate
		}
	case tea.WindowSizeMsg:
		return m, hashPageCalculate
	case selectHashMenuMsg:
		m.reset()
		return m, hashPageCalculate
	case hashPageCalculateMsg:
		m.updateContent()
		return m, nil
	}
	var cmd tea.Cmd
	m.inputView, cmd = m.inputView.Update(msg)
	return m, cmd
}

func (m hashPageModel) View() string {
	menu := m.menuView()
	sep := m.separetorView()
	in := m.inputView.View()
	out := m.outputView.View()
	return lipgloss.JoinVertical(0, menu, sep, in, sep, out)
}

func (m hashPageModel) menuView() string {
	s := ""

	var algo string
	switch m.algo {
	case md5:
		algo = "     MD5     "
	case sha1:
		algo = "    SHA-1    "
	case sha224:
		algo = "   SHA-224   "
	case sha256:
		algo = "   SHA-256   "
	case sha384:
		algo = "   SHA-384   "
	case sha512_224:
		algo = " SHA-512/224 "
	case sha512_256:
		algo = " SHA-512/256 "
	case sha512:
		algo = "   SHA-512   "
	}

	s += hashPageItemStyle.Render(m.withStyle(algo, m.selected == hashPageSelectableAlgorithm, false, false))

	return s
}

func (m hashPageModel) separetorView() string {
	sep := strings.Repeat("-", m.width)
	return hashPageDisabledItemColorStyle.Render(sep)
}

func (hashPageModel) withStyle(s string, selected, first, last bool) string {
	l := "<"
	r := ">"
	if first {
		l = hashPageDisabledItemColorStyle.Render(l)
	} else if selected {
		l = hashPageSelectedItemColorStyle.Render(l)
	}
	if last {
		r = hashPageDisabledItemColorStyle.Render(r)
	} else if selected {
		r = hashPageSelectedItemColorStyle.Render(r)
	}
	if selected {
		s = hashPageSelectedItemColorStyle.Render(s)
	}
	return fmt.Sprintf("%s %s %s", l, s, r)
}
