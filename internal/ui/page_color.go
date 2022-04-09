package ui

import (
	"bytes"
	"fmt"
	"strconv"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/lusingander/btox/internal/color"
)

type colorPageModel struct {
	listView viewport.Model

	delegateKeys colorPageDelegateKeyMap

	width, height int
}

func colorPageColorListContent() string {
	var buf bytes.Buffer
	for _, col := range color.Cols {
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
	m.listView.SetContent(colorPageColorListContent())
	m.delegateKeys = newColorPageDelegateKeyMap()
	m.reset()
	return m
}

type colorPageDelegateKeyMap struct {
	back key.Binding
}

func newColorPageDelegateKeyMap() colorPageDelegateKeyMap {
	return colorPageDelegateKeyMap{
		back: key.NewBinding(
			key.WithKeys("backspace", "ctrl+h"),
			key.WithHelp("backspace", ""),
		),
	}
}

func (m *colorPageModel) setSize(w, h int) {
	m.width, m.height = w, h

	m.listView.Width = w
	m.listView.Height = h
}

func (m *colorPageModel) reset() {
	m.listView.GotoTop()
}

func (m colorPageModel) Init() tea.Cmd {
	return nil
}

func (m colorPageModel) Update(msg tea.Msg) (colorPageModel, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, m.delegateKeys.back):
			return m, goBack
		}
	case selectColorMenuMsg:
		m.reset()
		return m, nil
	}
	var cmd tea.Cmd
	m.listView, cmd = m.listView.Update(msg)
	return m, cmd
}

func (m colorPageModel) View() string {
	return m.listView.View()
}
