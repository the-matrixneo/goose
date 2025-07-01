import React, { useState, useEffect } from 'react';
import { ProjectMetadata } from '../../projects';
import { fetchProjects, createProject } from '../../projects';
import ProjectCard from './ProjectCard';
import CreateProjectModal from './CreateProjectModal';
import { Button } from '../ui/button';
import { FolderPlus, Loader, AlertCircle, FileText } from 'lucide-react';
import { toastError, toastSuccess } from '../../toasts';
import { ScrollArea } from '../ui/scroll-area';
import { MainPanelLayout } from '../Layout/MainPanelLayout';
import { useSidebar } from '../ui/sidebar';
import { Skeleton } from '../ui/skeleton';

interface ProjectsViewProps {
  onSelectProject: (projectId: string) => void;
  refreshTrigger?: number;
}

const ProjectsView: React.FC<ProjectsViewProps> = ({ onSelectProject, refreshTrigger = 0 }) => {
  const [projects, setProjects] = useState<ProjectMetadata[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [showSkeleton, setShowSkeleton] = useState(true);
  const [showContent, setShowContent] = useState(false);
  const { open: isSidebarOpen } = useSidebar();

  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';

  // Calculate padding based on sidebar state and macOS
  const headerPadding = !isSidebarOpen ? (safeIsMacOS ? 'pl-20' : 'pl-12') : 'pl-4';

  // Load projects on component mount and when refreshTrigger changes
  useEffect(() => {
    loadProjects();
  }, [refreshTrigger]);

  // Minimum loading time to prevent skeleton flash
  useEffect(() => {
    if (!loading && showSkeleton) {
      const timer = setTimeout(() => {
        setShowSkeleton(false);
        // Add a small delay before showing content for fade-in effect
        setTimeout(() => {
          setShowContent(true);
        }, 50);
      }, 300); // Show skeleton for at least 300ms

      return () => clearTimeout(timer);
    }
  }, [loading, showSkeleton]);

  const loadProjects = async () => {
    try {
      setLoading(true);
      setShowSkeleton(true);
      setShowContent(false);
      setError(null);

      const projectsList = await fetchProjects();
      setProjects(projectsList);
    } catch (err) {
      console.error('Failed to load projects:', err);
      setError('Failed to load projects. Please try again.');
      toastError({ title: 'Error', msg: 'Failed to load projects' });
    } finally {
      setLoading(false);
    }
  };

  // Get the current working directory or fallback to home
  const getDefaultDirectory = () => {
    if (window.appConfig && typeof window.appConfig.get === 'function') {
      const dir = window.appConfig.get('GOOSE_WORKING_DIR');
      return typeof dir === 'string' ? dir : '';
    }
    return typeof process !== 'undefined' && process.env && typeof process.env.HOME === 'string'
      ? process.env.HOME
      : '';
  };

  const handleCreateProject = async (
    name: string,
    description: string,
    defaultDirectory?: string
  ) => {
    try {
      const newProject = await createProject({
        name,
        description: description.trim() === '' ? undefined : description,
        default_directory: defaultDirectory || getDefaultDirectory(),
      });
      console.log('Create project response:', newProject);

      setProjects((prevProjects) => [
        ...prevProjects,
        {
          id: newProject.id,
          name: newProject.name,
          description: newProject.description,
          default_directory: newProject.default_directory,
          sessionCount: 0,
          createdAt: newProject.createdAt,
          updatedAt: newProject.updatedAt,
        },
      ]);

      setIsCreateModalOpen(false);
      toastSuccess({ title: 'Success', msg: `Project "${name}" created successfully` });
    } catch (err) {
      console.error('Failed to create project:', err);
      toastError({ title: 'Error', msg: 'Failed to create project' });
    }
  };

  // Render skeleton loader for project items
  const ProjectSkeleton = () => (
    <div className="p-2 mb-2 bg-background-default border border-border-subtle rounded-lg">
      <div className="flex justify-between items-start gap-4">
        <div className="min-w-0 flex-1">
          <Skeleton className="h-5 w-3/4 mb-2" />
          <Skeleton className="h-4 w-full mb-2" />
          <Skeleton className="h-4 w-24" />
        </div>
        <div className="flex items-center gap-2 shrink-0">
          <Skeleton className="h-8 w-20" />
          <Skeleton className="h-8 w-8" />
        </div>
      </div>
    </div>
  );

  const renderContent = () => {
    if (loading || showSkeleton) {
      return (
        <div className="space-y-6">
          <div className="space-y-3">
            <Skeleton className="h-6 w-24" />
            <div className="space-y-2">
              <ProjectSkeleton />
              <ProjectSkeleton />
              <ProjectSkeleton />
            </div>
          </div>
        </div>
      );
    }

    if (error) {
      return (
        <div className="flex flex-col items-center justify-center h-full text-text-muted">
          <AlertCircle className="h-12 w-12 text-red-500 mb-4" />
          <p className="text-lg mb-2">Error Loading Projects</p>
          <p className="text-sm text-center mb-4">{error}</p>
          <Button onClick={loadProjects} variant="default">
            Try Again
          </Button>
        </div>
      );
    }

    if (projects.length === 0) {
      return (
        <div className="flex flex-col justify-center h-full">
          <p className="text-lg">No projects yet</p>
          <p className="text-sm mb-4 text-text-muted">
            Create your first project to organize related sessions together
          </p>
        </div>
      );
    }

    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {projects.map((project) => (
          <ProjectCard
            key={project.id}
            project={project}
            onClick={() => onSelectProject(project.id)}
            onRefresh={loadProjects}
          />
        ))}
      </div>
    );
  };

  return (
    <MainPanelLayout>
      <div className="flex-1 flex flex-col min-h-0">
        {/* Content Area */}
        <div className="flex flex-col mt-13 mb-8 px-2">
          <h1 className="text-4xl font-light">Projects</h1>
          <p className="text-sm text-text-muted mb-1">
            Create and manage your projects to organize related sessions together.
          </p>
          <Button onClick={() => setIsCreateModalOpen(true)} className="self-start mt-4">
            <FolderPlus className="h-4 w-4 mr-2" />
            New project
          </Button>
        </div>

        <div className="flex-1 min-h-0 relative px-2">
          <ScrollArea className="h-full">
            <div
              className={`h-full relative transition-all duration-300 ${
                showContent ? 'opacity-100 animate-in slide-in-from-right-8 ' : 'opacity-0'
              }`}
            >
              {renderContent()}
            </div>
          </ScrollArea>
        </div>
      </div>

      <CreateProjectModal
        isOpen={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
        onCreate={handleCreateProject}
        defaultDirectory={getDefaultDirectory()}
      />
    </MainPanelLayout>
  );
};

export default ProjectsView;
