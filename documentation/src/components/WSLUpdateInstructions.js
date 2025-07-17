const WSLUpdateInstructions = () => {
  return (
    <details>
      <summary>Update via Windows Subsystem for Linux (WSL)</summary>
      <div style={{ marginTop: '1rem' }}>
        <p>To update your WSL installation, use <code>goose update</code> or run the installation script again via WSL:</p>
        <pre><code>curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | CONFIGURE=false bash</code></pre>
      </div>
    </details>
  );
};

export default WSLUpdateInstructions;
