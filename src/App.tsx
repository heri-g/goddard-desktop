import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import "./App.css";

function App() {
  const [projectName, setProjectName] = useState("");
  const [projectPath, setProjectPath] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [status, setStatus] = useState("");
  const [error, setError] = useState("");

  async function selectDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Project Directory",
      });
      
      if (selected && typeof selected === "string") {
        setProjectPath(selected);
        setError("");
      }
    } catch (err) {
      setError(`Failed to select directory: ${err}`);
    }
  }

  async function createProject() {
    if (!projectName.trim()) {
      setError("Please enter a project name");
      return;
    }
    
    if (!projectPath.trim()) {
      setError("Please select a project directory");
      return;
    }

    setIsCreating(true);
    setError("");
    setStatus("Creating project...");

    try {
      const result = await invoke("create_shadcn_project", {
        projectName: projectName.trim(),
        projectPath: projectPath.trim(),
      });
      
      setStatus(`Success! Project created at: ${result}`);
      setProjectName("");
    } catch (err) {
      setError(`Failed to create project: ${err}`);
      setStatus("");
    } finally {
      setIsCreating(false);
    }
  }

  return (
    <div className="container">
      <h1 className="subtitle">
        Create beautiful shadcn/ui projects with a single click
      </h1>

      <div className={`form ${isCreating ? 'creating' : ''}`}>
        <div className="input-group">
          <label htmlFor="projectName">Project Name</label>
          <input
            id="projectName"
            type="text"
            placeholder="my-awesome-app"
            value={projectName}
            onChange={(e) => setProjectName(e.target.value)}
            disabled={isCreating}
          />
        </div>

        <div className="input-group">
          <label htmlFor="projectPath">Project Directory</label>
          <div className="path-selector">
            <input
              id="projectPath"
              type="text"
              placeholder="Select a directory..."
              value={projectPath}
              readOnly
              disabled={isCreating}
            />
            <button
              onClick={selectDirectory}
              disabled={isCreating}
              className="browse-btn"
            >
              Browse
            </button>
          </div>
        </div>

        <button
          onClick={createProject}
          disabled={isCreating || !projectName || !projectPath}
          className="create-btn"
        >
          {isCreating ? "Creating..." : "Create Project"}
        </button>

        {status && <div className="status success">{status}</div>}
        {error && <div className="status error">{error}</div>}
      </div>
    </div>
  );
}

export default App;
