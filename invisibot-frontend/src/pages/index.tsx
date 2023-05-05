import styles from "@/styles/Home.module.css";
import { GameBoard } from "@/components/views/game_board/GameBoard";
import { GetServerSideProps } from "next";
import { Api } from "@/api/Api";
import { Round } from "@/api/Round";
import { GameControlDashboard } from "@/components/views/game_control_dashboard/GameControlDashboard";
import { ErrorCard } from "@/components/elements/error_card/ErrorCard";

type HomeProps = {
  rounds?: Round[];
  error?: string;
};

export default function Home({ rounds, error }: HomeProps) {
  return (
    <>
      <main className={styles.main}>
        {error ? (
          <ErrorCard error={error} />
        ) : !rounds ? (
          <ErrorCard error={"Failed to load rounds"} />
        ) : (
          <GameBoard rounds={rounds} />
        )}
        <GameControlDashboard />
      </main>
    </>
  );
}

export const getServerSideProps: GetServerSideProps = async (ctx) => {
  let response = await Api.game.get();
  if (!response.success) {
    console.error("?? Not sure what happened here", response);
    return {
      props: {
        error: response.error ?? "Something went wrong",
      },
    };
  }

  return {
    props: {
      rounds: response.data!!.rounds,
    },
  };
};
