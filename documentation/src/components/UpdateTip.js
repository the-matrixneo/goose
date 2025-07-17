const UpdateTip = ({ interface: interfaceType }) => {
  if (interfaceType === 'desktop') {
    return (
      <div className="admonition admonition-tip alert alert--success">
        <div className="admonition-heading">
          <h5><span className="admonition-icon">💡</span>Updating Goose</h5>
        </div>
        <div className="admonition-content">
          <p>It's best to keep Goose updated by periodically running the installation steps again.</p>
        </div>
      </div>
    );
  }

  if (interfaceType === 'cli') {
    return (
      <div className="admonition admonition-tip alert alert--success">
        <div className="admonition-heading">
          <h5><span className="admonition-icon">💡</span>Updating Goose</h5>
        </div>
        <div className="admonition-content">
          <p>It's best to keep Goose updated. To update Goose, run:</p>
          <pre><code>goose update</code></pre>
        </div>
      </div>
    );
  }

  return null;
};

export default UpdateTip;
