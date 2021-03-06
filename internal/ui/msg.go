package ui

import tea "github.com/charmbracelet/bubbletea"

type selectUuidMenuMsg struct{}

func selectUuidMenu() tea.Msg {
	return selectUuidMenuMsg{}
}

type selectHashMenuMsg struct{}

func selectHashMenu() tea.Msg {
	return selectHashMenuMsg{}
}

type selectColorMenuMsg struct{}

func selectColorMenu() tea.Msg {
	return selectColorMenuMsg{}
}

type goBackMsg struct{}

func goBack() tea.Msg {
	return goBackMsg{}
}

type redrawMsg struct{}

func redraw() tea.Msg {
	return redrawMsg{}
}

func windowSize(w, h int) tea.Cmd {
	return func() tea.Msg { return tea.WindowSizeMsg{Width: w, Height: h} }
}
