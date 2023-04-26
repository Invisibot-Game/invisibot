/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,

  async rewrites() {
    if (process.env.NODE_ENV === "development") {
      return [
        {
          source: "/api/:ep*",
          destination: `http://host.docker.internal:8000/api/:ep*`,
        },
      ];
    } else {
      return [];
    }
  },
};

module.exports = nextConfig;
