const path = require('path')

const {parsed: env} = require("dotenv").config({ path: path.resolve(__dirname, `../.env`) });

/** @type {import('next').NextConfig} */
const nextConfig = {env}

module.exports = nextConfig
