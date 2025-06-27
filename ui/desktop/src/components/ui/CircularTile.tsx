import circularTileImage from '../../assets/circular-tile-bg.png';
import { Goose } from '../icons/Goose';

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
        border-2 border-white/50
        relative overflow-hidden
        group
        ${className}
      `}
      style={{
        backgroundImage: `url(${circularTileImage})`,
        backgroundSize: 'contain',
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
        boxShadow: '0px 2px 4px 0px rgba(255, 255, 255, 0.8) inset, 0px 8px 16px 0px rgba(255, 255, 255, 0.3) inset, -2px 2px 8px 0px rgba(255, 255, 255, 0.4) inset, 0px 4px 4px 0px rgba(0, 0, 0, 0.25)',
      }}
    >
      {/* Hover overlay effect */}
      <div className="absolute inset-0 bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-full" />
      
      {/* Subtle shine effect */}
      <div className="absolute top-1 left-1 w-2 h-2 bg-white/30 rounded-full opacity-60" />
      
      {/* 16x16 Goose logo in bottom right corner inside the circle */}
      <div className="absolute bottom-2 w-4 h-4 text-white/90 drop-shadow-md z-10" style={{ right: '11px' }}>
        <Goose className="w-4 h-4" />
      </div>
      
      {/* Optional overlay for better contrast if needed */}
      <div className="absolute inset-0 bg-gradient-to-br from-transparent via-transparent to-black/5 rounded-full" />
    </button>
  );
}