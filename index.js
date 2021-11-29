const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'rstoml' means native addon name is `rstoml`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `rstoml.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@rstoml/core-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'rstoml', '@rstoml/core')
