FROM node:25-alpine AS build

WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

COPY . .
RUN yarn build:prod


FROM steebchen/nginx-spa

COPY --from=build /app/dist /app
EXPOSE 80
CMD ["nginx"]
