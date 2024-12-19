FROM denoland/deno:2.1.4 AS base
WORKDIR /app


FROM base AS build
COPY .docker.env .env
COPY . .
RUN deno install --allow-scripts=npm:@sveltejs/kit
RUN deno task build


FROM build AS run
WORKDIR /build/
COPY --from=build /app/node_modules/ .
COPY --from=build /app/package.json .
COPY --from=build /app/build/ .
RUN deno install --allow-scripts=npm:@sveltejs/kit
RUN deno cache index.js
USER deno
EXPOSE 3000
ENV DOCKER=true
ENV BACKEND_URL=http://localhost:8000
CMD [ "run", "--allow-env", "--allow-net", "--allow-read", "--no-check", "index.js" ]