import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  output: 'export',
  distDir: 'build',
  assetPrefix: '/next'
};

export default nextConfig;
