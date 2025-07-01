import React, { useState, useCallback } from 'react';
import ProjectsView from './ProjectsView';
import ProjectDetailsView from './ProjectDetailsView';

const ProjectsContainer: React.FC = () => {
  const [selectedProjectId, setSelectedProjectId] = useState<string | null>(null);
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  const handleSelectProject = (projectId: string) => {
    setSelectedProjectId(projectId);
  };

  const handleBack = () => {
    setSelectedProjectId(null);
    // Trigger a refresh of the projects list when returning from details
    setRefreshTrigger((prev) => prev + 1);
  };

  const triggerRefresh = useCallback(() => {
    setRefreshTrigger((prev) => prev + 1);
  }, []);

  if (selectedProjectId) {
    return <ProjectDetailsView projectId={selectedProjectId} onBack={handleBack} />;
  }

  return <ProjectsView onSelectProject={handleSelectProject} refreshTrigger={refreshTrigger} />;
};

export default ProjectsContainer;
