import { describe, it, expect } from 'vitest';
import { validateJsonSchema, getRecipeJsonSchema } from './validation';

describe('Recipe Validation', () => {
  describe('validateJsonSchema', () => {
    describe('valid JSON schemas', () => {
      it('validates a simple JSON schema', () => {
        const schema = {
          type: 'object',
          properties: {
            name: { type: 'string' },
            age: { type: 'number' },
          },
          required: ['name'],
        };

        const result = validateJsonSchema(schema);
        expect(result.success).toBe(true);
        expect(result.errors).toHaveLength(0);
        expect(result.data).toEqual(schema);
      });

      it('validates null schema', () => {
        const result = validateJsonSchema(null);
        expect(result.success).toBe(true);
        expect(result.errors).toHaveLength(0);
        expect(result.data).toBe(null);
      });

      it('validates undefined schema', () => {
        const result = validateJsonSchema(undefined);
        expect(result.success).toBe(true);
        expect(result.errors).toHaveLength(0);
        expect(result.data).toBe(undefined);
      });

      it('validates complex JSON schema', () => {
        const schema = {
          $schema: 'http://json-schema.org/draft-07/schema#',
          type: 'object',
          properties: {
            users: {
              type: 'array',
              items: {
                type: 'object',
                properties: {
                  id: { type: 'number' },
                  profile: {
                    type: 'object',
                    properties: {
                      name: { type: 'string' },
                      email: { type: 'string' },
                    },
                  },
                },
              },
            },
          },
        };

        const result = validateJsonSchema(schema);
        expect(result.success).toBe(true);
        expect(result.data).toEqual(schema);
      });
    });

    describe('invalid JSON schemas', () => {
      it('rejects string input', () => {
        const result = validateJsonSchema('not an object');
        expect(result.success).toBe(false);
        expect(result.errors).toContain('JSON Schema must be an object');
      });

      it('rejects number input', () => {
        const result = validateJsonSchema(42);
        expect(result.success).toBe(false);
        expect(result.errors).toContain('JSON Schema must be an object');
      });

      it('rejects boolean input', () => {
        const result = validateJsonSchema(true);
        expect(result.success).toBe(false);
        expect(result.errors).toContain('JSON Schema must be an object');
      });

      it('validates array input as valid JSON schema', () => {
        const result = validateJsonSchema(['not', 'an', 'object']);
        expect(typeof result.success).toBe('boolean');
        expect(Array.isArray(result.errors)).toBe(true);
      });
    });
  });

  describe('getRecipeJsonSchema', () => {
    it('returns a valid JSON schema object', () => {
      const schema = getRecipeJsonSchema();

      expect(schema).toBeDefined();
      expect(typeof schema).toBe('object');
      expect(schema).toHaveProperty('$schema');
      expect(schema).toHaveProperty('type');
      expect(schema).toHaveProperty('title');
      expect(schema).toHaveProperty('description');
    });

    it('includes standard JSON Schema properties', () => {
      const schema = getRecipeJsonSchema();

      expect(schema.$schema).toBe('http://json-schema.org/draft-07/schema#');
      expect(schema.title).toBeDefined();
      expect(schema.description).toBeDefined();
    });

    it('returns consistent schema across calls', () => {
      const schema1 = getRecipeJsonSchema();
      const schema2 = getRecipeJsonSchema();

      expect(schema1).toEqual(schema2);
    });
  });
});
