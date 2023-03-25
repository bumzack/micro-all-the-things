# React - Vite Federation Demo

This example demos consumption of federated modules from a vite bundle. `host` (react based) depends on a component exposed by `remote` app (react based).

## Running

First, run `pnpm install`, then `pnpm build` and `pnpm preview`. This will build and serve both `host` and `remote` on ports 7000, 7001 respectively.

- HOST: [localhost:7000](http://localhost:7000/)
- REMOTE: [localhost:7001](http://localhost:7001/)

`CTRL + C` can only stop the host server. You can run `pnpm stop` to stop all services.
