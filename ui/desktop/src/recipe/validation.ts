import type { Recipe } from '../api/types.gen';

/**
 * OpenAPI-based validation utilities for Recipe objects.
 *
 * This module uses the generated OpenAPI specification directly for validation,
 * ensuring automatic synchronization with backend schema changes.
 * Zod schemas are generated dynamically from the OpenAPI spec.
 */

// Import the OpenAPI spec directly for schema extraction
import openApiSpec from '../../openapi.json';

// Extract the Recipe schema from OpenAPI components
function getRecipeSchema() {
  return openApiSpec.components?.schemas?.Recipe;
}

/**
 * Resolves $ref references in OpenAPI schemas by expanding them with the actual schema definitions
 */
function resolveRefs(
  schema: Record<string, unknown>,
  openApiSpec: Record<string, unknown>
): Record<string, unknown> {
  if (!schema || typeof schema !== 'object') {
    return schema;
  }

  // Handle $ref
  if (typeof schema.$ref === 'string') {
    const refPath = schema.$ref.replace('#/', '').split('/');
    let resolved: unknown = openApiSpec;

    for (const segment of refPath) {
      if (resolved && typeof resolved === 'object' && segment in resolved) {
        resolved = (resolved as Record<string, unknown>)[segment];
      } else {
        console.warn(`Could not resolve $ref: ${schema.$ref}`);
        return schema; // Return original if can't resolve
      }
    }

    if (resolved && typeof resolved === 'object') {
      // Recursively resolve refs in the resolved schema
      return resolveRefs(resolved as Record<string, unknown>, openApiSpec);
    }

    return schema;
  }

  // Handle allOf (merge schemas)
  if (Array.isArray(schema.allOf)) {
    const merged: Record<string, unknown> = {};
    for (const subSchema of schema.allOf) {
      if (typeof subSchema === 'object' && subSchema !== null) {
        const resolved = resolveRefs(subSchema as Record<string, unknown>, openApiSpec);
        Object.assign(merged, resolved);
      }
    }
    // Keep other properties from the original schema
    const { allOf: _allOf, ...rest } = schema;
    return { ...merged, ...rest };
  }

  // Handle oneOf/anyOf (keep as union)
  if (Array.isArray(schema.oneOf)) {
    return {
      ...schema,
      oneOf: schema.oneOf.map((subSchema) =>
        typeof subSchema === 'object' && subSchema !== null
          ? resolveRefs(subSchema as Record<string, unknown>, openApiSpec)
          : subSchema
      ),
    };
  }

  if (Array.isArray(schema.anyOf)) {
    return {
      ...schema,
      anyOf: schema.anyOf.map((subSchema) =>
        typeof subSchema === 'object' && subSchema !== null
          ? resolveRefs(subSchema as Record<string, unknown>, openApiSpec)
          : subSchema
      ),
    };
  }

  // Handle object properties
  if (schema.type === 'object' && schema.properties && typeof schema.properties === 'object') {
    const resolvedProperties: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(schema.properties)) {
      if (typeof value === 'object' && value !== null) {
        resolvedProperties[key] = resolveRefs(value as Record<string, unknown>, openApiSpec);
      } else {
        resolvedProperties[key] = value;
      }
    }
    return {
      ...schema,
      properties: resolvedProperties,
    };
  }

  // Handle array items
  if (schema.type === 'array' && schema.items && typeof schema.items === 'object') {
    return {
      ...schema,
      items: resolveRefs(schema.items as Record<string, unknown>, openApiSpec),
    };
  }

  // Return schema as-is if no refs to resolve
  return schema;
}

export type RecipeValidationResult = {
  success: boolean;
  errors: string[];
  data?: Recipe | unknown;
};

// TODO: Lifei Remove this
/**
 * JSON schema validation for the response.json_schema field.
 * Uses basic structural validation instead of AJV to avoid CSP eval security issues.
 */
export function validateJsonSchema(schema: unknown): RecipeValidationResult {
  try {
    // Allow null/undefined schemas
    if (schema === null || schema === undefined) {
      return { success: true, errors: [], data: schema as unknown };
    }

    if (typeof schema !== 'object') {
      return {
        success: false,
        errors: ['JSON Schema must be an object'],
        data: undefined,
      };
    }

    const schemaObj = schema as Record<string, unknown>;
    const errors: string[] = [];

    // Check for valid JSON Schema structure
    if (schemaObj.type && typeof schemaObj.type !== 'string' && !Array.isArray(schemaObj.type)) {
      errors.push('Invalid type field: must be a string or array');
    }

    // Check for valid properties structure if it exists
    if (schemaObj.properties && typeof schemaObj.properties !== 'object') {
      errors.push('Invalid properties field: must be an object');
    }

    // Check for valid required array if it exists
    if (schemaObj.required && !Array.isArray(schemaObj.required)) {
      errors.push('Invalid required field: must be an array');
    }

    // Check for valid items structure if it exists (for array types)
    if (schemaObj.items && typeof schemaObj.items !== 'object' && !Array.isArray(schemaObj.items)) {
      errors.push('Invalid items field: must be an object or array');
    }

    if (errors.length > 0) {
      return {
        success: false,
        errors: errors.map((err) => `Invalid JSON Schema: ${err}`),
        data: undefined,
      };
    }

    return {
      success: true,
      errors: [],
      data: schema as unknown,
    };
  } catch (error) {
    return {
      success: false,
      errors: [
        `JSON Schema validation error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      ],
      data: undefined,
    };
  }
}

/**
 * Returns a JSON schema representation derived directly from the OpenAPI specification.
 * This schema is used for documentation in form help text.
 *
 * This function extracts the Recipe schema from the OpenAPI spec and converts it
 * to a standard JSON Schema format, ensuring it stays in sync with backend changes.
 *
 * All $ref references are automatically resolved and expanded.
 */
export function getRecipeJsonSchema() {
  const recipeSchema = getRecipeSchema();

  if (!recipeSchema) {
    // Fallback minimal schema if OpenAPI schema is not available
    return {
      $schema: 'http://json-schema.org/draft-07/schema#',
      type: 'object',
      title: 'Recipe',
      description: 'Recipe schema not found in OpenAPI specification',
      required: ['title', 'description'],
      properties: {
        title: { type: 'string' },
        description: { type: 'string' },
      },
    };
  }

  // Resolve all $refs in the schema
  const resolvedSchema = resolveRefs(
    recipeSchema as Record<string, unknown>,
    openApiSpec as Record<string, unknown>
  );

  // Convert OpenAPI schema to JSON Schema format
  return {
    $schema: 'http://json-schema.org/draft-07/schema#',
    ...resolvedSchema,
    title: resolvedSchema.title || 'Recipe',
    description:
      resolvedSchema.description ||
      'A Recipe represents a personalized, user-generated agent configuration that defines specific behaviors and capabilities within the Goose system.',
  };
}
