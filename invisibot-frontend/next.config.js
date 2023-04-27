/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,

  async rewrites() {
    if (process.env.NODE_ENV === "development") {
      return [
        {
          source: "/api/:ep*",
          destination: `${process.env.NEXT_PUBLIC_BASE_URL}/:ep*`,
        },
      ];
    } else {
      return [];
    }
  },
};

module.exports = nextConfig;
