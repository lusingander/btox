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
    ToolPaneSelectFooPage,
    ToolPaneSelectBarPage,

    UuidPageSelectNextItem,
    UuidPageSelectPrevItem,
    UuidPageCurrentItemSelectNext,
    UuidPageCurrentItemSelectPrev,
    UuidPageGenerate,
    UuidPageCopy,
    UuidPagePaste,
}
