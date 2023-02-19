# stage1 as builder
FROM node:18-alpine as builder

WORKDIR /app

# Copy the files and install dependencies
COPY . .
RUN yarn install && yarn build

FROM nginx:alpine as production-build
COPY ./.nginx/nginx.conf /etc/nginx/nginx.conf

## Remove default nginx index page
RUN rm -rf /usr/share/nginx/html/*

# Copy from the stahg 1
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]
# docker run -p 3003:80 -d clash_frontend