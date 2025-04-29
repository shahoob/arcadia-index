ARG NODE_VERSION=lts

FROM node:${NODE_VERSION}-slim AS build

WORKDIR /home/node/app

COPY . .

RUN --mount=type=cache,target=/root/.npm \
    npm ci --verbose --no-audit
# This should be npm run build
RUN npx vite build

FROM nginx:stable-alpine-slim

COPY --from=build /home/node/app/dist/ /usr/share/nginx/html
COPY ./docker/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
