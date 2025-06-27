import { useEffect, useRef, useState, useCallback } from 'react';

// Game constants
const CANVAS_WIDTH = 320;
const CANVAS_HEIGHT = 200;
const MAP_SIZE = 16;
const CELL_SIZE = 64;
const FOV = Math.PI / 3; // 60 degrees
const RAYS = 160;
const MAX_DEPTH = 800;

// Game entities
interface Bug {
  x: number;
  y: number;
  angle: number;
  speed: number;
  lastMove: number;
}

interface Bread {
  x: number;
  y: number;
  collected: boolean;
}

interface Player {
  x: number;
  y: number;
  angle: number;
  health: number;
  breadCollected: number;
}

interface GameState {
  player: Player;
  bugs: Bug[];
  bread: Bread[];
  isPlaying: boolean;
  keys: { [key: string]: boolean };
  gameWon: boolean;
  gameOver: boolean;
}
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

// Initialize game entities
const createBugs = (): Bug[] => {
  const bugs: Bug[] = [];
  const bugPositions = [
    { x: 3 * CELL_SIZE, y: 3 * CELL_SIZE },
    { x: 12 * CELL_SIZE, y: 5 * CELL_SIZE },
    { x: 6 * CELL_SIZE, y: 10 * CELL_SIZE },
    { x: 13 * CELL_SIZE, y: 13 * CELL_SIZE },
    { x: 2 * CELL_SIZE, y: 8 * CELL_SIZE },
  ];
  
  bugPositions.forEach(pos => {
    bugs.push({
      x: pos.x,
      y: pos.y,
      angle: Math.random() * Math.PI * 2,
      speed: 1 + Math.random() * 0.5,
      lastMove: 0,
    });
  });
  
  return bugs;
};

const createBread = (): Bread[] => {
  const bread: Bread[] = [];
  const breadPositions = [
    { x: 2 * CELL_SIZE, y: 2 * CELL_SIZE },
    { x: 13 * CELL_SIZE, y: 2 * CELL_SIZE },
    { x: 7 * CELL_SIZE, y: 7 * CELL_SIZE },
    { x: 2 * CELL_SIZE, y: 13 * CELL_SIZE },
    { x: 13 * CELL_SIZE, y: 13 * CELL_SIZE },
    { x: 9 * CELL_SIZE, y: 4 * CELL_SIZE },
    { x: 4 * CELL_SIZE, y: 11 * CELL_SIZE },
  ];
  
  breadPositions.forEach(pos => {
    bread.push({
      x: pos.x,
      y: pos.y,
      collected: false,
    });
  });
  
  return bread;
};

