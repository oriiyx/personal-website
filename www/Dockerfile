# Build stage
FROM node:20-alpine AS deps

WORKDIR /app
COPY package.json yarn.lock ./

# Cache node_modules based on yarn.lock hash
RUN --mount=type=cache,target=/root/.yarn \
    --mount=type=cache,target=/root/.cache/yarn \
    yarn install --frozen-lockfile --network-timeout 1000000

# Build stage
FROM node:20-alpine AS builder
WORKDIR /app

# Copy cached node_modules
COPY --from=deps /app/node_modules ./node_modules
COPY . .

# Cache the build artifacts
RUN --mount=type=cache,target=/app/.cache \
    --mount=type=cache,target=/app/node_modules/.cache \
    yarn build

# Production stage
FROM nginx:alpine
RUN rm -rf /usr/share/nginx/html/* && \
    rm /etc/nginx/conf.d/default.conf

# Copy custom Nginx configuration
COPY .docker/nginx.conf /etc/nginx/nginx.conf

# Copy only the built assets from builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Set permissions
RUN chown -R nginx:nginx /usr/share/nginx/html && \
    chmod -R 755 /usr/share/nginx/html

EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]
