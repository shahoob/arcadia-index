ARG NODE_VERSION=lts

FROM node:${NODE_VERSION}-slim AS build

WORKDIR /home/node/app

COPY . .

RUN --mount=type=cache,target=/root/.npm \
    npm ci --verbose --no-audit
# This should be npm run build
RUN npx vite build

FROM nginx:stable-alpine-slim

ENV ASSETS=/usr/share/nginx/html
ENV NGINX_DEFAULT_SITE_CONFIG=/etc/nginx/conf.d/default.conf

COPY --from=build /home/node/app/dist/ ${ASSETS}
COPY ./docker/nginx.conf ${NGINX_DEFAULT_SITE_CONFIG}

EXPOSE 80
