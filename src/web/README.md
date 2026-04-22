# Backend

...

---

## SpacetimeDB

### Commands

Generate Rust Client side spacetimedb code:

```bash
spacetime generate --lang rust --out-dir src/module_bindings --module-path spacetimedb/
```

Generate openAPI models with npx and openapi-cli

```bash
npx @openapitools/openapi-generator-cli generate \
  -i vs.swagger.yaml \
  -g rust \
  -o ./generated-api \
  --global-property models \
  --skip-validate-spec 
```