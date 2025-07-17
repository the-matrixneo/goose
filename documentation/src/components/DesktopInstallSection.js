import MacDesktopInstallButtons from '@site/src/components/MacDesktopInstallButtons';
import WindowsDesktopInstallButtons from '@site/src/components/WindowsDesktopInstallButtons';
import LinuxDesktopInstallButtons from '@site/src/components/LinuxDesktopInstallButtons';
import DesktopInstallSteps from '@site/src/components/DesktopInstallSteps';
import UpdateTip from '@site/src/components/UpdateTip';

const DesktopInstallSection = ({ os, showUpdateTip = true }) => {
  const getInstallButtons = () => {
    switch (os) {
      case 'mac':
        return <MacDesktopInstallButtons />;
      case 'windows':
        return <WindowsDesktopInstallButtons />;
      case 'linux':
        return <LinuxDesktopInstallButtons />;
      default:
        return null;
    }
  };

  const getTitle = () => {
    switch (os) {
      case 'mac':
        return 'Install via Download';
      case 'windows':
        return 'Install via Download';
      case 'linux':
        return 'Install via Download';
      default:
        return 'Install via Download';
    }
  };

  return (
    <div>
      <h3 style={{ marginTop: '1rem' }}>{getTitle()}</h3>
      {getInstallButtons()}
      <DesktopInstallSteps os={os} />
      {showUpdateTip && <UpdateTip interface="desktop" />}
    </div>
  );
};

export default DesktopInstallSection;
