import styles from "@/styles/Home.module.css";
import { GameBoard } from "@/components/views/game_board/GameBoard";
import { GetServerSideProps } from "next";
import { Api } from "@/api/Api";
import { Round } from "@/api/Round";
import { ErrorHeader } from "@/components/views/error_header/ErrorHeader";

type HomeProps = {
  rounds?: Round[];
  error?: string;
};

export default function Home({ rounds, error }: HomeProps) {
  if (error) {
    return <ErrorHeader error={error} />;
  }

  if (!rounds) {
    return <ErrorHeader error={"Failed to reach backend"} />;
  }

  return (
    <>
      <main className={styles.main}>
        <GameBoard rounds={rounds} />
      </main>
    </>
  );
}

export const getServerSideProps: GetServerSideProps = async (ctx) => {
  let response = await Api.game.get();
  if (!response.success) {
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
