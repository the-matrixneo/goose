import CLIInstallCommand from '@site/src/components/CLIInstallCommand';
import UpdateTip from '@site/src/components/UpdateTip';
import WindowsPrerequisites from '@site/src/components/WindowsPrerequisites';
import WSLInstallInstructions from '@site/src/components/WSLInstallInstructions';

const CLIInstallSection = ({ os, showUpdateTip = true, showPrerequisites = true, showWSL = true }) => {
  const getOSSpecificContent = () => {
    switch (os) {
      case 'mac':
        return (
          <div>
            <h3 style={{ marginTop: '1rem' }}>Option 1: Install via Download script</h3>
            <p>Run the following command to install the latest version of Goose on macOS:</p>
            <CLIInstallCommand os="unix" />
            {showUpdateTip && <UpdateTip interface="cli" />}
            <h3>Option 2: Install via Homebrew</h3>
            <p>Homebrew downloads the <a href="https://github.com/Homebrew/homebrew-core/blob/master/Formula/b/block-goose-cli.rb">a precompiled CLI tool</a> and can take care of updates.</p>
            <pre><code>brew install block-goose-cli</code></pre>
          </div>
        );
      case 'linux':
        return (
          <div>
            <CLIInstallCommand os="unix" />
            {showUpdateTip && <UpdateTip interface="cli" />}
          </div>
        );
      case 'windows':
        return (
          <div>
            <CLIInstallCommand os="windows" />
            {showPrerequisites && <WindowsPrerequisites />}
            {showWSL && <WSLInstallInstructions />}
          </div>
        );
      default:
        return <CLIInstallCommand />;
    }
  };

  return getOSSpecificContent();
};

export default CLIInstallSection;
