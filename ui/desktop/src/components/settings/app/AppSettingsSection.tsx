import { useState, useEffect, useRef } from 'react';
import { Switch } from '../../ui/switch';
import { Button } from '../../ui/button';
import { Settings } from 'lucide-react';
import Modal from '../../Modal';
import UpdateSection from './UpdateSection';
import { UPDATES_ENABLED } from '../../../updates';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../ui/card';

interface AppSettingsSectionProps {
  scrollToSection?: string;
}

export default function AppSettingsSection({ scrollToSection }: AppSettingsSectionProps) {
  const [menuBarIconEnabled, setMenuBarIconEnabled] = useState(true);
  const [dockIconEnabled, setDockIconEnabled] = useState(true);
  const [quitConfirmationEnabled, setQuitConfirmationEnabled] = useState(true);
  const [isMacOS, setIsMacOS] = useState(false);
  const [isDockSwitchDisabled, setIsDockSwitchDisabled] = useState(false);
  const [showNotificationModal, setShowNotificationModal] = useState(false);
  const updateSectionRef = useRef<HTMLDivElement>(null);

  // Check if running on macOS
  useEffect(() => {
    setIsMacOS(window.electron.platform === 'darwin');
  }, []);

  // Handle scrolling to update section
  useEffect(() => {
    if (scrollToSection === 'update' && updateSectionRef.current) {
      // Use a timeout to ensure the DOM is ready
      setTimeout(() => {
        updateSectionRef.current?.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }, 100);
    }
  }, [scrollToSection]);

  // Load menu bar and dock icon states
  useEffect(() => {
    window.electron.getMenuBarIconState().then((enabled) => {
      setMenuBarIconEnabled(enabled);
    });

    window.electron.getQuitConfirmationState().then((enabled) => {
      setQuitConfirmationEnabled(enabled);
    });

    if (isMacOS) {
      window.electron.getDockIconState().then((enabled) => {
        setDockIconEnabled(enabled);
      });
    }
  }, [isMacOS]);

  const handleMenuBarIconToggle = async () => {
    const newState = !menuBarIconEnabled;
    // If we're turning off the menu bar icon and the dock icon is hidden,
    // we need to show the dock icon to maintain accessibility
    if (!newState && !dockIconEnabled && isMacOS) {
      const success = await window.electron.setDockIcon(true);
      if (success) {
        setDockIconEnabled(true);
      }
    }
    const success = await window.electron.setMenuBarIcon(newState);
    if (success) {
      setMenuBarIconEnabled(newState);
    }
  };

  const handleDockIconToggle = async () => {
    const newState = !dockIconEnabled;
    // If we're turning off the dock icon and the menu bar icon is hidden,
    // we need to show the menu bar icon to maintain accessibility
    if (!newState && !menuBarIconEnabled) {
      const success = await window.electron.setMenuBarIcon(true);
      if (success) {
        setMenuBarIconEnabled(true);
      }
    }

    // Disable the switch to prevent rapid toggling
    setIsDockSwitchDisabled(true);
    setTimeout(() => {
      setIsDockSwitchDisabled(false);
    }, 1000);

    // Set the dock icon state
    const success = await window.electron.setDockIcon(newState);
    if (success) {
      setDockIconEnabled(newState);
    }
  };

  const handleQuitConfirmationToggle = async () => {
    const newState = !quitConfirmationEnabled;
    const success = await window.electron.setQuitConfirmation(newState);
    if (success) {
      setQuitConfirmationEnabled(newState);
    }
  };

  return (
    <div className="space-y-4 pr-4 pb-8">
      <section className="mb-4">
        <h1 className="text-2xl text-text-default">App</h1>
        <p className="text-sm text-text-muted">Configure goose app</p>
      </section>

      <Card className="rounded-lg">
        <CardHeader className="pb-0">
          <CardTitle className="">Appearance</CardTitle>
          <CardDescription>Configure how goose appears on your system</CardDescription>
        </CardHeader>
        <CardContent className="pt-4 space-y-4 px-4">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-text-default text-xs">Notifications</h3>
              <p className="text-xs text-text-muted max-w-md mt-[2px]">
                Notifications are managed by your OS{' - '}
                <span
                  className="underline hover:cursor-pointer"
                  onClick={() => setShowNotificationModal(true)}
                >
                  Configuration guide
                </span>
              </p>
            </div>
            <div className="flex items-center">
              <Button
                className="flex items-center gap-2 justify-center"
                variant="secondary"
                size="sm"
                onClick={async () => {
                  try {
                    await window.electron.openNotificationsSettings();
                  } catch (error) {
                    console.error('Failed to open notification settings:', error);
                  }
                }}
              >
                <Settings />
                Open Settings
              </Button>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-text-default text-xs">Menu bar icon</h3>
              <p className="text-xs text-text-muted max-w-md mt-[2px]">
                Show goose in the menu bar
              </p>
            </div>
            <div className="flex items-center">
              <Switch
                checked={menuBarIconEnabled}
                onCheckedChange={handleMenuBarIconToggle}
                variant="mono"
              />
            </div>
          </div>

          {isMacOS && (
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-text-default text-xs">Dock icon</h3>
                <p className="text-xs text-text-muted max-w-md mt-[2px]">Show goose in the dock</p>
              </div>
              <div className="flex items-center">
                <Switch
                  disabled={isDockSwitchDisabled}
                  checked={dockIconEnabled}
                  onCheckedChange={handleDockIconToggle}
                  variant="mono"
                />
              </div>
            </div>
          )}

          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-text-default text-xs">Quit confirmation</h3>
              <p className="text-xs text-text-muted max-w-md mt-[2px]">
                Show confirmation dialog when quitting the app
              </p>
            </div>
            <div className="flex items-center">
              <Switch
                checked={quitConfirmationEnabled}
                onCheckedChange={handleQuitConfirmationToggle}
                variant="mono"
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card className="rounded-lg">
        <CardHeader className="pb-0">
          <CardTitle className="mb-1">Help & feedback</CardTitle>
          <CardDescription>
            Help us improve goose by reporting issues or requesting new features
          </CardDescription>
        </CardHeader>
        <CardContent className="pt-4 px-4">
          <div className="flex space-x-4">
            <Button
              onClick={() => {
                window.open(
                  'https://github.com/block/goose/issues/new?template=bug_report.md',
                  '_blank'
                );
              }}
              variant="secondary"
              size="sm"
            >
              Report a Bug
            </Button>
            <Button
              onClick={() => {
                window.open(
                  'https://github.com/block/goose/issues/new?template=feature_request.md',
                  '_blank'
                );
              }}
              variant="secondary"
              size="sm"
            >
              Request a Feature
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Update Section */}
      {UPDATES_ENABLED && (
        <div ref={updateSectionRef}>
          <Card className="rounded-lg">
            <CardHeader className="pb-0">
              <CardTitle className="mb-1">Updates</CardTitle>
              <CardDescription>
                Check for and install updates to keep goose running at its best
              </CardDescription>
            </CardHeader>
            <CardContent className="px-4">
              <UpdateSection />
            </CardContent>
          </Card>
        </div>
      )}

      {/* Notification Instructions Modal */}
      {showNotificationModal && (
        <Modal
          onClose={() => setShowNotificationModal(false)}
          footer={
            <Button
              onClick={() => setShowNotificationModal(false)}
              variant="ghost"
              className="w-full h-[60px] rounded-none hover:bg-bgSubtle text-textSubtle hover:text-textStandard text-md font-regular"
            >
              Close
            </Button>
          }
        >
          {/* Title and Icon */}
          <div className="flex flex-col mb-6">
            <div>
              <Settings className="text-iconStandard" size={24} />
            </div>
            <div className="mt-2">
              <h2 className="text-2xl font-regular text-textStandard">
                How to Enable Notifications
              </h2>
            </div>
          </div>

          {/* OS-specific instructions */}
          {isMacOS ? (
            <div className="space-y-4">
              <p>To enable notifications on macOS:</p>
              <ol className="list-decimal pl-5 space-y-2">
                <li>Open System Preferences</li>
                <li>Click on Notifications</li>
                <li>Find and select goose in the application list</li>
                <li>Enable notifications and adjust settings as desired</li>
              </ol>
            </div>
          ) : (
            <div className="space-y-4">
              <p>To enable notifications on Windows:</p>
              <ol className="list-decimal pl-5 space-y-2">
                <li>Open Settings</li>
                <li>Go to System &gt; Notifications</li>
                <li>Find and select goose in the application list</li>
                <li>Toggle notifications on and adjust settings as desired</li>
              </ol>
            </div>
          )}
        </Modal>
      )}
    </div>
  );
}
