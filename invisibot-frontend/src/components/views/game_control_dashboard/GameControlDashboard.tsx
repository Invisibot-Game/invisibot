import { Api } from "@/api/Api";
import { useRouter } from "next/router";
import styles from "./GameControlDashboard.module.scss";
import { useState } from "react";

const PLAYER_SLIDER_ID = "playerSliderId";
const MAX_ROUNDS_SLIDER_ID = "maxRoundsSliderId";

export const GameControlDashboard = () => {
  const router = useRouter();
  const [numPlayers, setNumPlayers] = useState<number>(2);
  const [numRounds, setNumRounds] = useState<number>(10);

  return (
    <div className={styles.dashboard}>
      <div className={styles.dashboardRow}>
        <div className={styles.sliderContainer}>
          <label htmlFor={PLAYER_SLIDER_ID}>Players: {numPlayers}</label>
          <input
            id={PLAYER_SLIDER_ID}
            type="range"
            min={2}
            max={8}
            value={numPlayers}
            onChange={(e) => {
              const num = Number.parseInt(e.target.value, 10);
              setNumPlayers(num);
            }}
          />
        </div>
        <div className={styles.sliderContainer}>
          <label htmlFor={MAX_ROUNDS_SLIDER_ID}>Max rounds: {numRounds}</label>
          <input
            id={MAX_ROUNDS_SLIDER_ID}
            type="range"
            min={1}
            max={1000}
            value={numRounds}
            onChange={(e) => {
              const num = Number.parseInt(e.target.value, 10);
              setNumRounds(num);
            }}
          />
        </div>
      </div>
      <div className={styles.dashboardRow}>
        <button
          onClick={() =>
            Api.game
              .create(numPlayers, numRounds)
              .then(() => {
                router.reload();
              })
              .catch((err) => {
                alert(`Failed to create game ${err}`);
              })
          }
        >
          New game
        </button>
        <button
          onClick={() =>
            Api.game
              .delete()
              .then((resp) => {
                console.log("RESP", resp);
                alert("Game deleted");
                router.reload();
              })
              .catch((err) => {
                alert(`Failed to delete game ${err}`);
              })
          }
        >
          Delete game
        </button>
      </div>
    </div>
  );
};
