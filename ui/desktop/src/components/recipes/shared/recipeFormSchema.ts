import { z } from 'zod';
import { validateJsonSchema } from '../../../recipe/validation';

// Zod schema for Parameter - matching API RecipeParameter type
const parameterSchema = z.object({
  key: z.string().min(1, 'Parameter key is required'),
  input_type: z.enum(['string', 'number', 'boolean', 'date', 'file', 'select']),
  requirement: z.enum(['required', 'optional', 'user_prompt']),
  description: z.string().min(1, 'Parameter description is required'),
  default: z.string().nullable().optional(),
  options: z.array(z.string()).nullable().optional(),
});

// Export the parameter type for use in components
export type RecipeParameter = z.infer<typeof parameterSchema>;

// Main recipe form schema
export const recipeFormSchema = z.object({
  title: z
    .string()
    .min(1, 'Title is required')
    .min(3, 'Title must be at least 3 characters')
    .max(100, 'Title must be 100 characters or less'),

  description: z
    .string()
    .min(1, 'Description is required')
    .min(10, 'Description must be at least 10 characters')
    .max(500, 'Description must be 500 characters or less'),

  instructions: z
    .string()
    .min(1, 'Instructions are required')
    .min(20, 'Instructions must be at least 20 characters'),

  prompt: z.string().optional(),

  activities: z.array(z.string()).default([]),

  parameters: z.array(parameterSchema).default([]),

  jsonSchema: z
    .string()
    .optional()
    .refine((value) => {
      if (!value || !value.trim()) return true;
      try {
        const parsed = JSON.parse(value.trim());
        const validationResult = validateJsonSchema(parsed);
        return validationResult.success;
      } catch {
        return false;
      }
    }, 'Invalid JSON schema format'),

  recipeName: z
    .string()
    .optional()
    .refine((name) => {
      if (!name || !name.trim()) return true;
      return /^[^<>:"/\\|?*]+$/.test(name.trim());
    }, 'Recipe name contains invalid characters (< > : " / \\ | ? *)'),

  global: z.boolean().default(true),
});

export type RecipeFormData = z.infer<typeof recipeFormSchema>;

// Type for the form API - using any to avoid complex generic constraints
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type RecipeFormApi = any;
