import React, { useState, useEffect, useRef } from 'react';
import { Card } from '../ui/card';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { 
  Move, 
  WifiOff, 
  Send, 
  Hash,
  Zap,
  Activity
} from 'lucide-react';

interface Position {
  x: number;
  y: number;
}

interface BitChatMessage {
  id: string;
  sender: string;
  content: string;
  timestamp: Date;
  type: 'public' | 'private' | 'system' | 'alert';
  hops?: number;
  encrypted?: boolean;
}

interface MeshPeer {
  id: string;
  name: string;
  signalStrength: number;
  lastSeen: Date;
  isConnected: boolean;
  nodeType: 'relay' | 'endpoint' | 'bridge';
}

interface BitChatWidgetProps {
  initialPosition?: Position;
  onPositionChange?: (position: Position) => void;
}

export const BitChatWidget: React.FC<BitChatWidgetProps> = ({
  initialPosition = { x: 420, y: 20 },
  onPositionChange,
}) => {
  const [position, setPosition] = useState(initialPosition);
  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  const [isExpanded, setIsExpanded] = useState(false);
  const [isOnline, setIsOnline] = useState(true);
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<BitChatMessage[]>([]);
  const [peers, setPeers] = useState<MeshPeer[]>([]);
  const [activeChannel, setActiveChannel] = useState('#underground');
  const [batteryLevel, setBatteryLevel] = useState(87);
  const [meshStrength, setMeshStrength] = useState(3);
  const [currentTime, setCurrentTime] = useState(new Date());
  
  const widgetRef = useRef<HTMLDivElement>(null);

  // Sync position with initialPosition prop - CRITICAL FOR STATE PERSISTENCE
  useEffect(() => {
    setPosition(initialPosition);
  }, [initialPosition]);

  // Update time every second for pager-like display
  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentTime(new Date());
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  // Handle mouse down for dragging - MATCHES ClockWidget
  const handleMouseDown = (e: React.MouseEvent) => {
    if (!widgetRef.current) return;
    
    const rect = widgetRef.current.getBoundingClientRect();
    setDragOffset({
      x: e.clientX - rect.left,
      y: e.clientY - rect.top,
    });
    setIsDragging(true);
  };

  // Handle mouse move for dragging - MATCHES ClockWidget
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging) return;

      const newPosition = {
        x: e.clientX - dragOffset.x,
        y: e.clientY - dragOffset.y,
      };

      // Keep widget within viewport bounds
      const maxX = window.innerWidth - 380; // BitChat widget width
      const maxY = window.innerHeight - 100; // Approximate widget height
      
      newPosition.x = Math.max(0, Math.min(maxX, newPosition.x));
      newPosition.y = Math.max(0, Math.min(maxY, newPosition.y));

      setPosition(newPosition);
      onPositionChange?.(newPosition);
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, dragOffset, onPositionChange]);

  // Mock data with hacker/underground theme
  useEffect(() => {
    const mockMessages: BitChatMessage[] = [
      {
        id: '1',
        sender: 'n0d3_runner',
        content: '>>> mesh network expanding... 7 nodes active',
        timestamp: new Date(Date.now() - 300000),
        type: 'system',
        hops: 2,
        encrypted: true,
      },
      {
        id: '2',
        sender: 'cipher_ghost',
        content: 'anyone got eyes on the downtown relay?',
        timestamp: new Date(Date.now() - 240000),
        type: 'public',
        hops: 1,
        encrypted: false,
      },
      {
        id: '3',
        sender: 'data_mule',
        content: '/status - all green, 12 hops to main grid',
        timestamp: new Date(Date.now() - 180000),
        type: 'system',
        hops: 3,
        encrypted: true,
      },
      {
        id: '4',
        sender: 'mesh_walker',
        content: 'signal strength dropping... switching to backup node',
        timestamp: new Date(Date.now() - 120000),
        type: 'alert',
        hops: 4,
        encrypted: false,
      },
      {
        id: '5',
        sender: 'void_seeker',
        content: 'new drop point established at coordinates 37.7749,-122.4194',
        timestamp: new Date(Date.now() - 60000),
        type: 'private',
        hops: 2,
        encrypted: true,
      }
    ];

    const mockPeers: MeshPeer[] = [
      {
        id: '1',
        name: 'relay_alpha',
        signalStrength: 85,
        lastSeen: new Date(),
        isConnected: true,
        nodeType: 'relay',
      },
      {
        id: '2',
        name: 'bridge_beta',
        signalStrength: 72,
        lastSeen: new Date(Date.now() - 30000),
        isConnected: true,
        nodeType: 'bridge',
      },
      {
        id: '3',
        name: 'endpoint_gamma',
        signalStrength: 45,
        lastSeen: new Date(Date.now() - 120000),
        isConnected: false,
        nodeType: 'endpoint',
      },
    ];

    setMessages(mockMessages);
    setPeers(mockPeers);

    // Simulate network activity
    const networkTimer = setInterval(() => {
      setBatteryLevel(prev => Math.max(20, prev - Math.random() * 2));
      setMeshStrength(prev => Math.max(1, Math.min(5, prev + (Math.random() - 0.5))));
      setIsOnline(prev => Math.random() > 0.05 ? true : !prev);
    }, 5000);

    return () => clearInterval(networkTimer);
  }, []);

  const connectedPeers = peers.filter(p => p.isConnected);

  const handleSendMessage = () => {
    if (!message.trim()) return;

    const newMessage: BitChatMessage = {
      id: Date.now().toString(),
      sender: 'local_node',
      content: message,
      timestamp: new Date(),
      type: message.startsWith('/') ? 'system' : 'public',
      hops: 0,
      encrypted: message.includes('encrypt') || message.includes('secure'),
    };

    setMessages(prev => [...prev.slice(-12), newMessage]);
    setMessage('');
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSendMessage();
    }
  };

  const handleToggleExpanded = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsExpanded(!isExpanded);
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString([], { 
      hour12: false,
      hour: '2-digit', 
      minute: '2-digit',
      second: '2-digit'
    });
  };

  const getNodeTypeIcon = (type: MeshPeer['nodeType']) => {
    switch (type) {
      case 'relay': return '◊';
      case 'bridge': return '▲';
      case 'endpoint': return '●';
      default: return '○';
    }
  };

  const getMessageTypePrefix = (type: BitChatMessage['type']) => {
    switch (type) {
      case 'system': return '[SYS]';
      case 'private': return '[PVT]';
      case 'alert': return '[!!!]';
      default: return '[PUB]';
    }
  };

  const getMessageColor = (type: BitChatMessage['type']) => {
    switch (type) {
      case 'system': return 'text-blue-400';
      case 'private': return 'text-purple-400';
      case 'alert': return 'text-red-400';
      default: return 'text-green-400';
    }
  };

  return (
    <Card
      ref={widgetRef}
      className={`fixed bg-black/90 backdrop-blur-md border border-green-500/30 shadow-lg shadow-green-500/20 transition-all duration-200 select-none font-mono ${
        isDragging ? 'cursor-grabbing scale-105 shadow-xl shadow-green-500/40' : 'cursor-grab hover:shadow-xl hover:shadow-green-500/30 hover:border-green-500/50'
      }`}
      style={{
        left: position.x,
        top: position.y,
        zIndex: 10,
        minWidth: '360px',
        maxWidth: '380px',
        background: 'linear-gradient(135deg, rgba(0,0,0,0.95) 0%, rgba(0,20,0,0.9) 100%)',
        transform: isDragging ? 'none' : undefined,
        willChange: isDragging ? 'transform' : 'auto',
      }}
      onMouseDown={handleMouseDown}
    >
      <div className="p-3">
        {/* Header - Pager Style */}
        <div 
          className="flex items-center gap-2 mb-2 cursor-pointer border-b border-green-500/20 pb-2"
          onClick={handleToggleExpanded}
        >
          {/* App Icon */}
          <img 
            src="/icons/bitchat-icon.png" 
            alt="BitChat" 
            className="w-6 h-6 rounded-sm opacity-90"
          />
          
          <div className="flex items-center gap-2 flex-1">
            <span className="text-green-400 text-xs font-bold tracking-wider">
              BITCHAT.MESH
            </span>
            <div className="flex items-center gap-1">
              {Array.from({length: meshStrength}).map((_, i) => (
                <div key={i} className="w-1 h-3 bg-green-500 opacity-80" />
              ))}
              {Array.from({length: 5 - meshStrength}).map((_, i) => (
                <div key={i} className="w-1 h-3 bg-green-500/20" />
              ))}
            </div>
          </div>
          
          <div className="flex items-center gap-2 text-xs">
            <span className="text-green-400">{formatTime(currentTime)}</span>
            <div className="flex items-center gap-1">
              <Zap className="w-3 h-3 text-yellow-400" />
              <span className="text-yellow-400">{batteryLevel}%</span>
            </div>
            <Move className="w-3 h-3 text-green-500/50" />
          </div>
        </div>

        {/* Status Line - Hacker Style */}
        <div className="flex items-center justify-between text-xs text-green-400 mb-2 font-mono">
          <div className="flex items-center gap-2">
            <Hash className="w-3 h-3" />
            <span>{activeChannel}</span>
            <span className="text-green-500/60">|</span>
            <span>{connectedPeers.length} nodes</span>
          </div>
          <div className="flex items-center gap-2">
            {isOnline ? (
              <Activity className="w-3 h-3 text-green-500 animate-pulse" />
            ) : (
              <WifiOff className="w-3 h-3 text-red-500" />
            )}
            <span className={isOnline ? 'text-green-500' : 'text-red-500'}>
              {isOnline ? 'MESH' : 'OFFLINE'}
            </span>
          </div>
        </div>

        {/* Expanded Content */}
        {isExpanded && (
          <div className="space-y-3">
            {/* Message Feed */}
            <div className="bg-black/50 border border-green-500/20 rounded p-2 h-48 overflow-y-auto">
              <div className="space-y-1 text-xs">
                {messages.map((msg) => (
                  <div key={msg.id} className="flex flex-col gap-1">
                    <div className="flex items-center gap-2">
                      <span className={`${getMessageColor(msg.type)} font-bold`}>
                        {getMessageTypePrefix(msg.type)}
                      </span>
                      <span className="text-green-300">{msg.sender}</span>
                      <span className="text-green-500/60 text-xs">
                        {msg.formatTime ? msg.formatTime(msg.timestamp) : formatTime(msg.timestamp)}
                      </span>
                      {msg.encrypted && <span className="text-yellow-400">🔒</span>}
                      {msg.hops && <span className="text-blue-400">h:{msg.hops}</span>}
                    </div>
                    <div className="text-green-200 ml-2 break-words">
                      {msg.content}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Message Input */}
            <div className="flex gap-2">
              <Input
                value={message}
                onChange={(e) => setMessage(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder="Type message..."
                className="flex-1 bg-black/50 border-green-500/30 text-green-200 text-xs font-mono placeholder:text-green-500/50 focus:border-green-500/60"
              />
              <Button
                onClick={handleSendMessage}
                size="sm"
                className="bg-green-600/20 hover:bg-green-600/40 border border-green-500/30 text-green-400"
              >
                <Send className="w-3 h-3" />
              </Button>
            </div>

            {/* Peer List */}
            <div className="bg-black/50 border border-green-500/20 rounded p-2">
              <div className="text-xs text-green-400 mb-2 font-bold">MESH NODES:</div>
              <div className="space-y-1">
                {peers.map((peer) => (
                  <div key={peer.id} className="flex items-center justify-between text-xs">
                    <div className="flex items-center gap-2">
                      <span className="text-green-300">
                        {getNodeTypeIcon(peer.nodeType)}
                      </span>
                      <span className={peer.isConnected ? 'text-green-400' : 'text-red-400'}>
                        {peer.name}
                      </span>
                    </div>
                    <div className="flex items-center gap-2">
                      <div className="flex items-center gap-1">
                        {Array.from({length: Math.ceil(peer.signalStrength / 20)}).map((_, i) => (
                          <div key={i} className="w-1 h-2 bg-green-500" />
                        ))}
                        {Array.from({length: 5 - Math.ceil(peer.signalStrength / 20)}).map((_, i) => (
                          <div key={i} className="w-1 h-2 bg-green-500/20" />
                        ))}
                      </div>
                      <span className="text-green-500/60">{peer.signalStrength}%</span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* Collapsed Status */}
        {!isExpanded && (
          <div className="text-xs text-green-400 font-mono">
            <div className="flex items-center justify-between">
              <span>Last: {messages.length > 0 ? messages[messages.length - 1].sender : 'none'}</span>
              <span className="text-green-500/60">Click to expand</span>
            </div>
            {messages.length > 0 && (
              <div className="text-green-300/80 mt-1 truncate">
                {messages[messages.length - 1].content}
              </div>
            )}
          </div>
        )}
      </div>
    </Card>
  );
};

export default BitChatWidget;
