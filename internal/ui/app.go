package ui

import (
	"strings"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/lusingander/btox/internal/app"
)

var (
	headerStyle = lipgloss.NewStyle().
		Foreground(lipgloss.Color("39")).
		Padding(0, 1)
)

type page interface {
	crumb() string
}

type menuPage struct{}

func (menuPage) crumb() string { return app.Name }

type uuidPage struct{}

func (uuidPage) crumb() string { return menuPageUuidMenu }

type hashPage struct{}

func (hashPage) crumb() string { return menuPageHashMenu }

type colorPage struct{}

func (colorPage) crumb() string { return menuPageColorMenu }

type pageStack struct {
	stack []page
}

func (s pageStack) crumbs() []string {
	ret := make([]string, len(s.stack))
	for i, p := range s.stack {
		ret[i] = p.crumb()
	}
	return ret
}

func newPageStack(p page) *pageStack {
	return &pageStack{
		stack: []page{p},
	}
}

func (s *pageStack) pushPage(p page) {
	s.stack = append(s.stack, p)
}

func (s *pageStack) popPage() page {
	l := len(s.stack)
	if l <= 1 {
		return nil
	}
	p := s.stack[l-1]
	s.stack = s.stack[:l-1]
	return p
}

func (s *pageStack) currentPage() page {
	return s.stack[len(s.stack)-1]
}

type model struct {
	*pageStack

	menuPage  menuPageModel
	uuidPage  uuidPageModel
	hashPage  hashPageModel
	colorPage colorPageModel

	width, height int
}

var _ tea.Model = (*model)(nil)

func newModel() model {
	return model{
		pageStack: newPageStack(menuPage{}),
		menuPage:  newMenuPageModel(),
		uuidPage:  newUuidPageModel(),
		hashPage:  newHashPageModel(),
		colorPage: newColorPageModel(),
	}
}

func (m *model) setSize(w, h int) {
	m.width, m.height = w, h
	h = h - 1
	m.menuPage.setSize(w, h)
	m.uuidPage.setSize(w, h)
	m.hashPage.setSize(w, h)
	m.colorPage.setSize(w, h)
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c":
			return m, tea.Quit
		}
	case tea.WindowSizeMsg:
		m.setSize(msg.Width, msg.Height)
	case selectUuidMenuMsg:
		m.pushPage(uuidPage{})
	case selectHashMenuMsg:
		m.pushPage(hashPage{})
	case selectColorMenuMsg:
		m.pushPage(colorPage{})
	case goBackMsg:
		m.popPage()
	case redrawMsg:
		return m, windowSize(m.width, m.height)
	}
	switch m.currentPage().(type) {
	case menuPage:
		m.menuPage, cmd = m.menuPage.Update(msg)
		return m, cmd
	case uuidPage:
		m.uuidPage, cmd = m.uuidPage.Update(msg)
		return m, cmd
	case hashPage:
		m.hashPage, cmd = m.hashPage.Update(msg)
		return m, cmd
	case colorPage:
		m.colorPage, cmd = m.colorPage.Update(msg)
		return m, cmd
	default:
		return m, nil
	}
}

func (m model) View() string {
	return lipgloss.JoinVertical(0, m.header(), m.content())
}

func (m model) header() string {
	bd := strings.Join(m.crumbs(), " > ")
	return headerStyle.Render(bd)
}

func (m model) content() string {
	switch m.currentPage().(type) {
	case menuPage:
		return m.menuPage.View()
	case uuidPage:
		return m.uuidPage.View()
	case hashPage:
		return m.hashPage.View()
	case colorPage:
		return m.colorPage.View()
	default:
		return "error... :("
	}
}

func Start() error {
	m := newModel()
	p := tea.NewProgram(m, tea.WithAltScreen())
	return p.Start()
}
