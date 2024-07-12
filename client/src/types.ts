export type InitMessage = {
  init: {
    field: string;
    player_count: number;
  };
};

export type GameStateMessage = {
  state: {
    players: Record<string, string>;
    ap: string;
  };
};
