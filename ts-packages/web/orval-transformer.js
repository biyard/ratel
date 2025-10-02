/**
 * Helper function to find all `$ref`s under a specific object in the OpenAPI spec
 * and add their names to a Set.
 * @param {object} obj - The object to start searching from.
 * @param {Set<string>} refsSet - The Set where reference names will be stored.
 */
function findSchemaRefs(obj, refsSet) {
  if (!obj || typeof obj !== 'object') {
    return;
  }

  if (Array.isArray(obj)) {
    obj.forEach((item) => findSchemaRefs(item, refsSet));
    return;
  }

  for (const key in obj) {
    if (Object.prototype.hasOwnProperty.call(obj, key)) {
      if (
        key === '$ref' &&
        typeof obj[key] === 'string' &&
        obj[key].startsWith('#/components/schemas/')
      ) {
        // e.g., '#/components/schemas/MySchema' -> 'MySchema'
        const schemaName = obj[key].split('/').pop();
        if (schemaName) {
          refsSet.add(schemaName);
        }
      } else {
        findSchemaRefs(obj[key], refsSet);
      }
    }
  }
}

/**
 * Custom transformer function for Orval.
 * 1. Filters API paths.
 * 2. Prunes the spec to keep only the schemas (and their dependencies) used by the filtered paths.
 * @param {object} openapi - The original OpenAPI specification JSON object.
 * @returns {object} - The modified OpenAPI specification object.
 */
module.exports = (openapi) => {
  console.log('[Orval Transformer] Running transformer with schema pruning...');

  // Step 1: Filter paths and convert Partition parameters to string
  const filteredPaths = Object.keys(openapi.paths)
    .filter((path) => path.startsWith('/v3'))
    .reduce((acc, path) => {
      const pathItem = { ...openapi.paths[path] };

      // For each HTTP method in the path
      Object.keys(pathItem).forEach((method) => {
        if (
          typeof pathItem[method] === 'object' &&
          pathItem[method].parameters
        ) {
          // Convert Partition type parameters in path to string type
          pathItem[method].parameters = pathItem[method].parameters.map(
            (param) => {
              if (
                param.in === 'path' &&
                param.schema?.$ref === '#/components/schemas/Partition'
              ) {
                return {
                  ...param,
                  schema: { type: 'string' },
                };
              }
              return param;
            },
          );
        }
      });

      acc[path] = pathItem;
      return acc;
    }, {});

  // Step 2: Collect all schema references directly used by the filtered paths
  const requiredSchemaNames = new Set();
  findSchemaRefs(filteredPaths, requiredSchemaNames);

  // Step 3: Resolve all nested dependencies of the collected schemas
  const processingQueue = [...requiredSchemaNames]; // A queue of schema names to process
  const allSchemas = openapi.components?.schemas || {};

  while (processingQueue.length > 0) {
    const schemaName = processingQueue.shift(); // Dequeue a schema name
    const schemaDefinition = allSchemas[schemaName];

    if (schemaDefinition) {
      const nestedRefs = new Set();
      findSchemaRefs(schemaDefinition, nestedRefs); // Find references inside this schema's definition

      for (const nestedSchemaName of nestedRefs) {
        // If a new, un-tracked dependency is found, add it to the set and the queue
        if (!requiredSchemaNames.has(nestedSchemaName)) {
          requiredSchemaNames.add(nestedSchemaName);
          processingQueue.push(nestedSchemaName);
        }
      }
    }
  }

  console.log(
    `[Orval Transformer] Found ${requiredSchemaNames.size} required schemas.`,
  );

  // Step 4: Build the new schemas object containing only the required schemas
  const newSchemas = {};
  for (const schemaName of requiredSchemaNames) {
    if (allSchemas[schemaName]) {
      newSchemas[schemaName] = allSchemas[schemaName];
    }
  }

  // Return the final, modified specification
  return {
    ...openapi,
    paths: filteredPaths,
    components: {
      ...openapi.components,
      schemas: newSchemas, // Use the pruned schemas object instead of the original one
    },
  };
};
