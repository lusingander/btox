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
    ToolPaneSelectFooPage,
    ToolPaneSelectBarPage,

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
}
