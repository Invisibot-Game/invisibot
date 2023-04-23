import styles from "./ErrorHeader.module.scss";

export const ErrorHeader = ({ error }: { error: string }) => {
  return <header className={styles.errorHeader}>Error {error}</header>;
};
