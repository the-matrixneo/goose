const DesktopInstallSteps = ({ os }) => {
  const getSteps = () => {
    switch (os) {
      case 'mac':
      case 'windows':
        return (
          <div style={{ marginTop: '1rem' }}>
            1. Unzip the downloaded zip file.<br/>
            2. Run the executable file to launch the Goose Desktop application.
          </div>
        );
      case 'linux':
        return (
          <div style={{ marginTop: '1rem' }}>
            <strong>For Debian/Ubuntu-based distributions:</strong><br/>
            1. Download the DEB file<br/>
            2. Navigate to the directory where it is saved in a terminal<br/>
            3. Run <code>sudo dpkg -i (filename).deb</code><br/>
            4. Launch Goose from the app menu
          </div>
        );
      default:
        return null;
    }
  };

  return getSteps();
};

export default DesktopInstallSteps;
