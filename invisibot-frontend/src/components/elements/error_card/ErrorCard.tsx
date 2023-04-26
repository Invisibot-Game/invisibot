export const ErrorCard = ({ error }: { error: string }) => {
  return (
    <div className={`card`}>
      <p>Error: {error}</p>
    </div>
  );
};
