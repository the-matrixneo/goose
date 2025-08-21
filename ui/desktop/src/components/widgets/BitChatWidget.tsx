import React, { useState, useEffect, useRef, useCallback } from 'react';
import { Card } from '../ui/card';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { 
  Move, 
  Wifi, 
  WifiOff, 
  Users, 
  Send, 
  MessageCircle, 
  Bluetooth,
  Signal,
  Hash,
  User,
  Clock
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
  type: 'public' | 'private' | 'system';
  hops?: number;
}

interface MeshPeer {
  id: string;
  name: string;
  signalStrength: number;
  lastSeen: Date;
  isConnected: boolean;
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
  const [isConnected, setIsConnected] = useState(true);
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<BitChatMessage[]>([]);
  const [peers, setPeers] = useState<MeshPeer[]>([]);
  const [activeChannel, setActiveChannel] = useState('#general');
  const widgetRef = useRef<HTMLDivElement>(null);

  // Mock data for demonstration
  useEffect(() => {
    const mockMessages: BitChatMessage[] = [
      {
        id: '1',
        sender: 'alice_mesh',
        content: 'Anyone else seeing the mesh network expand?',
        timestamp: new Date(Date.now() - 5 * 60 * 1000),
        type: 'public',
        hops: 2,
      },
      {
        id: '2',
        sender: 'bob_node',
        content: '/who',
        timestamp: new Date(Date.now() - 3 * 60 * 1000),
        type: 'public',
        hops: 1,
      },
      {
        id: '3',
        sender: 'system',
        content: 'Connected peers: alice_mesh, bob_node, charlie_bt',
        timestamp: new Date(Date.now() - 3 * 60 * 1000),
        type: 'system',
      },
      {
        id: '4',
        sender: 'charlie_bt',
        content: 'This mesh chat is incredible! No internet needed 🔥',
        timestamp: new Date(Date.now() - 1 * 60 * 1000),
        type: 'public',
        hops: 3,
      },
    ];

    const mockPeers: MeshPeer[] = [
      {
        id: 'alice_mesh',
        name: 'Alice',
        signalStrength: 85,
        lastSeen: new Date(Date.now() - 30 * 1000),
        isConnected: true,
      },
      {
        id: 'bob_node',
        name: 'Bob',
        signalStrength: 72,
        lastSeen: new Date(Date.now() - 45 * 1000),
        isConnected: true,
      },
      {
        id: 'charlie_bt',
        name: 'Charlie',
        signalStrength: 58,
        lastSeen: new Date(Date.now() - 60 * 1000),
        isConnected: true,
      },
      {
        id: 'diana_offline',
        name: 'Diana',
        signalStrength: 0,
        lastSeen: new Date(Date.now() - 10 * 60 * 1000),
        isConnected: false,
      },
    ];

    setMessages(mockMessages);
    setPeers(mockPeers);

    // Simulate periodic mesh activity
    const interval = setInterval(() => {
      // Randomly add new messages
      if (Math.random() < 0.3) {
        const senders = ['alice_mesh', 'bob_node', 'charlie_bt'];
        const contents = [
          'Mesh network is stable',
          'Anyone near the downtown area?',
          '/slap bob_node',
          'This decentralized chat rocks!',
          'Battery at 85%, mesh mode active',
        ];
        
        const newMessage: BitChatMessage = {
          id: Date.now().toString(),
          sender: senders[Math.floor(Math.random() * senders.length)],
          content: contents[Math.floor(Math.random() * contents.length)],
          timestamp: new Date(),
          type: 'public',
          hops: Math.floor(Math.random() * 4) + 1,
        };
        
        setMessages(prev => [...prev.slice(-10), newMessage]);
      }
    }, 8000);

    return () => clearInterval(interval);
  }, []);

  // Handle mouse down for dragging
  const handleMouseDown = (e: React.MouseEvent) => {
    if (!widgetRef.current) return;
    
    const rect = widgetRef.current.getBoundingClientRect();
    setDragOffset({
      x: e.clientX - rect.left,
      y: e.clientY - rect.top,
    });
    setIsDragging(true);
  };

