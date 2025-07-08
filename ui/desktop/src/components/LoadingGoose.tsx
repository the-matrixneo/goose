import GooseLogo from './GooseLogo';

const LoadingGoose = () => {
  return (
    <div className="w-full animate-fade-slide-up">
      <div
        data-testid="loading-indicator"
        className="flex items-center gap-2 text-xs text-textStandard py-2"
      >
        <GooseLogo size="small" hover={false} />
        goose is working on it…
      </div>
    </div>
  );
};

export default LoadingGoose;
