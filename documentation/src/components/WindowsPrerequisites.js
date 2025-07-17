const WindowsPrerequisites = () => {
  return (
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
  );
};

export default WindowsPrerequisites;
