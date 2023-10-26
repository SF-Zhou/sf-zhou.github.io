FROM node:20
WORKDIR /app
COPY . .
RUN npm i && ./build.sh
EXPOSE 80
CMD [ "node", "src/server.js", "80" ]
