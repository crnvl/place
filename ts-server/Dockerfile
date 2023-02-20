# stage1 as builder
FROM node:18-alpine as builder

WORKDIR /usr/src/

# Copy the files and install dependencies
COPY . .
RUN npm install
CMD ["npm", "run", "start"]
# docker run -p 3004:3000 -d clash_backend