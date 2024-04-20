#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Quit,
    SwitchPane,
    ToggleHelp,

    ListPaneSelectNext,
    ListPaneSelectPrev,

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
