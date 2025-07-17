const CLIInstallCommand = ({ showConfigure = true, os = 'unix' }) => {
  const getCommand = () => {
    if (os === 'windows') {
      return 'curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash';
    }
    return 'curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash';
  };

  const getShellInfo = () => {
    if (os === 'windows') {
      return (
        <div style={{ marginBottom: '1rem' }}>
          Run the following command in <strong>Git Bash</strong>, <strong>MSYS2</strong>, or <strong>PowerShell</strong> to install the Goose CLI natively on Windows:
        </div>
      );
    }
    return (
      <div style={{ marginBottom: '1rem' }}>
        Run the following command to install the latest version of Goose:
      </div>
    );
  };

  return (
    <div>
      {getShellInfo()}
      <pre><code>{getCommand()}</code></pre>
      {showConfigure && (
        <div style={{ marginTop: '1rem' }}>
          <p>This script will fetch the latest version of Goose and set it up on your system.</p>
          <p>If you'd like to install without interactive configuration, disable <code>CONFIGURE</code>:</p>
          <pre><code>{getCommand().replace('| bash', '| CONFIGURE=false bash')}</code></pre>
        </div>
      )}
    </div>
  );
};

export default CLIInstallCommand;
