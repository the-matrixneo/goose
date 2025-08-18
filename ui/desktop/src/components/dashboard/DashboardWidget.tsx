import React from 'react';
import { Card, CardContent, CardDescription } from '../ui/card';
import { WidgetData, WidgetType } from '../../types/dashboard';
import { Button } from '../ui/button';
import { ChatSmart } from '../icons/';
import { Skeleton } from '../ui/skeleton';

interface DashboardWidgetProps {
  widget: WidgetData;
  onMouseDown: (e: React.MouseEvent) => void;
  isDragging: boolean;
}

export function DashboardWidget({ widget, onMouseDown, isDragging }: DashboardWidgetProps) {
  const renderWidgetContent = () => {
    switch (widget.type) {
      case WidgetType.TOTAL_SESSIONS:
        return (
          <CardContent className="flex flex-col justify-end h-full p-0">
            <div className="flex flex-col justify-end">
              <p className="text-3xl font-mono font-light flex items-end text-white/90">
                {widget.data?.totalSessions ?? 0}
              </p>
              <span className="text-xs text-white/60">Total sessions</span>
            </div>
          </CardContent>
        );

      case WidgetType.TOTAL_TOKENS:
        return (
          <CardContent className="flex flex-col justify-end h-full p-0">
            <div className="flex flex-col justify-end">
              <p className="text-3xl font-mono font-light flex items-end text-white/90">
                {widget.data?.totalTokens && widget.data.totalTokens > 0
                  ? `${(widget.data.totalTokens / 1000000).toFixed(2)}M`
                  : '0.00M'}
              </p>
              <span className="text-xs text-white/60">Total tokens</span>
            </div>
          </CardContent>
        );

      case WidgetType.RECENT_CHATS:
        return (
          <CardContent className="p-0">
            <div className="flex justify-between items-center mb-3">
              <CardDescription className="mb-0">
                <span className="text-sm text-white/90">Recent chats</span>
              </CardDescription>
              <Button
                variant="ghost"
                size="sm"
                className="text-xs text-white/60 flex items-center gap-1 !px-0 hover:bg-transparent hover:underline hover:text-white/80"
              >
                See all
              </Button>
            </div>
            <div className="space-y-1 max-h-32 overflow-y-auto">
              {widget.data?.recentSessions?.slice(0, 3).map((session: any, index: number) => (
                <div
                  key={session.id}
                  className="flex items-center justify-between text-xs py-1 px-1 rounded-md hover:bg-white/10 cursor-pointer transition-colors"
                >
                  <div className="flex items-center space-x-2">
                    <ChatSmart className="h-3 w-3 text-white/60" />
                    <span className="truncate max-w-[150px] text-white/80">
                      {session.metadata?.description || session.id}
                    </span>
                  </div>
                  <span className="text-white/50 font-mono font-light">
                    {new Date(session.modified).toLocaleDateString('en-US', { 
                      month: '2-digit', 
                      day: '2-digit' 
                    })}
                  </span>
                </div>
              )) || (
                <div className="text-white/60 text-xs py-2">No recent chats</div>
              )}
            </div>
          </CardContent>
        );

      case WidgetType.GREETING:
        return (
          <CardContent className="flex flex-col justify-center h-full p-0">
            <div className="text-center">
              <h2 className="text-lg font-medium text-white/90 mb-2">
                {widget.data?.greeting || 'Welcome back!'}
              </h2>
              <p className="text-sm text-white/60">
                {widget.data?.subtitle || 'Ready to start a new conversation?'}
              </p>
            </div>
          </CardContent>
        );

      default:
        return (
          <CardContent className="flex items-center justify-center h-full p-0">
            <div className="text-center">
              <p className="text-sm text-white/60">Unknown widget type</p>
            </div>
          </CardContent>
        );
    }
  };

  return (
    <div
      className={`absolute cursor-grab select-none ${
        isDragging ? 'z-50 cursor-grabbing' : 'z-10'
      }`}
      style={{
        left: widget.position.x,
        top: widget.position.y,
        width: widget.size.width,
        height: widget.size.height,
        transform: isDragging ? 'scale(1.05)' : 'scale(1)',
        transition: isDragging ? 'none' : 'transform 0.2s ease-out',
        willChange: isDragging ? 'transform' : 'auto',
      }}
      onMouseDown={onMouseDown}
    >
      {/* Glass effect background */}
      <div
        className={`absolute inset-0 rounded-xl border transition-all duration-200 ${
          isDragging 
            ? 'border-white/40 shadow-3xl' 
            : 'border-white/20 shadow-2xl hover:border-white/30 hover:shadow-3xl'
        }`}
        style={{
          background: `
            linear-gradient(135deg, 
              rgba(255, 255, 255, ${isDragging ? '0.15' : '0.1'}) 0%, 
              rgba(255, 255, 255, ${isDragging ? '0.08' : '0.05'}) 100%
            )
          `,
          backdropFilter: `blur(20px) saturate(${isDragging ? '200%' : '180%'})`,
          WebkitBackdropFilter: `blur(20px) saturate(${isDragging ? '200%' : '180%'})`,
        }}
      />
      
      {/* Content */}
      <div className="relative z-10 p-4 h-full pointer-events-none">
        {renderWidgetContent()}
      </div>
      
      {/* Resize handle */}
      <div className="absolute bottom-0 right-0 w-3 h-3 cursor-se-resize opacity-0 hover:opacity-100 transition-opacity z-20 pointer-events-auto">
        <div className="w-full h-full bg-white/30 rounded-tl-sm" />
      </div>
    </div>
  );
}
