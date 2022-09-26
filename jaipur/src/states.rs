pub enum AppState {
    MainMenu,
    InitGame,
    AiTurn,
    TurnTransition,
    InGame,
    WaitForTweensToFinish,
    GameOver,
}

pub enum TurnState {
    None,
    Take,
    Sell,
}
