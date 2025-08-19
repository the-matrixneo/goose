import { useState, useEffect } from 'react';

interface AnalogClockProps {
  size?: number;
  showNumbers?: boolean;
  showMinuteMarks?: boolean;
  className?: string;
}

export default function AnalogClock({ 
  size = 200, 
  showNumbers = true, 
  showMinuteMarks = true,
  className = '' 
}: AnalogClockProps) {
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const timer = setInterval(() => {
      setTime(new Date());
    }, 1000);

    return () => clearInterval(timer);
  }, []);

  // Calculate angles for hands
  const secondAngle = (time.getSeconds() * 6) - 90; // 6 degrees per second
  const minuteAngle = (time.getMinutes() * 6) + (time.getSeconds() * 0.1) - 90; // 6 degrees per minute + smooth seconds
  const hourAngle = ((time.getHours() % 12) * 30) + (time.getMinutes() * 0.5) - 90; // 30 degrees per hour + smooth minutes

  const radius = size / 2;
  const centerX = radius;
  const centerY = radius;

  // Generate hour numbers
  const hourNumbers = Array.from({ length: 12 }, (_, i) => {
    const number = i === 0 ? 12 : i;
    const angle = (i * 30) - 90; // 30 degrees per hour
    const numberRadius = radius * 0.75;
    const x = centerX + Math.cos((angle * Math.PI) / 180) * numberRadius;
    const y = centerY + Math.sin((angle * Math.PI) / 180) * numberRadius;
    
    return { number, x, y };
  });

  // Generate minute marks
  const minuteMarks = Array.from({ length: 60 }, (_, i) => {
    const angle = (i * 6) - 90; // 6 degrees per minute
    const isHourMark = i % 5 === 0;
    const outerRadius = radius * 0.9;
    const innerRadius = radius * (isHourMark ? 0.8 : 0.85);
    
    const x1 = centerX + Math.cos((angle * Math.PI) / 180) * innerRadius;
    const y1 = centerY + Math.sin((angle * Math.PI) / 180) * innerRadius;
    const x2 = centerX + Math.cos((angle * Math.PI) / 180) * outerRadius;
    const y2 = centerY + Math.sin((angle * Math.PI) / 180) * outerRadius;
    
    return { x1, y1, x2, y2, isHourMark };
  });

  // Hand coordinates
  const getHandCoordinates = (angle: number, length: number) => {
    const x = centerX + Math.cos((angle * Math.PI) / 180) * length;
    const y = centerY + Math.sin((angle * Math.PI) / 180) * length;
    return { x, y };
  };

  const hourHand = getHandCoordinates(hourAngle, radius * 0.5);
  const minuteHand = getHandCoordinates(minuteAngle, radius * 0.7);
  const secondHand = getHandCoordinates(secondAngle, radius * 0.8);

  return (
    <div className={`relative ${className}`}>
      {/* Glass container */}
      <div 
        className="relative rounded-full bg-white/5 backdrop-blur-md border border-white/10 shadow-lg"
        style={{ width: size, height: size }}
      >
        {/* SVG Clock Face */}
        <svg
          width={size}
          height={size}
          className="absolute inset-0"
          viewBox={`0 0 ${size} ${size}`}
        >
          {/* Outer ring */}
          <circle
            cx={centerX}
            cy={centerY}
            r={radius * 0.95}
            fill="none"
            stroke="rgba(255, 255, 255, 0.2)"
            strokeWidth="2"
          />
          
          {/* Inner ring */}
          <circle
            cx={centerX}
            cy={centerY}
            r={radius * 0.92}
            fill="none"
            stroke="rgba(255, 255, 255, 0.1)"
            strokeWidth="1"
          />

          {/* Minute marks */}
          {showMinuteMarks && minuteMarks.map((mark, i) => (
            <line
              key={i}
              x1={mark.x1}
              y1={mark.y1}
              x2={mark.x2}
              y2={mark.y2}
              stroke={mark.isHourMark ? "rgba(255, 255, 255, 0.4)" : "rgba(255, 255, 255, 0.2)"}
              strokeWidth={mark.isHourMark ? "2" : "1"}
            />
          ))}

          {/* Hour hand */}
          <line
            x1={centerX}
            y1={centerY}
            x2={hourHand.x}
            y2={hourHand.y}
            stroke="rgba(255, 255, 255, 0.8)"
            strokeWidth="4"
            strokeLinecap="round"
          />

          {/* Minute hand */}
          <line
            x1={centerX}
            y1={centerY}
            x2={minuteHand.x}
            y2={minuteHand.y}
            stroke="rgba(255, 255, 255, 0.7)"
            strokeWidth="3"
            strokeLinecap="round"
          />

          {/* Second hand */}
          <line
            x1={centerX}
            y1={centerY}
            x2={secondHand.x}
            y2={secondHand.y}
            stroke="rgba(239, 68, 68, 0.8)"
            strokeWidth="1"
            strokeLinecap="round"
          />

          {/* Center dot */}
          <circle
            cx={centerX}
            cy={centerY}
            r="4"
            fill="rgba(255, 255, 255, 0.8)"
          />
          
          {/* Center dot inner */}
          <circle
            cx={centerX}
            cy={centerY}
            r="2"
            fill="rgba(239, 68, 68, 0.8)"
          />
        </svg>

        {/* Hour numbers */}
        {showNumbers && (
          <div className="absolute inset-0">
            {hourNumbers.map((hour, i) => (
              <div
                key={i}
                className="absolute text-white/70 font-medium select-none"
                style={{
                  left: hour.x - 8,
                  top: hour.y - 10,
                  fontSize: `${size * 0.08}px`,
                  lineHeight: '20px',
                  textAlign: 'center',
                  width: '16px',
                }}
              >
                {hour.number}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
