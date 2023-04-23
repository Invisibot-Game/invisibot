import styles from "./RoundBar.module.scss";

export type LoadingBarProps = {
  curr: number;
  total: number;
  setRound: (num: number) => void;
};

const ROUND_BAR_ID = "roundBar";
export const RoundBar = ({ curr, total, setRound }: LoadingBarProps) => {
  return (
    <div>
      <label htmlFor={ROUND_BAR_ID}> {`${curr}/${total}`} </label>
      <input
        id={ROUND_BAR_ID}
        type="range"
        min={1}
        max={total}
        value={curr}
        onChange={(e) => {
          const num = Number.parseInt(e.target.value, 10);
          setRound(num);
        }}
      />
    </div>
  );
};
