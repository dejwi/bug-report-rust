const path = require('path')

const {parsed: env} = require("dotenv").config({ path: './../.env' });

console.log(env);

/** @type {import('next').NextConfig} */
const nextConfig = {
  env, 
  webpack(config) {
    config.module.rules.push({
      test: /\.svg$/i,
      issuer: /\.[jt]sx?$/,
      use: ['@svgr/webpack'],
    })

    return config
  }
}

module.exports = nextConfig
