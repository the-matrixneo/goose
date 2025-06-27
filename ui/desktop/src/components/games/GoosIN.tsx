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

    // Clear canvas with light UI-matching background
    ctx.fillStyle = '#FAFAFA'; // Even lighter background
    ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    
    // Draw ceiling (very light gray)
    ctx.fillStyle = '#F5F5F5';
    ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT / 2);
    
    // Draw floor (clean light gray, no patterns)
    ctx.fillStyle = '#EEEEEE';
    ctx.fillRect(0, CANVAS_HEIGHT / 2, CANVAS_WIDTH, CANVAS_HEIGHT / 2);

    // Cast rays for 3D rendering
    for (let i = 0; i < RAYS; i++) {
      const rayAngle = newPlayer.angle - FOV / 2 + (i / RAYS) * FOV;
      const distance = castRay(rayAngle, newPlayer.x, newPlayer.y);
      
      // Calculate wall height
      const wallHeight = (CELL_SIZE * CANVAS_HEIGHT) / distance;
      const wallTop = (CANVAS_HEIGHT - wallHeight) / 2;
      
      // Very light walls with subtle depth shading only
      const brightness = Math.max(0.7, 1 - distance / MAX_DEPTH);
      
      // Much lighter wall colors - almost white
      const baseGray = Math.floor(240 * brightness);  // Very light gray base
      
      const wallSlice = i * (CANVAS_WIDTH / RAYS);
      const sliceWidth = Math.ceil(CANVAS_WIDTH / RAYS);
      
      // Clean wall color (no patterns)
      ctx.fillStyle = `rgb(${baseGray}, ${baseGray}, ${baseGray})`;
      ctx.fillRect(wallSlice, wallTop, sliceWidth, wallHeight);
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

  // Draw sprites in 3D space with proper occlusion
  const drawSprites = (ctx: CanvasRenderingContext2D, player: Player, bugs: Bug[], bread: Bread[]) => {
    // Collect all sprites with distance for depth sorting
    const allSprites: Array<{
      x: number;
      y: number;
      distance: number;
      type: 'bug' | 'bread';
      data: Bug | Bread;
    }> = [];
    
    // Add bugs to sprite list
    bugs.forEach(bug => {
      const dx = bug.x - player.x;
      const dy = bug.y - player.y;
      const distance = Math.sqrt(dx * dx + dy * dy);
      allSprites.push({ x: bug.x, y: bug.y, distance, type: 'bug', data: bug });
    });
    
    // Add bread to sprite list
    bread.forEach(breadSlice => {
      if (!breadSlice.collected) {
        const dx = breadSlice.x - player.x;
        const dy = breadSlice.y - player.y;
        const distance = Math.sqrt(dx * dx + dy * dy);
        allSprites.push({ x: breadSlice.x, y: breadSlice.y, distance, type: 'bread', data: breadSlice });
      }
    });
    
    // Sort sprites by distance (far to near)
    allSprites.sort((a, b) => b.distance - a.distance);
    
    // Draw each sprite with occlusion checking
    allSprites.forEach(sprite => {
      if (sprite.distance < MAX_DEPTH) {
        const dx = sprite.x - player.x;
        const dy = sprite.y - player.y;
        const angle = Math.atan2(dy, dx) - player.angle;
        const normalizedAngle = ((angle + Math.PI) % (2 * Math.PI)) - Math.PI;
        
        if (Math.abs(normalizedAngle) < FOV / 2) {
          // Cast multiple rays around the sprite position for better occlusion detection
          const centerRayDistance = castRay(player.angle + normalizedAngle, player.x, player.y);
          const leftRayDistance = castRay(player.angle + normalizedAngle - 0.02, player.x, player.y);
          const rightRayDistance = castRay(player.angle + normalizedAngle + 0.02, player.x, player.y);
          
          // Use the closest wall distance from multiple rays
          const minWallDistance = Math.min(centerRayDistance, leftRayDistance, rightRayDistance);
          
          // More generous visibility test - sprite is visible if any ray shows it's not blocked
          const isVisible = sprite.distance < minWallDistance - 2 || 
                           sprite.distance < centerRayDistance - 5 ||
                           minWallDistance > MAX_DEPTH - 100;
          
          if (isVisible) {
            const screenX = (normalizedAngle / (FOV / 2)) * (CANVAS_WIDTH / 2) + CANVAS_WIDTH / 2;
            
            if (sprite.type === 'bug') {
              drawBugSprite(ctx, screenX, sprite.distance);
            } else {
              drawBreadSprite(ctx, screenX, sprite.distance);
            }
          }
        }
      }
    });
  };
  
  // Draw bug sprite
  const drawBugSprite = (ctx: CanvasRenderingContext2D, screenX: number, distance: number) => {
    const spriteHeight = (CELL_SIZE / 2 * CANVAS_HEIGHT) / distance;
    const spriteY = CANVAS_HEIGHT / 2 - spriteHeight / 2;
    const spriteWidth = spriteHeight * 0.8;
    
    // Draw scary bug creature (pixelated style)
    const pixelSize = Math.max(1, spriteHeight / 16);
    
    // Body (dark red/brown)
    ctx.fillStyle = '#7F1D1D';
    ctx.fillRect(screenX - spriteWidth/2, spriteY + spriteHeight*0.3, spriteWidth, spriteHeight*0.5);
    
    // Head (darker)
    ctx.fillStyle = '#450A0A';
    ctx.fillRect(screenX - spriteWidth/3, spriteY, spriteWidth*0.66, spriteHeight*0.4);
    
    // Eyes (glowing red)
    ctx.fillStyle = '#DC2626';
    const eyeSize = Math.max(1, pixelSize * 2);
    ctx.fillRect(screenX - spriteWidth/4, spriteY + spriteHeight*0.1, eyeSize, eyeSize);
    ctx.fillRect(screenX + spriteWidth/6, spriteY + spriteHeight*0.1, eyeSize, eyeSize);
    
    // Legs/appendages
    ctx.fillStyle = '#7F1D1D';
    for (let i = 0; i < 4; i++) {
      const legX = screenX - spriteWidth/2 + (i * spriteWidth/4);
      ctx.fillRect(legX, spriteY + spriteHeight*0.8, pixelSize, spriteHeight*0.2);
    }
  };
  
  // Draw bread sprite
  const drawBreadSprite = (ctx: CanvasRenderingContext2D, screenX: number, distance: number) => {
    const spriteHeight = (CELL_SIZE / 3 * CANVAS_HEIGHT) / distance;
    const spriteY = CANVAS_HEIGHT / 2 - spriteHeight / 2;
    const spriteWidth = spriteHeight * 0.7;
    
    // Draw pixelated bread slice
    const pixelSize = Math.max(1, spriteHeight / 12);
    
    // Bread crust (dark brown)
    ctx.fillStyle = '#92400E';
    ctx.fillRect(screenX - spriteWidth/2, spriteY, spriteWidth, spriteHeight);
    
    // Bread interior (light brown/beige)
    ctx.fillStyle = '#FDE68A';
    ctx.fillRect(
      screenX - spriteWidth/2 + pixelSize, 
      spriteY + pixelSize, 
      spriteWidth - pixelSize*2, 
      spriteHeight - pixelSize*2
    );
    
    // Bread texture (darker spots)
    ctx.fillStyle = '#F59E0B';
    for (let i = 0; i < 3; i++) {
      const spotX = screenX - spriteWidth/4 + (i * spriteWidth/6);
      const spotY = spriteY + spriteHeight/3 + (i % 2) * spriteHeight/4;
      ctx.fillRect(spotX, spotY, pixelSize, pixelSize);
    }
  };

  // Draw UI-matching HUD with clean design
  const drawHUD = (ctx: CanvasRenderingContext2D, player: Player) => {
    // Subtle HUD background bar
    ctx.fillStyle = 'rgba(107, 114, 128, 0.6)'; // More transparent
    ctx.fillRect(0, CANVAS_HEIGHT - 25, CANVAS_WIDTH, 25);
    
    // Health bar with UI-matching colors
    ctx.fillStyle = '#E5E5E5'; // Very light gray background
    ctx.fillRect(5, CANVAS_HEIGHT - 20, 60, 6);
    ctx.fillStyle = '#EF4444'; // Red health (matching UI red)
    ctx.fillRect(5, CANVAS_HEIGHT - 20, (player.health / 100) * 60, 6);
    
    // Health text in UI gray
    ctx.fillStyle = '#374151';
    ctx.font = 'bold 9px monospace';
    ctx.fillText(`HP ${player.health}`, 5, CANVAS_HEIGHT - 5);
    
    // Bread counter in UI gray
    ctx.fillStyle = '#374151';
    ctx.font = 'bold 9px monospace';
    ctx.fillText(`BREAD ${player.breadCollected}/7`, CANVAS_WIDTH - 75, CANVAS_HEIGHT - 5);
    
    // Subtle crosshair matching UI colors
    ctx.strokeStyle = '#9CA3AF';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(CANVAS_WIDTH / 2 - 5, CANVAS_HEIGHT / 2);
    ctx.lineTo(CANVAS_WIDTH / 2 + 5, CANVAS_HEIGHT / 2);
    ctx.moveTo(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 - 5);
    ctx.lineTo(CANVAS_WIDTH / 2, CANVAS_HEIGHT / 2 + 5);
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
            filter: gameState.isPlaying ? 'contrast(1.05) saturate(1.1)' : 'brightness(0.8) contrast(1.05)',
          }}
        />
        
        {/* Game Over Overlay */}
        {gameState.gameOver && (
          <div className="absolute inset-4 flex flex-col justify-end items-start bg-red-50 bg-opacity-95 rounded">
            <div className="text-red-600 font-mono text-sm mb-2">GAME OVER</div>
            <button
              onClick={toggleGame}
              className="px-3 py-2 bg-red-600 hover:bg-red-700 text-white font-mono text-sm rounded border border-red-400 transition-colors shadow-sm"
            >
              TRY AGAIN
            </button>
          </div>
        )}
        
        {/* Win Overlay */}
        {gameState.gameWon && (
          <div className="absolute inset-4 flex flex-col justify-end items-start bg-green-50 bg-opacity-95 rounded">
            <div className="text-green-600 font-mono text-sm mb-2">VICTORY</div>
            <button
              onClick={toggleGame}
              className="px-3 py-2 bg-green-600 hover:bg-green-700 text-white font-mono text-sm rounded border border-green-400 transition-colors shadow-sm"
            >
              PLAY AGAIN
            </button>
          </div>
        )}
        
        {/* Start Game Overlay */}
        {!gameState.isPlaying && !gameState.gameOver && !gameState.gameWon && (
          <div className="absolute inset-4 flex flex-col justify-between bg-background-default bg-opacity-90 rounded">
            <div className="flex justify-start pt-2">
              <svg width="37" height="11" viewBox="0 0 37 11" fill="none" xmlns="http://www.w3.org/2000/svg" className="w-16 h-auto">
                <path d="M1.25703 10.232V9.188H0.189031V7.7H1.60503V8.744H4.47303V7.1H1.25703V6.044H0.189031V3.512H1.25703V2.468H4.82103V3.512H5.88903V9.188H4.82103V10.232H1.25703ZM1.60503 5.6H4.47303V3.956H1.60503V5.6ZM8.05391 8.132V7.1H6.98591V3.512H8.05391V2.468H11.6059V3.512H12.6859V7.1H11.6059V8.132H8.05391ZM8.40191 6.644H11.2579V3.956H8.40191V6.644ZM15.2141 8.132V7.1H14.1461V3.512H15.2141V2.468H18.7661V3.512H19.8461V7.1H18.7661V8.132H15.2141ZM15.5621 6.644H18.4181V3.956H15.5621V6.644ZM21.3062 8.132V6.644H25.5782V6.044H22.3742V4.988H21.3062V3.512H22.3742V2.468H27.0062V3.956H22.7222V4.544H25.9262V5.6H27.0062V7.1H25.9262V8.132H21.3062ZM28.4664 8.132V2.468H29.8824V8.132H28.4664ZM28.4664 1.868V0.368H29.8824V1.868H28.4664ZM31.1148 8.132V2.468H35.7348V3.512H36.8148V8.132H35.3868V3.956H32.5308V8.132H31.1148Z" fill="currentColor" className="text-text-muted"/>
              </svg>
            </div>
            <div className="flex flex-col items-start">
              <div className="text-text-muted font-mono text-xs mb-2">COLLECT BREAD • AVOID BUGS</div>
              <button
                onClick={toggleGame}
                className="px-3 py-2 bg-background-accent hover:bg-background-accent/80 text-text-on-accent font-mono text-sm rounded border border-border-subtle transition-colors shadow-sm"
              >
                PLAY
              </button>
            </div>
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