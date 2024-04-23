#[derive(Debug, Clone)]
pub enum Msg {
    Quit,
    SwitchPane,
    ToggleHelp,

    ListPaneSelectNext,
    ListPaneSelectPrev,

    NotifyInfo(String),
    NotifyWarn(String),
    NotifyError(String),

    ToolPaneSelectUuidPage,
    ToolPaneSelectHashPage,
    ToolPaneSelectUnixTimePage,
    ToolPaneSelectNumberBasePage,

    UuidPageSelectNextItem,
    UuidPageSelectPrevItem,
    UuidPageCurrentItemSelectNext,
    UuidPageCurrentItemSelectPrev,
    UuidPageGenerate,
    UuidPageCopy,
    UuidPagePaste,

    HashPageSelectNextItem,
    HashPageSelectPrevItem,
    HashPageCurrentItemSelectNext,
    HashPageCurrentItemSelectPrev,
    HashPageCopy,
    HashPagePaste,

    UnixTimePageSelectNextItem,
    UnixTimePageSelectPrevItem,
    UnixTimePageCurrentItemSelectNext,
    UnixTimePageCurrentItemSelectPrev,
    UnixTimePageEditStart,
    UnixTimePageEditEnd,
    UnixTimePageEditKeyEvent(crossterm::event::KeyEvent),
    UnixTimePageCopy,
    UnixTimePagePaste,
}
