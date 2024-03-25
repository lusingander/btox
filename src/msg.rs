#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Quit,
    SwitchPane,

    ListPaneSelectNext,
    ListPaneSelectPrev,

    ToolPaneSelectFooPage,
    ToolPaneSelectBarPage,
}
