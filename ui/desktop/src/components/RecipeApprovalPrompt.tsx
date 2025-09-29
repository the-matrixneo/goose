import { useState } from 'react';
import { Recipe } from '../recipe';
import { Card } from './ui/card';
import { Button } from './ui/button';
import MarkdownContent from './MarkdownContent';

interface RecipeApprovalPromptProps {
  recipe: Recipe | null | undefined;
  onApprove: () => Promise<void> | void;
  onReject: () => void;
  isSubmitting?: boolean;
}

export default function RecipeApprovalPrompt({ recipe, onApprove, onReject, isSubmitting: externalSubmitting }: RecipeApprovalPromptProps) {
  const [internalSubmitting, setInternalSubmitting] = useState(false);
  const isSubmitting = externalSubmitting ?? internalSubmitting;

  const handleApprove = async () => {
    try {
      if (externalSubmitting === undefined) {
        setInternalSubmitting(true);
      }
      await onApprove();
    } finally {
      if (externalSubmitting === undefined) {
        setInternalSubmitting(false);
      }
    }
  };

  const title = recipe?.title ?? 'Recipe requires approval';
  const description = recipe?.description;
  const instructions = recipe?.instructions;
  const parameters = recipe?.parameters ?? [];

  return (
    <Card className="border border-borderSubtle bg-background-default shadow-sm">
      <div className="p-4 space-y-3">
        <div>
          <h2 className="text-lg font-semibold text-textProminent">{title}</h2>
          {description && <p className="text-sm text-textMuted mt-1">{description}</p>}
        </div>

        {instructions && (
          <div className="bg-background-muted border border-borderSubtle rounded-md p-3">
            <p className="text-xs uppercase tracking-wide text-textSubtle mb-2">Instructions</p>
            <MarkdownContent content={instructions} className="prose prose-sm text-textStandard" />
          </div>
        )}

        {parameters.length > 0 && (
          <div className="bg-background-muted border border-borderSubtle rounded-md p-3">
            <p className="text-xs uppercase tracking-wide text-textSubtle mb-2">Parameters</p>
            <ul className="space-y-2 text-sm text-textStandard">
              {parameters.map((param) => (
                <li key={param.key}>
                  <span className="font-medium">{param.key}</span>
                  {param.description && <span className="text-textMuted"> — {param.description}</span>}
                </li>
              ))}
            </ul>
          </div>
        )}

        <div className="flex justify-end gap-2">
          <Button variant="ghost" onClick={onReject} disabled={isSubmitting}>
            Cancel
          </Button>
          <Button onClick={handleApprove} disabled={isSubmitting}>
            {isSubmitting ? 'Approving…' : 'Approve'}
          </Button>
        </div>
      </div>
    </Card>
  );
}
