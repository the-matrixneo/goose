const CLIUpdateInstructions = ({ showOptions = true }) => {
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
    </div>
  );
};

export default CLIUpdateInstructions;
