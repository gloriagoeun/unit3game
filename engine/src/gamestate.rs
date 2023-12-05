pub struct GameState {
    /* the different game states possible to match with later
       0 = Title Screen
       1 = Gameplay
       2 = Game Over / Player 1 Wins
       3 = You Win! / Player 2 Wins
       4 = Tie (Game 2)
    */
    pub state: usize,
}
