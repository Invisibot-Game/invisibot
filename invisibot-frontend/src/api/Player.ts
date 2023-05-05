export type Player = {
  id: number;
  x: number;
  y: number;
  rotation: Direction;
  visible: boolean;
};

export type Direction = "Up" | "Down" | "Right" | "Left";
