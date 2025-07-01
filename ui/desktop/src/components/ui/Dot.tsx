export type LoadingStatus = 'loading' | 'success' | 'error';
export default function Dot({
  size,
  loadingStatus,
}: {
  size: number;
  loadingStatus: LoadingStatus;
}) {
  const backgroundColor =
    {
      loading: 'var(--primary)',
      success: 'var(--green-600)',
      error: 'var(--red-600)',
    }[loadingStatus] ?? 'var(--icon-extra-subtle)';

  return (
    <div
      className={`${
        loadingStatus === 'loading' ? 'animate-pulse' : ''
      } flex items-center justify-center`}
    >
      <div
        className="rounded-full"
        style={{
          width: `${size * 2}px`,
          height: `${size * 2}px`,
          backgroundColor: backgroundColor,
          boxShadow: loadingStatus === 'loading' ? '0 0 5px var(--primary-light)' : 'none',
        }}
      />
    </div>
  );
}
