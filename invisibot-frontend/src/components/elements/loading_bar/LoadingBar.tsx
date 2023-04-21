import styles from "./LoadingBar.module.scss";

export type LoadingBarProps = {
  curr: number;
  total: number;
};

export const LoadingBar = ({ curr, total }: LoadingBarProps) => {
  let percent = (curr / total) * 100;

  return (
    <div className={styles.loadingBarContainer}>
      <div className={styles.loadingBar} style={{ width: `${percent}%` }} />
    </div>
  );
};
