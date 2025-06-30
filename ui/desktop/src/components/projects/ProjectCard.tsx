import React, { useState } from 'react';
import { ProjectMetadata } from '../../projects';
import { Card } from '../ui/card';
import { Button } from '../ui/button';
import { Folder, Trash2, Edit, Calendar } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';
import { deleteProject } from '../../projects';
import { toastError, toastSuccess } from '../../toasts';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '../ui/alert-dialog';
import UpdateProjectModal from './UpdateProjectModal';

interface ProjectCardProps {
  project: ProjectMetadata;
  onClick: () => void;
  onRefresh: () => void;
}

const ProjectCard: React.FC<ProjectCardProps> = ({ project, onClick, onRefresh }) => {
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [isUpdateModalOpen, setIsUpdateModalOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const handleDelete = async () => {
    setIsDeleting(true);
    try {
      await deleteProject(project.id);
      toastSuccess({ title: 'Success', msg: `Project "${project.name}" deleted successfully` });
      onRefresh();
    } catch (err) {
      console.error('Failed to delete project:', err);
      toastError({ title: 'Error', msg: 'Failed to delete project' });
    } finally {
      setIsDeleting(false);
      setIsDeleteDialogOpen(false);
    }
  };

  return (
    <>
      <Card
        className="py-2 px-4 mb-2 bg-background-default border-none hover:bg-background-muted cursor-pointer transition-all duration-150"
        onClick={onClick}
      >
        <div className="flex justify-between items-start gap-4">
          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-2 mb-1">
              <h3 className="text-base truncate max-w-[50vw]">{project.name}</h3>
              <Folder className="w-4 h-4 text-text-muted flex-shrink-0" />
            </div>

            {project.description && (
              <p className="text-text-muted text-sm mb-2 line-clamp-2">{project.description}</p>
            )}

            <div className="flex items-center gap-4 text-xs text-text-muted">
              <div className="flex items-center">
                <Calendar className="w-3 h-3 mr-1" />
                {formatDistanceToNow(new Date(project.updatedAt))} ago
              </div>
              <span>
                {project.sessionCount} {project.sessionCount === 1 ? 'session' : 'sessions'}
              </span>
            </div>
          </div>

          <div className="flex items-center gap-2 shrink-0">
            <Button
              onClick={(e) => {
                e.stopPropagation();
                setIsUpdateModalOpen(true);
              }}
              variant="outline"
              size="sm"
              className="h-8"
            >
              <Edit className="w-4 h-4 mr-1" />
              Edit
            </Button>
            <Button
              onClick={(e) => {
                e.stopPropagation();
                setIsDeleteDialogOpen(true);
              }}
              variant="ghost"
              size="sm"
              className="h-8 text-red-500 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20"
            >
              <Trash2 className="w-4 h-4" />
            </Button>
          </div>
        </div>
      </Card>

      <AlertDialog open={isDeleteDialogOpen} onOpenChange={setIsDeleteDialogOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you sure you want to delete this project?</AlertDialogTitle>
            <AlertDialogDescription>
              This will delete the project "{project.name}". The sessions within this project won't
              be deleted, but they will no longer be part of this project.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              className="bg-red-500 hover:bg-red-600"
              onClick={(e) => {
                e.preventDefault();
                handleDelete();
              }}
              disabled={isDeleting}
            >
              {isDeleting ? 'Deleting...' : 'Delete'}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <UpdateProjectModal
        isOpen={isUpdateModalOpen}
        onClose={() => setIsUpdateModalOpen(false)}
        project={project}
        onRefresh={onRefresh}
      />
    </>
  );
};

export default ProjectCard;
