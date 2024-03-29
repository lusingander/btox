#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Quit,
    SwitchPane,

    ListPaneSelectNext,
    ListPaneSelectPrev,

    ToolPaneSelectUuidPage,
    ToolPaneSelectFooPage,
    ToolPaneSelectBarPage,

    UuidPageSelectNextItem,
    UuidPageSelectPrevItem,
    UuidPageCurrentItemSelectNext,
    UuidPageCurrentItemSelectPrev,
}
