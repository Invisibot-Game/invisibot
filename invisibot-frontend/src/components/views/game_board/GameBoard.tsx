import { Round } from "@/api/Round";
import styles from "./GameBoard.module.scss";
import { TileType } from "@/api/TileType";
import { Player } from "@/api/Player";
import { useEffect, useState } from "react";
import { RoundBar } from "@/components/elements/round_bar/RoundBar";

type GameBoardProps = {
  rounds: Round[];
};

export const GameBoard = ({ rounds }: GameBoardProps) => {
  const [round, setRound] = useState<number>(0);
  const [gameOver, setGameOver] = useState<boolean>(false);
  const [speed, setSpeed] = useState<number | null>(200);

  useEffect(() => {
    if (round < rounds.length - 1 && speed) {
      setTimeout(() => {
        setRound(round + 1);
      }, speed);
    }

    setGameOver(round >= rounds.length - 1);
  }, [round, speed]);

  const state: Round = rounds[round];

  return (
    <div className={styles.container}>
      <div className={styles.board}>
        {[...Array(state.width)].map((_, x) => (
          <div key={`row-${x}`} className={styles.gridRow}>
            {[...Array(state.height)].map((_, y) => {
              const index = x * state.height + y;
              const tile = state.tiles.at(index)!!;
              const tileStyle =
                tile === TileType.Wall ? styles.wall : styles.empty;

              const player = getPlayerAt(state, x, y);

              return (
                <div key={`tile-${x}-${y}`} className={styles.tile}>
                  <div className={tileStyle}>
                    <Player player={player} />
                  </div>
                </div>
              );
            })}
          </div>
        ))}
      </div>

      <div>
        <div className={styles.details}>
          <RoundBar
            curr={round + 1}
            total={rounds.length}
            setRound={(num) => {
              setRound(num - 1);
              setSpeed(null); // Pause the game
            }}
          />
          <p>{gameOver ? "Game over" : "Game running"}</p>
          <div className={styles.buttonGroup}>
            <button
              className={styles.speedButton}
              onClick={() => setSpeed(null)}
            >
              Paused
            </button>
            <button
              className={styles.speedButton}
              onClick={() => setSpeed(200)}
            >
              Play
            </button>
            <button
              className={styles.speedButton}
              onClick={() => setSpeed(100)}
            >
              Double speed
            </button>
            <button className={styles.speedButton} onClick={() => setSpeed(50)}>
              Triple speed
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

const Player = ({ player }: { player: Player | undefined }) => {
  const playerColor = getPlayerColor(player?.id);
  const invisible = player?.visible === false ? styles.invisible : "";
  if (playerColor) {
    return <div className={`${styles.player} ${playerColor} ${invisible}`} />;
  }

  return <div />;
};

function getPlayerColor(id?: number): string {
  switch (id) {
    case 0:
      return styles.player0;
    case 1:
      return styles.player1;
    case 2:
      return styles.player2;
    case 3:
      return styles.player3;
    case undefined:
      return "";
    default:
      console.error(`No color for playerId ${id}`);
      return "";
  }
}

function getPlayerAt(round: Round, x: number, y: number): Player | undefined {
  let players = round.players.filter((p) => p.x === x && p.y === y);

  if (players.length === 0) {
    return undefined;
  }

  if (players.length > 1) {
    console.error(`Multiple players (${players.length}) in the same position`);
  }

  return players[0];
}