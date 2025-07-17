const WSLInstallInstructions = () => {
  return (
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
  );
};

export default WSLInstallInstructions;
