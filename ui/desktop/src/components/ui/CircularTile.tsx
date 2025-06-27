import { useState } from 'react';

interface CircularTileProps {
  onClick?: () => void;
  className?: string;
  size?: 'small' | 'medium' | 'large';
}

export default function CircularTile({ onClick, className = '', size = 'medium' }: CircularTileProps) {
  const [isHovered, setIsHovered] = useState(false);

  const sizeClasses = {
    small: 'w-16 h-16',
    medium: 'w-20 h-20',
    large: 'w-24 h-24',
  };

  return (
    <button
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      className={`
        ${sizeClasses[size]}
        rounded-full
        bg-gradient-to-br from-orange-200 via-pink-200 to-blue-200
        hover:from-orange-300 hover:via-pink-300 hover:to-blue-300
        transition-all duration-300 ease-out
        transform hover:scale-105 active:scale-95
        shadow-lg hover:shadow-xl
        border-2 border-white/50
        relative overflow-hidden
        group
        ${className}
      `}
      style={{
        background: isHovered 
          ? 'linear-gradient(135deg, #fed7aa 0%, #fecaca 25%, #fde68a 50%, #a7f3d0 75%, #bfdbfe 100%)'
          : 'linear-gradient(135deg, #fed7aa 0%, #fecaca 25%, #fde68a 50%, #a7f3d0 75%, #bfdbfe 100%)',
      }}
    >
      {/* Underwater/Coral Scene */}
      <div className="absolute inset-0 flex items-center justify-center">
        {/* Coral/Seaweed elements */}
        <div className="relative w-full h-full">
          {/* Bottom coral base */}
          <div className="absolute bottom-1 left-2 w-3 h-4 bg-gradient-to-t from-orange-400 to-orange-300 rounded-t-full transform rotate-12 opacity-80" />
          <div className="absolute bottom-1 right-2 w-2 h-3 bg-gradient-to-t from-pink-400 to-pink-300 rounded-t-full transform -rotate-12 opacity-80" />
          <div className="absolute bottom-1 left-1/2 transform -translate-x-1/2 w-2 h-5 bg-gradient-to-t from-green-400 to-green-300 rounded-t-full opacity-80" />
          
          {/* Middle layer coral */}
          <div className="absolute bottom-2 left-3 w-2 h-3 bg-gradient-to-t from-red-400 to-red-300 rounded-full opacity-70" />
          <div className="absolute bottom-2 right-3 w-1 h-4 bg-gradient-to-t from-purple-400 to-purple-300 rounded-t-full opacity-70" />
          
          {/* Small fish/bubbles */}
          <div className="absolute top-3 left-3 w-1 h-1 bg-yellow-400 rounded-full opacity-90 animate-pulse" />
          <div className="absolute top-4 right-4 w-1 h-1 bg-blue-400 rounded-full opacity-80 animate-pulse" style={{ animationDelay: '0.5s' }} />
          <div className="absolute top-6 left-1/2 w-1 h-1 bg-green-400 rounded-full opacity-85 animate-pulse" style={{ animationDelay: '1s' }} />
          
          {/* Central decorative element */}
          <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
            <div className="w-3 h-3 bg-gradient-to-br from-yellow-300 to-orange-300 rounded-full opacity-90 animate-pulse" />
          </div>
        </div>
      </div>

      {/* Hover overlay effect */}
      <div className="absolute inset-0 bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-full" />
      
      {/* Subtle shine effect */}
      <div className="absolute top-1 left-1 w-2 h-2 bg-white/30 rounded-full opacity-60" />
    </button>
  );
}