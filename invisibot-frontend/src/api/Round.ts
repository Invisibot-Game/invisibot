import { Player } from "./Player";
import { TileType } from "./TileType";

export type Round = {
  players: Player[];
  width: number;
  height: number;
  tiles: TileType[];
};
