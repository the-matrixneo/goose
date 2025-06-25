import React from 'react';

interface ScrollTextIconProps {
  className?: string;
  size?: number;
}

export const ScrollTextIcon: React.FC<ScrollTextIconProps> = ({ className = '', size = 12 }) => {
  return (
    <svg 
      width={size} 
      height={size} 
      viewBox="0 0 12 12" 
      fill="none" 
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      <path 
        d="M7.5 6H5M7.5 4H5M9.5 8.5V2.5C9.5 2.23478 9.39464 1.98043 9.20711 1.79289C9.01957 1.60536 8.76522 1.5 8.5 1.5H2M2 1.5C2.26522 1.5 2.51957 1.60536 2.70711 1.79289C2.89464 1.98043 3 2.23478 3 2.5V9.5C3 9.76522 3.10536 10.0196 3.29289 10.2071C3.48043 10.3946 3.73478 10.5 4 10.5M2 1.5C1.73478 1.5 1.48043 1.60536 1.29289 1.79289C1.10536 1.98043 1 2.23478 1 2.5V3.5C1 3.63261 1.05268 3.75979 1.14645 3.85355C1.24021 3.94732 1.36739 4 1.5 4H3M4 10.5H10C10.2652 10.5 10.5196 10.3946 10.7071 10.2071C10.8946 10.0196 11 9.76522 11 9.5V9C11 8.86739 10.9473 8.74021 10.8536 8.64645C10.7598 8.55268 10.6326 8.5 10.5 8.5H5.5C5.36739 8.5 5.24021 8.55268 5.14645 8.64645C5.05268 8.74021 5 8.86739 5 9V9.5C5 9.76522 4.89464 10.0196 4.70711 10.2071C4.51957 10.3946 4.26522 10.5 4 10.5Z" 
        stroke="currentColor" 
        strokeLinecap="round" 
        strokeLinejoin="round"
      />
    </svg>
  );
};