export default function GoosIN() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationRef = useRef<number>();
  const [gameState, setGameState] = useState<GameState>({
    player: {
      x: 96, // Starting position
      y: 96,
      angle: 0,
      health: 100,
      breadCollected: 0,
    },
    bugs: createBugs(),
    bread: createBread(),
    isPlaying: false,
    keys: {},
    gameWon: false,
    gameOver: false,
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
    if (!canvas || !gameState.isPlaying || gameState.gameOver || gameState.gameWon) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Update player position based on input
    let newPlayer = { ...gameState.player };
    let newBugs = [...gameState.bugs];
    let newBread = [...gameState.bread];
    const moveSpeed = 3;
    const rotSpeed = 0.05;
    const currentTime = Date.now();

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

    // Update bugs
    newBugs = newBugs.map(bug => {
      if (currentTime - bug.lastMove > 100) { // Move every 100ms
        let newBugX = bug.x + Math.cos(bug.angle) * bug.speed;
        let newBugY = bug.y + Math.sin(bug.angle) * bug.speed;
        
        // Check collision with walls
        const mapX = Math.floor(newBugX / CELL_SIZE);
        const mapY = Math.floor(newBugY / CELL_SIZE);
        
        if (mapX < 0 || mapX >= MAP_SIZE || mapY < 0 || mapY >= MAP_SIZE || MAP[mapY][mapX] === 1) {
          // Change direction if hitting wall
          return { ...bug, angle: bug.angle + Math.PI + (Math.random() - 0.5), lastMove: currentTime };
        }
        
        return { ...bug, x: newBugX, y: newBugY, lastMove: currentTime };
      }
      return bug;
    });

    // Check bug collisions with player
    let playerHit = false;
    newBugs.forEach(bug => {
      const distance = Math.sqrt((bug.x - newPlayer.x) ** 2 + (bug.y - newPlayer.y) ** 2);
      if (distance < 20) { // Hit radius
        playerHit = true;
      }
    });

    if (playerHit) {
      newPlayer.health -= 10;
      if (newPlayer.health <= 0) {
        setGameState(prev => ({ ...prev, gameOver: true, isPlaying: false }));
        return;
      }
    }

    // Check bread collection
    newBread = newBread.map(bread => {
      if (!bread.collected) {
        const distance = Math.sqrt((bread.x - newPlayer.x) ** 2 + (bread.y - newPlayer.y) ** 2);
        if (distance < 25) { // Collection radius
          newPlayer.breadCollected++;
          return { ...bread, collected: true };
        }
      }
      return bread;
    });

    // Check win condition
    const totalBread = newBread.length;
    const collectedBread = newBread.filter(b => b.collected).length;
    if (collectedBread === totalBread) {
      setGameState(prev => ({ ...prev, gameWon: true, isPlaying: false }));
      return;
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

    // Draw sprites (bugs and bread)
    drawSprites(ctx, newPlayer, newBugs, newBread);

    // Draw HUD
    drawHUD(ctx, newPlayer);

    // Update game state
    setGameState(prev => ({
      ...prev,
      player: newPlayer,
      bugs: newBugs,
      bread: newBread,
    }));

    animationRef.current = requestAnimationFrame(gameLoop);
  }, [gameState]);

  // Draw sprites in 3D space
  const drawSprites = (ctx: CanvasRenderingContext2D, player: Player, bugs: Bug[], bread: Bread[]) => {
    // Draw bugs
    bugs.forEach(bug => {
      const dx = bug.x - player.x;
      const dy = bug.y - player.y;
      const distance = Math.sqrt(dx * dx + dy * dy);
      
      if (distance < MAX_DEPTH) {
        const angle = Math.atan2(dy, dx) - player.angle;
        const normalizedAngle = ((angle + Math.PI) % (2 * Math.PI)) - Math.PI;
        
        if (Math.abs(normalizedAngle) < FOV / 2) {
          const screenX = (normalizedAngle / (FOV / 2)) * (CANVAS_WIDTH / 2) + CANVAS_WIDTH / 2;
          const spriteHeight = (CELL_SIZE / 2 * CANVAS_HEIGHT) / distance;
          const spriteY = CANVAS_HEIGHT / 2 - spriteHeight / 2;
          
          // Draw bug as red circle
          ctx.fillStyle = '#DC2626';
          ctx.beginPath();
          ctx.arc(screenX, spriteY + spriteHeight / 2, spriteHeight / 4, 0, Math.PI * 2);
          ctx.fill();
        }
      }
    });
    
    // Draw bread
    bread.forEach(breadSlice => {
      if (!breadSlice.collected) {
        const dx = breadSlice.x - player.x;
        const dy = breadSlice.y - player.y;
        const distance = Math.sqrt(dx * dx + dy * dy);
        
        if (distance < MAX_DEPTH) {
          const angle = Math.atan2(dy, dx) - player.angle;
          const normalizedAngle = ((angle + Math.PI) % (2 * Math.PI)) - Math.PI;
          
          if (Math.abs(normalizedAngle) < FOV / 2) {
            const screenX = (normalizedAngle / (FOV / 2)) * (CANVAS_WIDTH / 2) + CANVAS_WIDTH / 2;
            const spriteHeight = (CELL_SIZE / 3 * CANVAS_HEIGHT) / distance;
            const spriteY = CANVAS_HEIGHT / 2 - spriteHeight / 2;
            
            // Draw bread as golden rectangle
            ctx.fillStyle = '#F59E0B';
            ctx.fillRect(screenX - spriteHeight / 4, spriteY, spriteHeight / 2, spriteHeight);
          }
        }
      }
    });
  };

  // Draw enhanced HUD
  const drawHUD = (ctx: CanvasRenderingContext2D, player: Player) => {
    // Health bar (top left)
    ctx.fillStyle = '#EF4444';
    ctx.fillRect(5, 5, (player.health / 100) * 60, 4);
    ctx.strokeStyle = '#6B7280';
    ctx.strokeRect(5, 5, 60, 4);
    
    // Bread counter (top right)
    ctx.fillStyle = '#6B7280';
    ctx.font = '12px monospace';
    ctx.fillText(`🍞 ${player.breadCollected}/7`, CANVAS_WIDTH - 60, 15);
    
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

  // Start/stop/restart game
  const toggleGame = () => {
    setGameState(prev => {
      if (prev.gameOver || prev.gameWon) {
        // Restart game
        return {
          player: {
            x: 96,
            y: 96,
            angle: 0,
            health: 100,
            breadCollected: 0,
          },
          bugs: createBugs(),
          bread: createBread(),
          isPlaying: true,
          keys: {},
          gameWon: false,
          gameOver: false,
        };
      } else {
        // Toggle play/pause
        return {
          ...prev,
          isPlaying: !prev.isPlaying
        };
      }
    });
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
    <div className="w-full sm:w-auto animate-in fade-in slide-in-from-right-8 duration-500 rounded-2xl min-w-[340px] max-w-[380px] bg-background-default border border-border-subtle shadow-sm">
      <div className="relative p-4">
        <canvas
          ref={canvasRef}
          width={CANVAS_WIDTH}
          height={CANVAS_HEIGHT}
          className="block bg-background-muted rounded w-full"
          style={{
            imageRendering: 'pixelated',
            filter: gameState.isPlaying ? 'none' : 'brightness(0.8)',
          }}
        />
        
        {/* Game Over Overlay */}
        {gameState.gameOver && (
          <div className="absolute inset-4 flex flex-col items-center justify-center bg-red-50 bg-opacity-95 rounded">
            <div className="text-red-600 font-mono text-sm mb-2">💀 Game Over</div>
            <button
              onClick={toggleGame}
              className="px-3 py-2 bg-red-600 hover:bg-red-700 text-white font-mono text-sm rounded border border-red-400 transition-colors shadow-sm"
            >
              🔄 Try Again
            </button>
          </div>
        )}
        
        {/* Win Overlay */}
        {gameState.gameWon && (
          <div className="absolute inset-4 flex flex-col items-center justify-center bg-green-50 bg-opacity-95 rounded">
            <div className="text-green-600 font-mono text-sm mb-2">🎉 You Won!</div>
            <button
              onClick={toggleGame}
              className="px-3 py-2 bg-green-600 hover:bg-green-700 text-white font-mono text-sm rounded border border-green-400 transition-colors shadow-sm"
            >
              🔄 Play Again
            </button>
          </div>
        )}
        
        {/* Start Game Overlay */}
        {!gameState.isPlaying && !gameState.gameOver && !gameState.gameWon && (
          <div className="absolute inset-4 flex flex-col items-center justify-center bg-background-default bg-opacity-90 rounded">
            <div className="text-text-muted font-mono text-xs mb-2">Collect 🍞 • Avoid 🐛</div>
            <button
              onClick={toggleGame}
              className="px-3 py-2 bg-background-accent hover:bg-background-accent/80 text-text-on-accent font-mono text-sm rounded border border-border-subtle transition-colors shadow-sm"
            >
              ▶ Play
            </button>
          </div>
        )}
        
        {/* Pause Button */}
        {gameState.isPlaying && (
          <button
            onClick={toggleGame}
            className="absolute top-2 right-2 px-2 py-1 bg-background-subtle hover:bg-background-muted text-text-muted font-mono text-xs rounded transition-colors"
          >
            ⏸
          </button>
        )}
      </div>
    </div>
  );
}