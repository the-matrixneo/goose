import { useCallback, useState } from 'react';

export interface DroppedFile {
  id: string;
  path: string;
  name: string;
  type: string;
  isImage: boolean;
  dataUrl?: string; // For image previews
  isLoading?: boolean;
  error?: string;
}

export const useFileDrop = () => {
  const [droppedFiles, setDroppedFiles] = useState<DroppedFile[]>([]);

  const handleDrop = useCallback(async (e: React.DragEvent<HTMLDivElement>) => {
    console.log('handleDrop called', e);
    e.preventDefault();
    const files = e.dataTransfer.files;
    console.log('Files dropped:', files.length, files);
    if (files.length > 0) {
      const droppedFileObjects: DroppedFile[] = [];

      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        console.log('Processing file:', file.name, file.type, file.size);

        let droppedFile: DroppedFile;

        try {
          const path = window.electron.getPathForFile(file);
          console.log('File path obtained:', path);
          const isImage = file.type.startsWith('image/');

          droppedFile = {
            id: `dropped-${Date.now()}-${i}`,
            path,
            name: file.name,
            type: file.type,
            isImage,
            isLoading: isImage, // Only images need loading state for preview generation
          };

          console.log('Created DroppedFile object:', droppedFile);
        } catch (error) {
          console.error('Error processing file:', file.name, error);
          // Create an error file object
          droppedFile = {
            id: `dropped-error-${Date.now()}-${i}`,
            path: '',
            name: file.name,
            type: file.type,
            isImage: false,
            isLoading: false,
            error: `Failed to get file path: ${error instanceof Error ? error.message : 'Unknown error'}`,
          };
        }

        droppedFileObjects.push(droppedFile);

        // For images, generate a preview (only if successfully processed)
        if (droppedFile.isImage && !droppedFile.error) {
          console.log('Generating preview for image:', file.name);
          const reader = new FileReader();
          reader.onload = (event) => {
            const dataUrl = event.target?.result as string;
            console.log('Image preview generated for:', file.name);
            setDroppedFiles((prev) =>
              prev.map((f) => (f.id === droppedFile.id ? { ...f, dataUrl, isLoading: false } : f))
            );
          };
          reader.onerror = () => {
            console.error('Failed to generate preview for:', file.name);
            setDroppedFiles((prev) =>
              prev.map((f) =>
                f.id === droppedFile.id
                  ? { ...f, error: 'Failed to load image preview', isLoading: false }
                  : f
              )
            );
          };
          reader.readAsDataURL(file);
        }
      }

      console.log('Adding dropped files to existing list. New files:', droppedFileObjects.length);
      setDroppedFiles((prev) => {
        console.log(
          'Current dropped files count:',
          prev.length,
          'Adding:',
          droppedFileObjects.length
        );
        return [...prev, ...droppedFileObjects];
      });
    }
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent<HTMLDivElement>) => {
    console.log('handleDragOver called in useFileDrop');
    e.preventDefault();
  }, []);

  return {
    droppedFiles,
    setDroppedFiles,
    handleDrop,
    handleDragOver,
  };
};