  // Handle mouse move for dragging
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging) return;

      const newPosition = {
        x: e.clientX - dragOffset.x,
        y: e.clientY - dragOffset.y,
      };

      const maxX = window.innerWidth - 350;
      const maxY = window.innerHeight - (isExpanded ? 400 : 120);
      
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
  }, [isDragging, dragOffset, isExpanded, onPositionChange]);

  const handleSendMessage = useCallback(() => {
    if (!message.trim()) return;

    const newMessage: BitChatMessage = {
      id: Date.now().toString(),
      sender: 'you',
      content: message,
      timestamp: new Date(),
      type: message.startsWith('/') ? 'system' : 'public',
      hops: 0,
    };

    setMessages(prev => [...prev.slice(-10), newMessage]);
    setMessage('');
  }, [message]);

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSendMessage();
    }
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const getSignalIcon = (strength: number) => {
    if (strength > 70) return <Signal className="w-3 h-3 text-green-500" />;
    if (strength > 40) return <Signal className="w-3 h-3 text-yellow-500" />;
    if (strength > 0) return <Signal className="w-3 h-3 text-red-500" />;
    return <WifiOff className="w-3 h-3 text-gray-500" />;
  };

  const connectedPeers = peers.filter(peer => peer.isConnected);

  return (
    <Card
      ref={widgetRef}
      className={`fixed bg-background-default/80 backdrop-blur-md border border-white/10 shadow-lg transition-all duration-200 select-none ${
        isDragging ? 'cursor-grabbing scale-105 shadow-xl' : 'cursor-grab hover:shadow-xl hover:scale-105'
      }`}
      style={{
        left: position.x,
        top: position.y,
        zIndex: 50,
        minWidth: '320px',
        maxWidth: '350px',
      }}
      onMouseDown={handleMouseDown}
    >
      <div className="p-4">
        {/* Header */}
        <div 
          className="flex items-center gap-2 mb-3 cursor-pointer"
          onClick={(e) => {
            e.stopPropagation();
            setIsExpanded(!isExpanded);
          }}
        >
          <div className="flex items-center gap-2">
            <Bluetooth className="w-4 h-4 text-blue-500" />
            <MessageCircle className="w-4 h-4 text-text-muted" />
            <span className="text-sm font-medium text-text-standard">
              BitChat Mesh
            </span>
          </div>
          <div className="flex items-center gap-1 ml-auto">
            {isConnected ? (
              <Wifi className="w-4 h-4 text-green-500" />
            ) : (
              <WifiOff className="w-4 h-4 text-red-500" />
            )}
            <Move className="w-3 h-3 text-text-muted opacity-50" />
          </div>
        </div>

        {/* Status Bar */}
        <div className="flex items-center justify-between text-xs text-text-muted mb-3">
          <div className="flex items-center gap-2">
            <Hash className="w-3 h-3" />
            <span>{activeChannel}</span>
          </div>
          <div className="flex items-center gap-2">
            <Users className="w-3 h-3" />
            <span>{connectedPeers.length} peers</span>
          </div>
        </div>

        {/* Expanded Content */}
        {isExpanded && (
          <div className="space-y-3">
            {/* Messages */}
            <div className="bg-background-muted/30 rounded-lg p-3 max-h-48 overflow-y-auto">
              <div className="space-y-2">
                {messages.slice(-8).map((msg) => (
                  <div key={msg.id} className="text-xs">
                    <div className="flex items-center gap-1 mb-1">
                      <Clock className="w-3 h-3 text-text-muted" />
                      <span className="text-text-muted">{formatTime(msg.timestamp)}</span>
                      {msg.type === 'public' && msg.hops && (
                        <span className="text-text-muted">({msg.hops} hops)</span>
                      )}
                    </div>
                    <div className="flex items-start gap-2">
                      {msg.type === 'system' ? (
                        <span className="text-blue-400 font-mono">*</span>
                      ) : (
                        <User className="w-3 h-3 text-text-muted mt-0.5" />
                      )}
                      <div>
                        <span className={`font-medium ${
                          msg.sender === 'you' 
                            ? 'text-blue-400' 
                            : msg.type === 'system' 
                            ? 'text-blue-400' 
                            : 'text-text-standard'
                        }`}>
                          {msg.sender}:
                        </span>
                        <span className="ml-1 text-text-standard">{msg.content}</span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Peers List */}
            <div className="bg-background-muted/30 rounded-lg p-3">
              <div className="text-xs font-medium text-text-standard mb-2">Mesh Peers</div>
              <div className="space-y-1">
                {peers.slice(0, 4).map((peer) => (
                  <div key={peer.id} className="flex items-center justify-between text-xs">
                    <div className="flex items-center gap-2">
                      {getSignalIcon(peer.signalStrength)}
                      <span className={peer.isConnected ? 'text-text-standard' : 'text-text-muted'}>
                        {peer.name}
                      </span>
                    </div>
                    <span className="text-text-muted">
                      {peer.isConnected ? `${peer.signalStrength}%` : 'offline'}
                    </span>
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
                placeholder="Type message or /command..."
                className="text-xs h-8"
                onClick={(e) => e.stopPropagation()}
              />
              <Button
                size="sm"
                onClick={(e) => {
                  e.stopPropagation();
                  handleSendMessage();
                }}
                className="h-8 px-2"
              >
                <Send className="w-3 h-3" />
              </Button>
            </div>
          </div>
        )}

        {/* Collapsed Status */}
        {!isExpanded && (
          <div className="space-y-2">
            <div className="text-xs text-text-muted">
              {messages.length > 0 && (
                <div className="flex items-center gap-1">
                  <span className="font-medium">{messages[messages.length - 1].sender}:</span>
                  <span className="truncate">{messages[messages.length - 1].content}</span>
                </div>
              )}
            </div>
            <div className="flex items-center justify-between text-xs text-text-muted">
              <span>{connectedPeers.length} peers online</span>
              <span>Click to expand</span>
            </div>
          </div>
        )}
      </div>
    </Card>
  );
};

export default BitChatWidget;
