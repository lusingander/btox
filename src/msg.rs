#[derive(Debug, Clone)]
pub enum Msg {
    Quit,
    SwitchPane,

    ListPaneSelectNext,
    ListPaneSelectPrev,

    NotifyInfo(String),
    NotifyWarn(String),
    NotifyError(String),

    ToolPaneSelectUuidPage,
    ToolPaneSelectUlidPage,
    ToolPaneSelectHashPage,
    ToolPaneSelectUnixTimePage,
    ToolPaneSelectNumberBasePage,

    Page(PageMsg),
}

#[derive(Debug, Clone)]
pub enum PageMsg {
    Uuid(UuidMsg),
    Ulid(UlidMsg),
    Hash(HashMsg),
    UnixTime(UnixTimeMsg),
    NumberBase(NumberBaseMsg),
}

#[derive(Debug, Clone)]
pub enum UuidMsg {
    SelectNextItem,
    SelectPrevItem,
    CurrentItemSelectNext,
    CurrentItemSelectPrev,
    ScrollDown,
    ScrollUp,
    Generate,
    Copy,
    Paste,
}

#[derive(Debug, Clone)]
pub enum UlidMsg {
    SelectNextItem,
    SelectPrevItem,
    CurrentItemSelectNext,
    CurrentItemSelectPrev,
    ScrollDown,
    ScrollUp,
    Generate,
    Copy,
    Paste,
}

#[derive(Debug, Clone)]
pub enum HashMsg {
    SelectNextItem,
    SelectPrevItem,
    CurrentItemSelectNext,
    CurrentItemSelectPrev,
    ScrollDown,
    ScrollUp,
    Copy,
    Paste,
}

#[derive(Debug, Clone)]
pub enum UnixTimeMsg {
    SelectNextItem,
    SelectPrevItem,
    CurrentItemSelectNext,
    CurrentItemSelectPrev,
    EditStart,
    EditEnd,
    EditKeyEvent(ratatui::crossterm::event::KeyEvent),
    Copy,
    Paste,
}

#[derive(Debug, Clone)]
pub enum NumberBaseMsg {
    SelectNextItem,
    SelectPrevItem,
    CurrentItemSelectNext,
    CurrentItemSelectPrev,
    EditStart,
    EditEnd,
    EditKeyEvent(ratatui::crossterm::event::KeyEvent),
    Copy,
    Paste,
}
