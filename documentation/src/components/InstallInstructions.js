import MacDesktopInstallButtons from '@site/src/components/MacDesktopInstallButtons';
import WindowsDesktopInstallButtons from '@site/src/components/WindowsDesktopInstallButtons';
import LinuxDesktopInstallButtons from '@site/src/components/LinuxDesktopInstallButtons';

const InstallInstructions = ({ 
  type, // 'install' or 'update'
  interface: interfaceType, // 'desktop' or 'cli'
  os, // 'mac', 'windows', 'linux'
  showUpdateTip = true,
  showPrerequisites = true,
  showWSL = true,
  showOptions = true
}) => {
  
  // Desktop Install Buttons
  const getDesktopButtons = () => {
    switch (os) {
      case 'mac': return <MacDesktopInstallButtons />;
      case 'windows': return <WindowsDesktopInstallButtons />;
      case 'linux': return <LinuxDesktopInstallButtons />;
      default: return null;
    }
  };

  // Desktop Install Steps
  const getDesktopSteps = () => {
    if (type === 'update') {
      const steps = os === 'linux' 
        ? ['Download the DEB file', 'Navigate to the directory where it is saved in a terminal', 'Run `sudo dpkg -i (filename).deb`', 'Launch Goose from the app menu']
        : ['Unzip the downloaded zip file', 'Run the executable file to launch the Goose Desktop application', 'Overwrite the existing Goose application with the new version', 'Run the executable file to launch the Goose Desktop application'];
      
      return (
        <div>
          <div className="admonition admonition-info alert alert--info">
            <div className="admonition-content">
              <p>To update Goose to the latest stable version, reinstall using the instructions below</p>
            </div>
          </div>
          <div style={{ marginTop: '1rem' }}>
            1. {getDesktopButtons()}
            <div style={{ marginTop: '1rem' }}>
              {steps.map((step, i) => <div key={i}>{i + 2}. {step}<br/></div>)}
            </div>
          </div>
        </div>
      );
    }

    // Regular install steps
    const steps = os === 'linux'
      ? ['Download the DEB file', 'Navigate to the directory where it is saved in a terminal', 'Run `sudo dpkg -i (filename).deb`', 'Launch Goose from the app menu']
      : ['Unzip the downloaded zip file', 'Run the executable file to launch the Goose Desktop application'];

    return (
      <div>
        <h3 style={{ marginTop: '1rem' }}>Install via Download</h3>
        {getDesktopButtons()}
        <div style={{ marginTop: '1rem' }}>
          {os === 'linux' && <strong>For Debian/Ubuntu-based distributions:</strong>}
          {steps.map((step, i) => <div key={i}>{i + 1}. {step}<br/></div>)}
        </div>
        {showUpdateTip && (
          <div className="admonition admonition-tip alert alert--success">
            <div className="admonition-heading">
              <h5><span className="admonition-icon">💡</span>Updating Goose</h5>
            </div>
            <div className="admonition-content">
              <p>It's best to keep Goose updated by periodically running the installation steps again.</p>
            </div>
          </div>
        )}
      </div>
    );
  };

  // CLI Instructions
  const getCLIInstructions = () => {
    const baseCommand = 'curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash';
    
    if (type === 'update') {
      return (
        <div>
          <p>You can update Goose by running:</p>
          <pre><code>goose update</code></pre>
          
          {showOptions && (
            <div>
              <p>Additional <a href="/docs/guides/goose-cli-commands#update-options">options</a>:</p>
              <pre><code>{`# Update to latest canary (development) version
goose update --canary

# Update and reconfigure settings
goose update --reconfigure`}</code></pre>
            </div>
          )}
          
          <p>Or you can run the <a href="/docs/getting-started/installation">installation</a> script again:</p>
          <pre><code>curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash</code></pre>
          
          <p>To check your current Goose version, use the following command:</p>
          <pre><code>goose --version</code></pre>

          {os === 'windows' && showWSL && (
            <details>
              <summary>Update via Windows Subsystem for Linux (WSL)</summary>
              <div style={{ marginTop: '1rem' }}>
                <p>To update your WSL installation, use <code>goose update</code> or run the installation script again via WSL:</p>
                <pre><code>curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash</code></pre>
              </div>
            </details>
          )}
        </div>
      );
    }

    // Regular install
    const getShellInfo = () => {
      if (os === 'windows') {
        return <p>Run the following command in <strong>Git Bash</strong>, <strong>MSYS2</strong>, or <strong>PowerShell</strong> to install the Goose CLI natively on Windows:</p>;
      }
      if (os === 'mac') {
        return <p>Run the following command to install the latest version of Goose on macOS:</p>;
      }
      return <p>Run the following command to install the Goose CLI on Linux:</p>;
    };

    return (
      <div>
        {os === 'mac' && <h3 style={{ marginTop: '1rem' }}>Option 1: Install via Download script</h3>}
        {getShellInfo()}
        <pre><code>{baseCommand}</code></pre>
        <p>This script will fetch the latest version of Goose and set it up on your system.</p>
        <p>If you'd like to install without interactive configuration, disable <code>CONFIGURE</code>:</p>
        <pre><code>{baseCommand.replace('| bash', '| CONFIGURE=false bash')}</code></pre>

        {showUpdateTip && (
          <div className="admonition admonition-tip alert alert--success">
            <div className="admonition-heading">
              <h5><span className="admonition-icon">💡</span>Updating Goose</h5>
            </div>
            <div className="admonition-content">
              <p>It's best to keep Goose updated. To update Goose, run:</p>
              <pre><code>goose update</code></pre>
            </div>
          </div>
        )}

        {os === 'mac' && (
          <div>
            <h3>Option 2: Install via Homebrew</h3>
            <p>Homebrew downloads the <a href="https://github.com/Homebrew/homebrew-core/blob/master/Formula/b/block-goose-cli.rb">a precompiled CLI tool</a> and can take care of updates.</p>
            <pre><code>brew install block-goose-cli</code></pre>
          </div>
        )}

        {os === 'windows' && showPrerequisites && (
          <div className="admonition admonition-note alert alert--info">
            <div className="admonition-heading">
              <h5><span className="admonition-icon">ℹ️</span>Prerequisites</h5>
            </div>
            <div className="admonition-content">
              <ul>
                <li><strong>Git Bash</strong> (recommended): Comes with <a href="https://git-scm.com/download/win">Git for Windows</a></li>
                <li><strong>MSYS2</strong>: Available from <a href="https://www.msys2.org/">msys2.org</a></li>
                <li><strong>PowerShell</strong>: Available on Windows 10/11 by default</li>
              </ul>
              <p>The script requires <code>curl</code> and <code>unzip</code> to be available in your environment.</p>
            </div>
          </div>
        )}

        {os === 'windows' && showWSL && (
          <details>
            <summary>Install via Windows Subsystem for Linux (WSL)</summary>
            <div style={{ marginTop: '1rem' }}>
              <p>We recommend running the Goose CLI natively on Windows, but you can use WSL if you prefer a Linux-like environment.</p>
              
              <p>1. Open <a href="https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows">PowerShell</a> as Administrator and install WSL and the default Ubuntu distribution:</p>
              <pre><code>wsl --install</code></pre>
              
              <p>2. If prompted, restart your computer to complete the WSL installation. Once restarted, or if WSL is already installed, launch your Ubuntu shell by running:</p>
              <pre><code>wsl -d Ubuntu</code></pre>
              
              <p>3. Run the Goose installation script:</p>
              <pre><code>curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash</code></pre>
              
              <div className="admonition admonition-tip alert alert--success">
                <div className="admonition-content">
                  <p>If you encounter any issues on download, you might need to install <code>bzip2</code> to extract the downloaded file:</p>
                  <pre><code>sudo apt update && sudo apt install bzip2 -y</code></pre>
                </div>
              </div>
              
              <p>If you'd like to install without interactive configuration, disable <code>CONFIGURE</code>:</p>
              <pre><code>curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash</code></pre>
            </div>
          </details>
        )}
      </div>
    );
  };

  return interfaceType === 'desktop' ? getDesktopSteps() : getCLIInstructions();
};

export default InstallInstructions;
