import wildflowerImage from '../../assets/wildflower-tile.png';

interface CircularTileProps {
  onClick?: () => void;
  className?: string;
  size?: 'small' | 'medium' | 'large';
}

export default function CircularTile({ onClick, className = '', size = 'medium' }: CircularTileProps) {
  const sizeClasses = {
    small: 'w-16 h-16',
    medium: 'w-20 h-20',
    large: 'w-24 h-24',
  };

  return (
    <button
      onClick={onClick}
      className={`
        ${sizeClasses[size]}
        rounded-full
        transition-all duration-300 ease-out
        transform hover:scale-105 active:scale-95
        shadow-lg hover:shadow-xl
        border-2 border-white/50
        relative overflow-hidden
        group
        ${className}
      `}
      style={{
        backgroundImage: `url(${wildflowerImage})`,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
      }}
    >
      {/* Hover overlay effect */}
      <div className="absolute inset-0 bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-full" />
      
      {/* Subtle shine effect */}
      <div className="absolute top-1 left-1 w-2 h-2 bg-white/30 rounded-full opacity-60" />
      
      {/* Optional overlay for better contrast if needed */}
      <div className="absolute inset-0 bg-gradient-to-br from-transparent via-transparent to-black/5 rounded-full" />
    </button>
  );
}