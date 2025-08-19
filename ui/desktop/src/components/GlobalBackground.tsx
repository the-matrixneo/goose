import React, { useState, useEffect } from 'react';

interface BackgroundOption {
  id: string;
  name: string;
  type: 'gradient' | 'image' | 'solid';
  value: string;
}

const DEFAULT_BACKGROUNDS: BackgroundOption[] = [
  {
    id: 'default-gradient',
    name: 'Default Gradient',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(100, 100, 110, 0.25) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(120, 120, 125, 0.22) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(90, 95, 100, 0.18) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(0, 0, 0, 0.02) 0%, 
        rgba(0, 0, 0, 0.04) 25%, 
        rgba(0, 0, 0, 0.02) 50%, 
        rgba(0, 0, 0, 0.06) 75%, 
        rgba(0, 0, 0, 0.03) 100%
      )
    `,
  },
  {
    id: 'blue-gradient',
    name: 'Ocean Blue',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(59, 130, 246, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(99, 102, 241, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(147, 197, 253, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(59, 130, 246, 0.02) 0%, 
        rgba(99, 102, 241, 0.04) 25%, 
        rgba(147, 197, 253, 0.02) 50%, 
        rgba(59, 130, 246, 0.06) 75%, 
        rgba(99, 102, 241, 0.03) 100%
      )
    `,
  },
  {
    id: 'purple-gradient',
    name: 'Purple Haze',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(147, 51, 234, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(168, 85, 247, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(196, 181, 253, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(147, 51, 234, 0.02) 0%, 
        rgba(168, 85, 247, 0.04) 25%, 
        rgba(196, 181, 253, 0.02) 50%, 
        rgba(147, 51, 234, 0.06) 75%, 
        rgba(168, 85, 247, 0.03) 100%
      )
    `,
  },
  {
    id: 'green-gradient',
    name: 'Forest Green',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(34, 197, 94, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(22, 163, 74, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(134, 239, 172, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(34, 197, 94, 0.02) 0%, 
        rgba(22, 163, 74, 0.04) 25%, 
        rgba(134, 239, 172, 0.02) 50%, 
        rgba(34, 197, 94, 0.06) 75%, 
        rgba(22, 163, 74, 0.03) 100%
      )
    `,
  },
  {
    id: 'solid-dark',
    name: 'Dark',
    type: 'solid',
    value: 'rgba(0, 0, 0, 0.1)',
  },
  {
    id: 'solid-light',
    name: 'Light',
    type: 'solid',
    value: 'rgba(255, 255, 255, 0.05)',
  },
];

interface GlobalBackgroundProps {
  blur?: boolean;
  opacity?: number;
}

export default function GlobalBackground({ blur = false, opacity = 1 }: GlobalBackgroundProps) {
  const [currentBackground, setCurrentBackground] = useState<string>('default-gradient');
  const [customImage, setCustomImage] = useState<string | null>(null);
  const [showDotOverlay, setShowDotOverlay] = useState(true);

  // Load background settings on mount
  useEffect(() => {
    const savedBackground = localStorage.getItem('dashboard-background') || 'default-gradient';
    const savedCustomImage = localStorage.getItem('dashboard-custom-image');
    const savedDotOverlay = localStorage.getItem('dashboard-dot-overlay');

    console.log('GlobalBackground - Loading settings:', { savedBackground, savedCustomImage, savedDotOverlay });

    setCurrentBackground(savedBackground);
    setCustomImage(savedCustomImage);
    setShowDotOverlay(savedDotOverlay !== 'false');
  }, []);

  // Listen for background changes
  useEffect(() => {
    const handleBackgroundChange = (event: CustomEvent) => {
      const { backgroundId, customImage: newCustomImage } = event.detail;
      console.log('GlobalBackground - Background changed:', { backgroundId, newCustomImage });
      setCurrentBackground(backgroundId);
      setCustomImage(newCustomImage);
    };

    const handleDotOverlayChange = (event: CustomEvent) => {
      const { enabled } = event.detail;
      setShowDotOverlay(enabled);
    };

    window.addEventListener('dashboard-background-changed', handleBackgroundChange as EventListener);
    window.addEventListener('dashboard-dot-overlay-changed', handleDotOverlayChange as EventListener);

    return () => {
      window.removeEventListener('dashboard-background-changed', handleBackgroundChange as EventListener);
      window.removeEventListener('dashboard-dot-overlay-changed', handleDotOverlayChange as EventListener);
    };
  }, []);

  // Get the current background style
  const getBackgroundStyle = () => {
    const baseStyle: React.CSSProperties = {
      opacity,
    };

    if (blur) {
      baseStyle.filter = 'blur(20px)';
      baseStyle.transform = 'scale(1.1)'; // Slightly scale up to avoid blur edge artifacts
    }

    if (currentBackground === 'custom-image' && customImage) {
      const style = {
        ...baseStyle,
        backgroundImage: `url(${customImage})`,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
      };
      console.log('GlobalBackground - Using custom image style:', style);
      return style;
    }

    const backgroundOption = DEFAULT_BACKGROUNDS.find(bg => bg.id === currentBackground);
    if (backgroundOption) {
      if (backgroundOption.type === 'gradient') {
        const style = {
          ...baseStyle,
          background: backgroundOption.value,
          backgroundSize: '400% 400%',
        };
        console.log('GlobalBackground - Using gradient style:', style);
        return style;
      } else {
        const style = {
          ...baseStyle,
          background: backgroundOption.value,
        };
        console.log('GlobalBackground - Using solid style:', style);
        return style;
      }
    }

    // Fallback to default
    const style = {
      ...baseStyle,
      background: DEFAULT_BACKGROUNDS[0].value,
      backgroundSize: '400% 400%',
    };
    console.log('GlobalBackground - Using fallback style:', style);
    return style;
  };

  console.log('GlobalBackground - Rendering with:', { currentBackground, customImage, showDotOverlay });

  return (
    <>
      {/* Main background layer */}
      <div 
        className={`fixed inset-0 -z-10 ${currentBackground !== 'custom-image' && !blur ? 'animate-gradient-slow' : ''}`}
        style={getBackgroundStyle()}
      />
      
      {/* Dot pattern overlay */}
      {showDotOverlay && (
        <div 
          className={`fixed inset-0 -z-10 opacity-10 dark:opacity-20 ${blur ? 'blur-sm' : ''}`}
          style={{
            backgroundImage: `radial-gradient(circle, rgba(0, 0, 0, 0.4) 1px, transparent 1px)`,
            backgroundSize: '24px 24px',
            backgroundPosition: '12px 12px',
          }}
        />
      )}
    </>
  );
}
