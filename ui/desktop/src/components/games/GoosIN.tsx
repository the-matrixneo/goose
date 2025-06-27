import { useEffect, useRef, useState, useCallback } from 'react';
import { Card, CardContent } from '../ui/card';

// Game constants
const CANVAS_WIDTH = 320;
const CANVAS_HEIGHT = 200;
const MAP_SIZE = 16;
const CELL_SIZE = 64;
const FOV = Math.PI / 3; // 60 degrees
const RAYS = 160;
const MAX_DEPTH = 800;

// Simple map (1 = wall, 0 = empty)
const MAP = [
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,1,1,0,1,1,0,0,1,1,0,1,1,0,1],
  [1,0,1,0,0,0,1,0,0,1,0,0,0,1,0,1],
  [1,0,1,0,1,0,0,0,0,0,0,1,0,1,0,1],
  [1,0,0,0,1,1,1,0,0,1,1,1,0,0,0,1],
  [1,0,1,0,0,0,0,0,0,0,0,0,0,1,0,1],
  [1,0,1,1,0,1,0,0,0,0,1,0,1,1,0,1],
  [1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],
  [1,0,1,1,0,0,0,0,0,0,0,0,1,1,0,1],
  [1,0,0,0,0,1,1,0,0,1,1,0,0,0,0,1],
  [1,0,1,0,1,0,0,0,0,0,0,1,0,1,0,1],
  [1,0,1,0,0,0,1,1,1,1,0,0,0,1,0,1],
  [1,0,1,1,0,0,0,0,0,0,0,0,1,1,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

interface Player {
  x: number;
  y: number;
  angle: number;
  health: number;
  ammo: number;
}

interface GameState {
  player: Player;
  isPlaying: boolean;
  keys: { [key: string]: boolean };
}

export default function GoosIN() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationRef = useRef<number>();
  const [gameState, setGameState] = useState<GameState>({
    player: {
      x: 96, // Starting position
      y: 96,
      angle: 0,
      health: 100,
      ammo: 50,
    },
    isPlaying: false,
    keys: {},
  });

  // Handle keyboard input
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (!gameState.isPlaying) return;
    
    setGameState(prev => ({
      ...prev,
      keys: { ...prev.keys, [e.key.toLowerCase()]: true }
    }));
  }, [gameState.isPlaying]);

  const handleKeyUp = useCallback((e: KeyboardEvent) => {
    setGameState(prev => ({
      ...prev,
      keys: { ...prev.keys, [e.key.toLowerCase()]: false }
    }));
  }, []);

  // Raycasting function
  const castRay = (angle: number, playerX: number, playerY: number) => {
    const rayX = Math.cos(angle);
    const rayY = Math.sin(angle);
    
    let distance = 0;
    let hit = false;
    
    while (!hit && distance < MAX_DEPTH) {
      distance += 2;
      
      const testX = Math.floor((playerX + rayX * distance) / CELL_SIZE);
      const testY = Math.floor((playerY + rayY * distance) / CELL_SIZE);
      
      if (testX < 0 || testX >= MAP_SIZE || testY < 0 || testY >= MAP_SIZE || MAP[testY][testX] === 1) {
        hit = true;
      }
    }
    
    return distance;
  };

  // Game update and render loop
  const gameLoop = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas || !gameState.isPlaying) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Update player position based on input
    let newPlayer = { ...gameState.player };
    const moveSpeed = 3;
    const rotSpeed = 0.05;

    if (gameState.keys['w'] || gameState.keys['arrowup']) {
      const newX = newPlayer.x + Math.cos(newPlayer.angle) * moveSpeed;
      const newY = newPlayer.y + Math.sin(newPlayer.angle) * moveSpeed;
      
      // Simple collision detection
      const mapX = Math.floor(newX / CELL_SIZE);
      const mapY = Math.floor(newY / CELL_SIZE);
      
      if (mapX >= 0 && mapX < MAP_SIZE && mapY >= 0 && mapY < MAP_SIZE && MAP[mapY][mapX] === 0) {
        newPlayer.x = newX;
        newPlayer.y = newY;
      }
    }
    
    if (gameState.keys['s'] || gameState.keys['arrowdown']) {
      const newX = newPlayer.x - Math.cos(newPlayer.angle) * moveSpeed;
      const newY = newPlayer.y - Math.sin(newPlayer.angle) * moveSpeed;
      
      const mapX = Math.floor(newX / CELL_SIZE);
      const mapY = Math.floor(newY / CELL_SIZE);
      
      if (mapX >= 0 && mapX < MAP_SIZE && mapY >= 0 && mapY < MAP_SIZE && MAP[mapY][mapX] === 0) {
        newPlayer.x = newX;
        newPlayer.y = newY;
      }
    }
    
    if (gameState.keys['a'] || gameState.keys['arrowleft']) {
      newPlayer.angle -= rotSpeed;
    }
    
    if (gameState.keys['d'] || gameState.keys['arrowright']) {
      newPlayer.angle += rotSpeed;
    }

    // Clear canvas with light background
    ctx.fillStyle = '#F8F9FA';
    ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // Draw ceiling (light gray)
    ctx.fillStyle = '#E9ECEF';
    ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT / 2);
    
    // Draw floor (subtle beige)
    ctx.fillStyle = '#F5F5F5';
    ctx.fillRect(0, CANVAS_HEIGHT / 2, CANVAS_WIDTH, CANVAS_HEIGHT / 2);

    // Cast rays for 3D rendering
    for (let i = 0; i < RAYS; i++) {
      const rayAngle = newPlayer.angle - FOV / 2 + (i / RAYS) * FOV;
      const distance = castRay(rayAngle, newPlayer.x, newPlayer.y);
      
      // Calculate wall height
      const wallHeight = (CELL_SIZE * CANVAS_HEIGHT) / distance;
      const wallTop = (CANVAS_HEIGHT - wallHeight) / 2;
      
      // Wall color based on distance (lighter theme)
      const brightness = Math.max(0.4, 1 - distance / MAX_DEPTH);
      const baseColor = 180; // Light gray base
      const wallColor = Math.floor(baseColor * brightness);
      
      ctx.fillStyle = `rgb(${wallColor}, ${wallColor}, ${wallColor})`;
      ctx.fillRect(i * (CANVAS_WIDTH / RAYS), wallTop, CANVAS_WIDTH / RAYS + 1, wallHeight);
    }

    // Draw HUD
    drawHUD(ctx, newPlayer);

    // Update game state
    setGameState(prev => ({
      ...prev,
      player: newPlayer
    }));

    animationRef.current = requestAnimationFrame(gameLoop);
  }, [gameState]);

  // Draw minimalist HUD
  const drawHUD = (ctx: CanvasRenderingContext2D, player: Player) => {
    // Simple crosshair
    ctx.strokeStyle = '#6B7280';
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(CANVAS_WIDTH / 2 - 4, CANVAS_HEIGHT / 2);
    ctx.lineTo(CANVAS_WIDTH / 2 + 4, CANVAS_HEIGHT / 2);
    ctx.moveTo(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 - 4);
    ctx.lineTo(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 + 4);
    ctx.stroke();
  };

  // Start/stop game
  const toggleGame = () => {
    setGameState(prev => ({
      ...prev,
      isPlaying: !prev.isPlaying
    }));
  };

  // Setup event listeners and game loop
  useEffect(() => {
    if (gameState.isPlaying) {
      window.addEventListener('keydown', handleKeyDown);
      window.addEventListener('keyup', handleKeyUp);
      animationRef.current = requestAnimationFrame(gameLoop);
    } else {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    }

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [gameState.isPlaying, gameLoop, handleKeyDown, handleKeyUp]);

  return (
    <Card className="w-full sm:w-auto animate-in fade-in slide-in-from-right-8 duration-500 rounded-2xl min-w-[340px] max-w-[380px]">
      <CardContent className="flex flex-col items-center p-4">
        <div className="relative border border-border-subtle rounded">
          <canvas
            ref={canvasRef}
            width={CANVAS_WIDTH}
            height={CANVAS_HEIGHT}
            className="block bg-background-muted"
            style={{
              imageRendering: 'pixelated',
              filter: gameState.isPlaying ? 'none' : 'brightness(0.8)',
            }}
          />
          
          {!gameState.isPlaying && (
            <div className="absolute inset-0 flex items-center justify-center bg-background-default bg-opacity-90 rounded">
              <button
                onClick={toggleGame}
                className="px-3 py-2 bg-background-accent hover:bg-background-accent/80 text-text-on-accent font-mono text-sm rounded border border-border-subtle transition-colors shadow-sm"
              >
                ▶ Play
              </button>
            </div>
          )}
        </div>
        
        {gameState.isPlaying && (
          <button
            onClick={toggleGame}
            className="mt-2 px-2 py-1 bg-background-subtle hover:bg-background-muted text-text-muted font-mono text-xs rounded transition-colors"
          >
            ⏸ Pause
          </button>
        )}
      </CardContent>
    </Card>
  );
}