import { useTextAnimator } from '../hooks/use-text-animator';
import { Card, CardContent, CardDescription } from './ui/card';
import { useState, useEffect } from 'react';
import { gsap } from 'gsap';

// Register GSAP plugins
gsap.registerPlugin();

interface SplashProps {
  append: (text: string) => void;
  activities: string[] | null;
  title?: string;
}

export default function Splash({ append, activities, title }: SplashProps) {
  const pills = activities || [];

  // Find any pill that starts with "message:"
  const messagePillIndex = pills.findIndex((pill) => pill.toLowerCase().startsWith('message:'));

  // Extract the message pill and the remaining pills
  const messagePill = messagePillIndex >= 0 ? pills[messagePillIndex] : null;
  const remainingPills =
    messagePillIndex >= 0
      ? [...pills.slice(0, messagePillIndex), ...pills.slice(messagePillIndex + 1)]
      : pills;

  const [selectedGreeting, setSelectedGreeting] = useState<{
    prefix: string;
    message: string;
  } | null>(null);

  const [currentTime, setCurrentTime] = useState<string>('');

  // Select a random greeting on component mount
  useEffect(() => {
    const prefixes = ['Hello.', 'Welcome.', 'Greetings.', 'Welcome back.', 'Hello there.'];

    const messages = [
      ' Ready to get started?',
      ' What would you like to work on?',
      ' Ready to build something amazing?',
      ' What would you like to explore?',
      " What's on your mind?",
      ' What shall we create today?',
      ' What project needs attention?',
      ' What would you like to tackle?',
      ' What would you like to explore?',
      ' What needs to be done?',
      " What's the plan for today?",
      ' Ready to create something great?',
      ' What can be built today?',
      " What's the next challenge?",
      ' What progress can be made?',
      ' What would you like to accomplish?',
      ' What task awaits?',
      " What's the mission today?",
      ' What can be achieved?',
      ' What project is ready to begin?',
    ];

    const randomPrefixIndex = Math.floor(Math.random() * prefixes.length);
    const randomMessageIndex = Math.floor(Math.random() * messages.length);

    setSelectedGreeting({
      prefix: prefixes[randomPrefixIndex],
      message: messages[randomMessageIndex],
    });
  }, []);

  // Update time every minute
  useEffect(() => {
    const updateTime = () => {
      const now = new Date();
      const timeString = now.toLocaleTimeString('en-US', {
        hour: 'numeric',
        minute: '2-digit',
        hour12: true,
      });
      setCurrentTime(timeString);
    };

    updateTime(); // Set initial time
    const interval = setInterval(updateTime, 60000); // Update every minute

    return () => clearInterval(interval);
  }, []);

  const getGreeting = () => {
    return selectedGreeting || { prefix: 'Hello.', message: ' How can I help you today?' };
  };

  const greeting = getGreeting();
  const [isContentReady, setIsContentReady] = useState(false);
  const greetingMessageRef = useTextAnimator({ text: greeting.message });

  // Set content ready after initial render
  useEffect(() => {
    setIsContentReady(true);
  }, []);

  return (
    <div className="flex flex-col">
      {title && (
        <div className="flex items-center px-4 py-2 mb-4">
          <span className="w-2 h-2 rounded-full bg-blockTeal mr-2" />
          <span className="text-sm">
            <span className="text-text-muted">Agent</span>{' '}
            <span className="text-text-default">{title}</span>
          </span>
        </div>
      )}

      {/* Compact greeting section with time tile */}
      <div className="flex items-start justify-between px-6 mb-0 gap-4">
        <div className="flex-1">
          <h1 className="text-text-prominent text-4xl font-light mb-2">
            <span>{greeting.prefix}</span>
            {/* <span className="text-text-muted">{greeting.message}</span> */}
          </h1>
        </div>
        
        {/* Time tile matching total sessions card style */}
        <Card className="animate-in fade-in slide-in-from-right-8 duration-500 rounded-2xl min-w-[120px] max-w-[150px]">
          <CardContent className="flex flex-col justify-end items-start h-full pt-4">
            <div className="flex flex-col justify-end items-start">
              <p className="text-2xl font-mono font-light flex items-end">
                {currentTime}
              </p>
              <CardDescription>Current time</CardDescription>
            </div>
          </CardContent>
        </Card>
      </div>

      <div className="flex flex-col">
        {messagePill && (
          <div className="mb-4 p-3 rounded-lg border animate-[fadein_500ms_ease-in_forwards]">
            {messagePill.replace(/^message:/i, '').trim()}
          </div>
        )}

        <div className="flex flex-wrap gap-3 animate-[fadein_500ms_ease-in_forwards]">
          {remainingPills.map((content, index) => (
            <Card
              key={index}
              onClick={() => append(content)}
              title={content.length > 100 ? content : undefined}
              className="cursor-pointer px-4 py-2 w-[200px]"
            >
              {content.length > 100 ? content.slice(0, 100) + '...' : content}
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
}
