import React, { useState, useEffect } from 'react';
import { Project } from '../../projects';
import { Session, fetchSessions } from '../../sessions';
import { getProject as fetchProject, removeSessionFromProject } from '../../projects';
import { Button } from '../ui/button';
import { ArrowLeft, Plus, Loader, RefreshCcw } from 'lucide-react';
import { toastError, toastSuccess } from '../../toasts';
import AddSessionToProjectModal from './AddSessionToProjectModal';
import SessionItem from '../sessions/SessionItem';

interface ProjectDetailsViewProps {
  projectId: string;
  onBack: () => void;
}

const ProjectDetailsView: React.FC<ProjectDetailsViewProps> = ({ projectId, onBack }) => {
  const [project, setProject] = useState<Project | null>(null);
  const [sessions, setSessions] = useState<Session[]>([]);
  const [allSessions, setAllSessions] = useState<Session[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isAddSessionModalOpen, setIsAddSessionModalOpen] = useState(false);

  // Fetch project details and associated sessions
  useEffect(() => {
    loadProjectData();
  }, [projectId]);

  const loadProjectData = async () => {
    setLoading(true);
    setError(null);

    try {
      // Fetch the project details
      const projectData = await fetchProject(projectId);
      setProject(projectData);

      // Fetch all sessions
      const allSessionsData = await fetchSessions();
      setAllSessions(allSessionsData);

      // Filter sessions that belong to this project
      const projectSessions = allSessionsData.filter((session: Session) =>
        projectData.sessionIds.includes(session.id)
      );

      setSessions(projectSessions);
    } catch (err) {
      console.error('Failed to load project data:', err);
      setError('Failed to load project data');
      toastError({ title: 'Error', msg: 'Failed to load project data' });
    } finally {
      setLoading(false);
    }
  };

  const handleRemoveSession = async (sessionId: string) => {
    if (!project) return;

    try {
      await removeSessionFromProject(project.id, sessionId);

      // Update local state
      setProject((prev) => {
        if (!prev) return null;
        return {
          ...prev,
          sessionIds: prev.sessionIds.filter((id) => id !== sessionId),
        };
      });

      setSessions((prev) => prev.filter((s) => s.id !== sessionId));
      toastSuccess({ title: 'Success', msg: 'Session removed from project' });
    } catch (err) {
      console.error('Failed to remove session from project:', err);
      toastError({ title: 'Error', msg: 'Failed to remove session from project' });
    }
  };

  const getSessionsNotInProject = () => {
    if (!project) return [];

    return allSessions.filter((session) => !project.sessionIds.includes(session.id));
  };

  if (loading) {
    return (
      <div className="flex flex-col h-full w-full items-center justify-center">
        <Loader className="h-10 w-10 animate-spin opacity-70 mb-4" />
        <p className="text-muted-foreground">Loading project...</p>
      </div>
    );
  }

  if (error || !project) {
    return (
      <div className="flex flex-col h-full w-full items-center justify-center">
        <div className="text-center">
          <p className="text-red-500 mb-4">{error || 'Project not found'}</p>
          <div className="flex gap-2">
            <Button onClick={onBack} variant="outline">
              <ArrowLeft className="mr-2 h-4 w-4" /> Back
            </Button>
            <Button onClick={loadProjectData}>
              <RefreshCcw className="mr-2 h-4 w-4" /> Retry
            </Button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full w-full">
      <div className="flex items-center justify-between border-b pb-4">
        <div className="flex items-center gap-2">
          <Button onClick={onBack} variant="ghost" size="sm" className="h-8 w-8 p-0 mr-2">
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div>
            <h1 className="text-2xl font-bold">{project.name}</h1>
            {project.description && <p className="text-muted-foreground">{project.description}</p>}
          </div>
        </div>
        <Button onClick={() => setIsAddSessionModalOpen(true)} className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          <span>Add Session</span>
        </Button>
      </div>

      <div className="flex items-center justify-between py-4">
        <div>
          <p className="text-sm text-muted-foreground">
            Directory:{' '}
            <span className="font-medium text-foreground">{project.defaultDirectory}</span>
          </p>
          <p className="text-sm text-muted-foreground">
            {sessions.length} {sessions.length === 1 ? 'session' : 'sessions'} in this project
          </p>
        </div>
        <Button
          variant="outline"
          size="sm"
          onClick={loadProjectData}
          className="flex items-center gap-2"
        >
          <RefreshCcw className="h-3 w-3" />
          <span>Refresh</span>
        </Button>
      </div>

      {sessions.length === 0 ? (
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center max-w-md">
            <h3 className="text-lg font-medium mb-2">No sessions in this project</h3>
            <p className="text-muted-foreground mb-4">
              Add sessions to this project to keep your work organized
            </p>
            <Button onClick={() => setIsAddSessionModalOpen(true)}>
              <Plus className="h-4 w-4 mr-2" />
              Add Session
            </Button>
          </div>
        </div>
      ) : (
        <div className="flex-1 overflow-y-auto pb-4">
          {sessions.map((session) => (
            <SessionItem
              key={session.id}
              session={session}
              extraActions={
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={(e) => {
                    e.stopPropagation();
                    handleRemoveSession(session.id);
                  }}
                >
                  Remove from project
                </Button>
              }
            />
          ))}
        </div>
      )}

      <AddSessionToProjectModal
        isOpen={isAddSessionModalOpen}
        onClose={() => setIsAddSessionModalOpen(false)}
        project={project}
        availableSessions={getSessionsNotInProject()}
        onSessionsAdded={loadProjectData}
      />
    </div>
  );
};

export default ProjectDetailsView;
