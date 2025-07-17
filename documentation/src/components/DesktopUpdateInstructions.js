import MacDesktopInstallButtons from '@site/src/components/MacDesktopInstallButtons';
import WindowsDesktopInstallButtons from '@site/src/components/WindowsDesktopInstallButtons';
import LinuxDesktopInstallButtons from '@site/src/components/LinuxDesktopInstallButtons';

const DesktopUpdateInstructions = ({ os }) => {
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

  const getSteps = () => {
    switch (os) {
      case 'mac':
        return (
          <div style={{ marginTop: '1rem' }}>
            2. Unzip the downloaded zip file.<br/>
            3. Run the executable file to launch the Goose Desktop application.<br/>
            4. Overwrite the existing Goose application with the new version.<br/>
            5. Run the executable file to launch the Goose desktop application.
          </div>
        );
      case 'windows':
        return (
          <div style={{ marginTop: '1rem' }}>
            2. Unzip the downloaded zip file.<br/>
            3. Run the executable file to launch the Goose Desktop application.<br/>
            4. Overwrite the existing Goose application with the new version.<br/>
            5. Run the executable file to launch the Goose Desktop application.
          </div>
        );
      case 'linux':
        return (
          <div style={{ marginTop: '1rem' }}>
            <strong>For Debian/Ubuntu-based distributions:</strong><br/>
            2. Download the DEB file<br/>
            3. Navigate to the directory where it is saved in a terminal<br/>
            4. Run <code>sudo dpkg -i (filename).deb</code><br/>
            5. Launch Goose from the app menu
          </div>
        );
      default:
        return null;
    }
  };

  return (
    <div>
      <div className="admonition admonition-info alert alert--info">
        <div className="admonition-content">
          <p>To update Goose to the latest stable version, reinstall using the instructions below</p>
        </div>
      </div>
      <div style={{ marginTop: '1rem' }}>
        1. {getInstallButtons()}
        {getSteps()}
      </div>
    </div>
  );
};

export default DesktopUpdateInstructions;
