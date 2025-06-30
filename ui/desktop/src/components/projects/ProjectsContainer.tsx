import React, { useState } from 'react';
import ProjectsView from './ProjectsView';
import ProjectDetailsView from './ProjectDetailsView';

const ProjectsContainer: React.FC = () => {
  const [selectedProjectId, setSelectedProjectId] = useState<string | null>(null);

  const handleSelectProject = (projectId: string) => {
    setSelectedProjectId(projectId);
  };

  const handleBack = () => {
    setSelectedProjectId(null);
  };

  if (selectedProjectId) {
    return <ProjectDetailsView projectId={selectedProjectId} onBack={handleBack} />;
  }

  return <ProjectsView onSelectProject={handleSelectProject} />;
};

export default ProjectsContainer;
